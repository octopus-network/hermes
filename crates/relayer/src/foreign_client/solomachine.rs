use super::ForeignClientError;
use crate::{
    chain::{
        handle::ChainHandle,
        near::CONTRACT_ACCOUNT_ID,
        requests::{
            IncludeProof, QueryChannelRequest, QueryClientStateRequest, QueryConnectionRequest,
            QueryHeight,
        },
        ChainType,
    },
    client_state::AnyClientState,
    connection::ConnectionMsgType,
    consensus_state::AnyConsensusState,
    error::Error as RelayerError,
    keyring::{AnySigningKeyPair, SigningKeyPair},
    light_client::AnyHeader,
};
use ibc_proto::{
    google::protobuf::Any,
    ibc::lightclients::solomachine::v3::{
        ConsensusState as RawSmConsensusState, SignBytes, TimestampedSignatureData,
    },
    protobuf::Protobuf,
};
use ibc_relayer_types::{
    clients::{
        ics06_solomachine::client_state::ClientState as SmClientState,
        ics06_solomachine::consensus_state::{ConsensusState as SmConsensusState, PublicKey},
        ics06_solomachine::header::{Header as SmHeader, HeaderData as SmHeaderData},
    },
    core::{
        ics02_client::{client_state::ClientState, error::Error as ClientError},
        ics03_connection::connection::State,
        ics23_commitment::commitment::{CommitmentProofBytes, CommitmentRoot},
        ics24_host::{
            identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
            path::{ChannelEndsPath, ClientConsensusStatePath, ClientStatePath, ConnectionsPath},
        },
    },
    proofs::{ConsensusProof, Proofs},
    timestamp::Timestamp,
    Height,
};
use prost::Message;
use tracing::{debug, info};

fn get_client_state(
    chain: &impl ChainHandle,
    client_id: &ClientId,
) -> Result<AnyClientState, RelayerError> {
    let (client_state, _) = {
        chain
            .query_client_state(
                QueryClientStateRequest {
                    client_id: client_id.clone(),
                    height: QueryHeight::Latest,
                },
                IncludeProof::No,
            )
            .map_err(|_| {
                RelayerError::query(format!(
                    "clent state for client {} on chain {}",
                    client_id,
                    chain.id()
                ))
            })?
    };
    info!(
        "Client_state of {} on {}: {:?}",
        client_id,
        chain.id(),
        client_state
    );

    if client_state.is_frozen() {
        return Err(RelayerError::light_client_state(
            ibc_relayer_types::core::ics02_client::error::Error::client_frozen(client_id.clone()),
        ));
    }

    Ok(client_state)
}

pub fn query_latest_height_of_chain(
    chain: &impl ChainHandle,
    counterparty_chain: &impl ChainHandle,
    client_id_on_counterparty_chain: &ClientId,
) -> Result<Height, RelayerError> {
    let client_state = get_client_state(counterparty_chain, client_id_on_counterparty_chain)?;
    match client_state {
        AnyClientState::Solomachine(sm_cs) => Ok(sm_cs.latest_height()),
        _ => chain
            .query_latest_height()
            .map_err(|_| RelayerError::query(format!("latest height on chain {}", chain.id()))),
    }
}

fn get_sm_client_pubkey(chain: &impl ChainHandle) -> PublicKey {
    match chain.get_key() {
        Ok(key) => match key {
            AnySigningKeyPair::Secp256k1(key) => PublicKey(
                tendermint::PublicKey::from_raw_secp256k1(&key.public_key.serialize_uncompressed())
                    .unwrap(),
            ),
            AnySigningKeyPair::Ed25519(_) => panic!("Ed25519 keys not supported for Solomachine"),
        },
        Err(_) => panic!("No key found for chain {}", chain.id()),
    }
}

fn get_sm_consensus_state(chain: &impl ChainHandle) -> SmConsensusState {
    let public_key = get_sm_client_pubkey(chain);
    SmConsensusState {
        public_key,
        diversifier: chain.id().to_string(),
        timestamp: Timestamp::now().nanoseconds(),
        root: CommitmentRoot::from_bytes(&public_key.to_bytes()),
    }
}

pub fn build_client_state(
    chain: &impl ChainHandle,
    height: Height,
) -> Result<AnyClientState, ForeignClientError> {
    info!(
        "{}: [build_client_state] - height: {:?}",
        chain.id(),
        height,
    );
    let client_state: AnyClientState = SmClientState {
        sequence: height.revision_height(),
        is_frozen: false,
        consensus_state: get_sm_consensus_state(chain),
    }
    .into();
    let client_state_proto: Any = client_state.clone().into();
    info!(
        "{}: [build_client_state] - client_state: {:?}",
        chain.id(),
        client_state_proto,
    );
    Ok(client_state)
}

pub fn build_consensus_state(
    chain: &impl ChainHandle,
    client_state: &AnyClientState,
) -> Result<AnyConsensusState, ForeignClientError> {
    info!(
        "{}: [build_consensus_state] - client_state: {:?}",
        chain.id(),
        client_state
    );
    match client_state {
        AnyClientState::Solomachine(client_state) => {
            let consensus_state = client_state.consensus_state.clone();
            Ok(consensus_state.into())
        }
        _ => panic!("Unsupported client type for Solomachine"),
    }
}

pub fn sign_bytes(
    chain_id: &ChainId,
    signing_key_pair: &AnySigningKeyPair,
    sequence: u64,
    timestamp: u64,
    path: Vec<u8>,
    data: Vec<u8>,
) -> Vec<u8> {
    use ibc_proto::cosmos::tx::signing::v1beta1::signature_descriptor::{
        data::{Single, Sum},
        Data,
    };

    info!(
        "{}: [sign_bytes_with_solomachine_pubkey] - sequence {:?}, timestamp: {:?}, path: {:?}, data: {:?}",
        chain_id, sequence, timestamp, path, data
    );
    let bytes = SignBytes {
        sequence,
        timestamp,
        diversifier: chain_id.to_string(),
        path,
        data,
    };
    let mut buf = Vec::new();
    Message::encode(&bytes, &mut buf).unwrap();
    debug!(
        "{}: [sign_bytes_with_solomachine_pubkey] - encoded_bytes: {:?}",
        chain_id, buf
    );

    match signing_key_pair {
        AnySigningKeyPair::Secp256k1(key_pair) => {
            let signature = key_pair.sign(&buf).unwrap();
            info!(
                "{}: [sign_bytes_with_solomachine_pubkey] - signature: {:?}, \
                length: {}",
                chain_id,
                signature,
                signature.len()
            );
            let sig = Data {
                sum: Some(Sum::Single(Single { mode: 1, signature })),
            };
            buf = Vec::new();
            Message::encode(&sig, &mut buf).unwrap();

            debug!(
                "{}: [sign_bytes_with_solomachine_pubkey] - sig_data: {:?}",
                chain_id, buf
            );
            buf
        }
        _ => panic!("Unsupported key type for Solomachine"),
    }
}

pub fn build_header(
    chain: &impl ChainHandle,
    trusted_height: Height,
    target_height: Height,
    client_state: &AnyClientState,
) -> Result<(AnyHeader, Vec<AnyHeader>), ForeignClientError> {
    info!(
        "{}: [build_header] - trusted_height: {:?}, target_height: {:?}, client_state: {:?}",
        chain.id(),
        trusted_height,
        target_height,
        client_state
    );

    if trusted_height.revision_height() >= target_height.revision_height() {
        return Err(ForeignClientError::client(ClientError::invalid_height()));
    }
    let cs = if let AnyClientState::Solomachine(cs) = client_state {
        cs
    } else {
        todo!()
    };
    let mut timestamp = cs.consensus_state.timestamp;
    let mut hs: Vec<AnyHeader> = Vec::new();
    let start = if trusted_height.revision_height() > cs.sequence {
        trusted_height.revision_height()
    } else {
        cs.sequence
    };
    let end = if target_height.revision_height() > cs.sequence {
        target_height.revision_height()
    } else {
        cs.sequence + 1
    };

    let signing_key_pair = chain.get_key().unwrap();
    for seq in start..end {
        let pk = get_sm_client_pubkey(chain);
        debug!("{}: [build_header] - pk: {:?}", chain.id(), pk);
        let data = SmHeaderData {
            new_pub_key: Some(pk),
            new_diversifier: chain.id().to_string(),
        };

        timestamp += 1;

        let sig_data = sign_bytes(
            &chain.id(),
            &signing_key_pair,
            seq,
            timestamp,
            "solomachine:header".to_string().as_bytes().to_vec(),
            data.encode_vec(),
        );

        let header = SmHeader {
            timestamp,
            signature: sig_data,
            new_public_key: Some(pk),
            new_diversifier: chain.id().to_string(),
        };

        hs.push(header.into());
    }

    let h = hs.pop().unwrap();
    Ok((h, hs))
}

/// Builds the required proofs and the client state for connection handshake messages.
/// The proofs and client state must be obtained from queries at same height.
pub fn build_connection_proofs_and_client_state(
    chain: &impl ChainHandle,
    counterparty_chain: &impl ChainHandle,
    message_type: ConnectionMsgType,
    connection_id: &ConnectionId,
    client_id: &ClientId,
    counter_party_client_id: &ClientId,
    height: Height,
) -> Result<(Option<AnyClientState>, Proofs), RelayerError> {
    info!(
            "{}: [build_connection_proofs_and_client_state] - message_type: {:?} connection_id: {:?} client_id: {:?} height: {:?}",
            chain.id(), message_type, connection_id, client_id, height
        );

    let sequence = height.revision_height();

    let (connection_end, _maybe_connection_proof) = chain.query_connection(
        QueryConnectionRequest {
            connection_id: connection_id.clone(),
            height: QueryHeight::Latest,
        },
        IncludeProof::No,
    )?;

    debug!("{}: ConnectionStateData: {:?}", chain.id(), connection_end);

    let timestamp_nanos = Timestamp::now().nanoseconds();
    let signing_key_pair = chain.get_key().unwrap();
    let sig_data = match chain.config().unwrap().r#type {
        ChainType::CosmosSdk => {
            let prefix = chain
                .query_commitment_prefix()
                .map_err(|_| RelayerError::query("commitment prefix".to_string()))?;
            let mut path = prefix.into_vec();
            path.extend(
                ConnectionsPath(connection_id.clone())
                    .to_string()
                    .into_bytes(),
            );
            sign_bytes(
                &chain.id(),
                &signing_key_pair,
                sequence + 1,
                timestamp_nanos,
                path,
                connection_end.clone().encode_vec(),
            )
        }
        ChainType::Near => {
            let path = format!(
                "/%09{}%2C/connections%2F{}",
                CONTRACT_ACCOUNT_ID,
                connection_id.as_str()
            )
            .into_bytes();
            sign_bytes(
                &chain.id(),
                &signing_key_pair,
                sequence + 1,
                timestamp_nanos,
                path,
                connection_end.clone().encode_vec(),
            )
        }
    };

    let timestamped = TimestampedSignatureData {
        signature_data: sig_data,
        timestamp: timestamp_nanos,
    };
    debug!(
        "{}: proof_init TimestampedSignatureData: {:?}",
        chain.id(),
        timestamped
    );
    let mut proof_init = Vec::new();
    Message::encode(&timestamped, &mut proof_init).unwrap();

    // Check that the connection state is compatible with the message
    match message_type {
        ConnectionMsgType::OpenTry => {
            if !connection_end.state_matches(&State::Init)
                && !connection_end.state_matches(&State::TryOpen)
            {
                return Err(RelayerError::bad_connection_state());
            }
        }
        ConnectionMsgType::OpenAck => {
            if !connection_end.state_matches(&State::TryOpen)
                && !connection_end.state_matches(&State::Open)
            {
                return Err(RelayerError::bad_connection_state());
            }
        }
        ConnectionMsgType::OpenConfirm => {
            if !connection_end.state_matches(&State::Open) {
                return Err(RelayerError::bad_connection_state());
            }
        }
    }

    let mut client_state = None;
    let mut client_proof = None;
    let mut consensus_proof = None;

    match message_type {
        ConnectionMsgType::OpenTry | ConnectionMsgType::OpenAck => {
            let (client_state_value, _) = counterparty_chain.query_client_state(
                QueryClientStateRequest {
                    client_id: counter_party_client_id.clone(),
                    height: QueryHeight::Latest,
                },
                IncludeProof::No,
            )?;

            debug!("{}: ClientStateData: {:?}", chain.id(), client_state_value);
            let signing_key_pair = chain.get_key().unwrap();
            let sig_data = match chain.config().unwrap().r#type {
                ChainType::CosmosSdk => {
                    let prefix = chain
                        .query_commitment_prefix()
                        .map_err(|_| RelayerError::query("commitment prefix".to_string()))?;
                    let mut path = prefix.into_vec();
                    path.extend(ClientStatePath(client_id.clone()).to_string().into_bytes());
                    sign_bytes(
                        &chain.id(),
                        &signing_key_pair,
                        sequence + 1,
                        timestamp_nanos,
                        path,
                        client_state_value.encode_vec(),
                    )
                }
                ChainType::Near => {
                    let path = format!(
                        "/%09{}%2C/clients%2F{}%2FclientState",
                        CONTRACT_ACCOUNT_ID,
                        client_id.as_str()
                    )
                    .into_bytes();
                    sign_bytes(
                        &chain.id(),
                        &signing_key_pair,
                        sequence + 2,
                        timestamp_nanos,
                        path,
                        client_state_value.encode_vec(),
                    )
                }
            };
            let timestamped = TimestampedSignatureData {
                signature_data: sig_data,
                timestamp: timestamp_nanos,
            };
            debug!(
                "{}: client_proof TimestampedSignatureData: {:?}",
                chain.id(),
                timestamped
            );
            let mut proof_client = Vec::new();
            Message::encode(&timestamped, &mut proof_client).unwrap();

            client_proof = Some(
                CommitmentProofBytes::try_from(proof_client)
                    .map_err(RelayerError::malformed_proof)?,
            );

            let consensus_state: SmConsensusState = match &client_state_value {
                AnyClientState::Solomachine(client_state) => client_state.consensus_state.clone(),
                _ => panic!("Unsupported client type for Solomachine"),
            };
            let sig_data = match chain.config().unwrap().r#type {
                ChainType::CosmosSdk => {
                    let prefix = chain
                        .query_commitment_prefix()
                        .map_err(|_| RelayerError::query("commitment prefix".to_string()))?;
                    let mut path = prefix.into_vec();
                    path.extend(
                        ClientConsensusStatePath {
                            client_id: client_id.clone(),
                            epoch: client_state_value.latest_height().revision_number(),
                            height: client_state_value.latest_height().revision_height(),
                        }
                        .to_string()
                        .into_bytes(),
                    );
                    sign_bytes(
                        &chain.id(),
                        &signing_key_pair,
                        sequence + 1,
                        timestamp_nanos,
                        path,
                        Protobuf::<RawSmConsensusState>::encode_vec(&consensus_state),
                    )
                }
                ChainType::Near => {
                    let path = format!(
                        "/%09{}%2C/clients%2F{}%2FconsensusStates%2F0-{}",
                        CONTRACT_ACCOUNT_ID,
                        client_id.as_str(),
                        client_state_value
                            .latest_height()
                            .revision_height()
                            .to_string()
                    )
                    .into_bytes();
                    sign_bytes(
                        &chain.id(),
                        &signing_key_pair,
                        sequence + 3,
                        timestamp_nanos,
                        path,
                        Protobuf::<RawSmConsensusState>::encode_vec(&consensus_state),
                    )
                }
            };
            let timestamped = TimestampedSignatureData {
                signature_data: sig_data,
                timestamp: timestamp_nanos,
            };
            debug!(
                "{}: consensus_proof TimestampedSignatureData: {:?}",
                chain.id(),
                timestamped
            );
            let mut consensus_state_proof = Vec::new();
            Message::encode(&timestamped, &mut consensus_state_proof).unwrap();

            consensus_proof = Option::from(
                ConsensusProof::new(
                    CommitmentProofBytes::try_from(consensus_state_proof)
                        .map_err(RelayerError::malformed_proof)?,
                    client_state_value.latest_height(),
                )
                .map_err(RelayerError::malformed_proof)?,
            );

            client_state = Some(client_state_value);
        }
        _ => {}
    }

    info!(
            "{}: [build_connection_proofs_and_client_state] - client_state: {:?} proof_init: {:?} client_proof: {:?} consensus_proof: {:?}",
            chain.id(), client_state, proof_init, client_proof, consensus_proof
        );
    Ok((
        client_state,
        Proofs::new(
            CommitmentProofBytes::try_from(proof_init.to_vec())
                .map_err(RelayerError::malformed_proof)?,
            client_proof,
            consensus_proof,
            None,
            height.increment(),
        )
        .map_err(RelayerError::malformed_proof)?,
    ))
}

/// Builds the proof for channel handshake messages.
pub fn build_channel_proofs(
    chain: &impl ChainHandle,
    port_id: &PortId,
    channel_id: &ChannelId,
    height: Height,
) -> Result<Proofs, RelayerError> {
    // Collect all proofs as required
    let (channel_end, _maybe_channel_proof) = chain.query_channel(
        QueryChannelRequest {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            height: QueryHeight::Latest,
        },
        IncludeProof::No,
    )?;

    debug!(
        "PortId: {}, ChannelId: {}, ChannelStateData: {:?}",
        port_id, channel_id, channel_end
    );

    let timestamp_nanos = Timestamp::now().nanoseconds();

    let path = match chain.config().unwrap().r#type {
        ChainType::CosmosSdk => {
            let prefix = chain
                .query_commitment_prefix()
                .map_err(|_| RelayerError::query("commitment prefix".to_string()))?;
            let mut path = prefix.into_vec();
            path.extend(
                ChannelEndsPath(port_id.clone(), channel_id.clone())
                    .to_string()
                    .into_bytes(),
            );
            path
        }
        ChainType::Near => format!(
            "/%09{}%2C/channelEnds%2Fports%2F{}%2Fchannels%2F{}",
            CONTRACT_ACCOUNT_ID, port_id, channel_id
        )
        .into_bytes(),
    };
    let signing_key_pair = chain.get_key().unwrap();
    let sig_data = sign_bytes(
        &chain.id(),
        &signing_key_pair,
        height.revision_height() + 1,
        timestamp_nanos,
        path,
        channel_end.encode_vec(),
    );

    let timestamped = TimestampedSignatureData {
        signature_data: sig_data,
        timestamp: timestamp_nanos,
    };
    let mut channel_proof = Vec::new();
    Message::encode(&timestamped, &mut channel_proof).unwrap();

    let channel_proof_bytes =
        CommitmentProofBytes::try_from(channel_proof).map_err(RelayerError::malformed_proof)?;

    Proofs::new(channel_proof_bytes, None, None, None, height.increment())
        .map_err(RelayerError::malformed_proof)
}
