use super::super::config::{ibc_node, MyConfig};
use anyhow::Result;
use futures::StreamExt;
use ibc_relayer_types::events::IbcEvent;
use subxt::OnlineClient;

/// Subscribe ibc events
/// Maybe in the future call ocw
pub async fn subscribe_ibc_event(client: OnlineClient<MyConfig>) -> Result<Vec<IbcEvent>> {
    println!("In call_ibc: [subscribe_events]");
    const COUNTER: i32 = 3;

    // Subscribe to any events that occur:
    let mut event_sub = client.events().subscribe().await?;

    let mut result_events = Vec::new();
    let mut counter = 0;

    // Our subscription will see the events emitted as a result of this:
    'outer: while let Some(events) = event_sub.next().await {
        let events = events?;
        println!(
            "In substrate: [subscribe_events] >> events length : {:?}",
            events.len()
        );
        let event_length = events.len();
        'inner: for event in events.iter_raw() {
            let event = event?;

            let block_hash = events.block_hash();

            // We can dynamically decode events:
            println!("  Dynamic event details: {block_hash:?}:");

            let raw_event = event.clone();

            let pallet = event.pallet_name();

            let variant = event.variant_name();
            println!("    {pallet}::{variant}");
            match variant.as_str() {
                "CreateClient" => {
                    let event = <ibc_node::ibc::events::CreateClient as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();

                    println!("In call_ibc: [subscribe_events] >> CreateClient Event");

                    let client_id = event.client_id;
                    let client_type = event.client_type;
                    let consensus_height = event.consensus_height;

                    use ibc_relayer_types::core::ics02_client::events::Attributes;
                    result_events.push(IbcEvent::CreateClient(
                        ibc_relayer_types::core::ics02_client::events::CreateClient::from(
                            Attributes {
                                client_id: client_id.into(),
                                client_type: client_type.into(),
                                consensus_height: consensus_height.into(),
                            },
                        ),
                    ));
                    break 'outer;
                }
                "UpdateClient" => {
                    let event = <ibc_node::ibc::events::UpdateClient as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> UpdateClient Event");

                    let client_id = event.client_id;
                    let client_type = event.client_type;
                    let consensus_height = event.consensus_height;

                    use ibc_relayer_types::core::ics02_client::events::Attributes;
                    result_events.push(IbcEvent::UpdateClient(
                        ibc_relayer_types::core::ics02_client::events::UpdateClient::from(
                            Attributes {
                                client_id: client_id.into(),
                                client_type: client_type.into(),
                                consensus_height: consensus_height.into(),
                            },
                        ),
                    ));
                    if event_length > 1 {
                        continue;
                    } else {
                        break 'inner;
                    }
                }
                "ClientMisbehaviour" => {
                    let event =
                        <ibc_node::ibc::events::ClientMisbehaviour as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> ClientMisbehaviour Event");

                    let client_id = event.client_id;
                    let client_type = event.client_type;
                    let consensus_height = event.consensus_height;

                    use ibc_relayer_types::core::ics02_client::events::Attributes;
                    result_events.push(IbcEvent::ClientMisbehaviour(
                        ibc_relayer_types::core::ics02_client::events::ClientMisbehaviour::from(
                            Attributes {
                                client_id: client_id.into(),
                                client_type: client_type.into(),
                                consensus_height: consensus_height.into(),
                            },
                        ),
                    ));
                    break 'outer;
                }
                "OpenInitConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenInitConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenInitConnection Event");

                    let connection_id = event.connection_id.map(|val| val.into());
                    let client_id = event.client_id;
                    let counterparty_connection_id =
                        event.counterparty_connection_id.map(|val| val.into());
                    let counterparty_client_id = event.counterparty_client_id;

                    use ibc_relayer_types::core::ics03_connection::events::Attributes;
                    result_events.push(IbcEvent::OpenInitConnection(
                        ibc_relayer_types::core::ics03_connection::events::OpenInit::from(
                            Attributes {
                                connection_id,
                                client_id: client_id.into(),
                                counterparty_connection_id,
                                counterparty_client_id: counterparty_client_id.into(),
                            },
                        ),
                    ));
                    break 'outer;
                }
                "OpenTryConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenTryConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenTryConnection Event");

                    let connection_id = event.connection_id.map(|val| val.into());
                    let client_id = event.client_id;
                    let counterparty_connection_id =
                        event.counterparty_connection_id.map(|val| val.into());
                    let counterparty_client_id = event.counterparty_client_id;

                    use ibc_relayer_types::core::ics03_connection::events::Attributes;
                    result_events.push(IbcEvent::OpenTryConnection(
                        ibc_relayer_types::core::ics03_connection::events::OpenTry::from(
                            Attributes {
                                connection_id,
                                client_id: client_id.into(),
                                counterparty_connection_id,
                                counterparty_client_id: counterparty_client_id.into(),
                            },
                        ),
                    ));
                    break 'outer;
                }
                "OpenAckConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenAckConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenAckConnection Event");

                    let connection_id = event.connection_id.map(|val| val.into());
                    let client_id = event.client_id;
                    let counterparty_connection_id =
                        event.counterparty_connection_id.map(|val| val.into());
                    let counterparty_client_id = event.counterparty_client_id;

                    use ibc_relayer_types::core::ics03_connection::events::Attributes;
                    result_events.push(IbcEvent::OpenAckConnection(
                        ibc_relayer_types::core::ics03_connection::events::OpenAck::from(
                            Attributes {
                                connection_id,
                                client_id: client_id.into(),
                                counterparty_connection_id,
                                counterparty_client_id: counterparty_client_id.into(),
                            },
                        ),
                    ));
                    break 'outer;
                }
                "OpenConfirmConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenConfirmConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenConfirmConnection Event");

                    let connection_id = event.connection_id.map(|val| val.into());
                    let client_id = event.client_id;
                    let counterparty_connection_id =
                        event.counterparty_connection_id.map(|val| val.into());
                    let counterparty_client_id = event.counterparty_client_id;

                    use ibc_relayer_types::core::ics03_connection::events::Attributes;
                    result_events.push(IbcEvent::OpenConfirmConnection(
                        ibc_relayer_types::core::ics03_connection::events::OpenConfirm::from(
                            Attributes {
                                connection_id,
                                client_id: client_id.into(),
                                counterparty_connection_id,
                                counterparty_client_id: counterparty_client_id.into(),
                            },
                        ),
                    ));
                    break 'outer;
                }
                "OpenInitChannel" => {
                    let event = <ibc_node::ibc::events::OpenInitChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenInitChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    result_events.push(IbcEvent::OpenInitChannel(
                        ibc_relayer_types::core::ics04_channel::events::OpenInit {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    ));
                    break 'outer;
                }
                "OpenTryChannel" => {
                    let event = <ibc_node::ibc::events::OpenTryChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenTryChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    result_events.push(IbcEvent::OpenTryChannel(
                        ibc_relayer_types::core::ics04_channel::events::OpenTry {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    ));
                    break 'outer;
                }
                "OpenAckChannel" => {
                    let event = <ibc_node::ibc::events::OpenAckChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenAckChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    result_events.push(IbcEvent::OpenAckChannel(
                        ibc_relayer_types::core::ics04_channel::events::OpenAck {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    ));
                    break 'outer;
                }
                "OpenConfirmChannel" => {
                    let event =
                        <ibc_node::ibc::events::OpenConfirmChannel as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenConfirmChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    result_events.push(IbcEvent::OpenConfirmChannel(
                        ibc_relayer_types::core::ics04_channel::events::OpenConfirm {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    ));
                    break 'outer;
                }
                "CloseInitChannel" => {
                    let event = <ibc_node::ibc::events::CloseInitChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> CloseInitChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    result_events.push(IbcEvent::CloseInitChannel(
                        ibc_relayer_types::core::ics04_channel::events::CloseInit {
                            port_id: port_id.into(),
                            channel_id: channel_id.unwrap_or_default(),
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    ));
                    break 'outer;
                }
                "CloseConfirmChannel" => {
                    let event =
                        <ibc_node::ibc::events::CloseConfirmChannel as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();

                    println!("In call_ibc: [subscribe_events] >> CloseConfirmChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    result_events.push(IbcEvent::CloseConfirmChannel(
                        ibc_relayer_types::core::ics04_channel::events::CloseConfirm {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    ));
                    break 'outer;
                }
                "SendPacket" => {
                    let event = <ibc_node::ibc::events::SendPacket as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [substrate_events] >> SendPacket Event");

                    let send_packet = ibc_relayer_types::core::ics04_channel::events::SendPacket {
                        packet: event.packet.into(),
                    };

                    result_events.push(IbcEvent::SendPacket(send_packet));
                    break 'outer;
                }
                "ReceivePacket" => {
                    let event = <ibc_node::ibc::events::ReceivePacket as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();

                    println!("In call_ibc: [substrate_events] >> ReceivePacket Event");

                    let receive_packet =
                        ibc_relayer_types::core::ics04_channel::events::ReceivePacket {
                            packet: event.packet.into(),
                        };

                    result_events.push(IbcEvent::ReceivePacket(receive_packet));

                    break 'outer;
                }
                "WriteAcknowledgement" => {
                    let event =
                        <ibc_node::ibc::events::WriteAcknowledgement as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [substrate_events] >> WriteAcknowledgement Event");

                    let write_acknowledgement =
                        ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement {
                            packet: event.packet.into(),
                            ack: event.ack,
                        };

                    result_events.push(IbcEvent::WriteAcknowledgement(write_acknowledgement));

                    break 'outer;
                }
                "AcknowledgePacket" => {
                    let event =
                        <ibc_node::ibc::events::AcknowledgePacket as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [substrate_events] >> AcknowledgePacket Event");

                    let acknowledge_packet =
                        ibc_relayer_types::core::ics04_channel::events::AcknowledgePacket {
                            packet: event.packet.into(),
                        };

                    result_events.push(IbcEvent::AcknowledgePacket(acknowledge_packet));

                    break 'outer;
                }
                "TimeoutPacket" => {
                    let event = <ibc_node::ibc::events::TimeoutPacket as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [substrate_events] >> TimeoutPacket Event");

                    let timeout_packet =
                        ibc_relayer_types::core::ics04_channel::events::TimeoutPacket {
                            packet: event.packet.into(),
                        };

                    result_events.push(IbcEvent::TimeoutPacket(timeout_packet));

                    break 'outer;
                }
                "TimeoutOnClosePacket" => {
                    let event =
                        <ibc_node::ibc::events::TimeoutOnClosePacket as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [substrate_events] >> TimeoutOnClosePacket Event");

                    let timeout_on_close_packet =
                        ibc_relayer_types::core::ics04_channel::events::TimeoutOnClosePacket {
                            packet: event.packet.into(),
                        };

                    result_events.push(IbcEvent::TimeoutOnClosePacket(timeout_on_close_packet));

                    break 'outer;
                }
                "ChainError" => {
                    let event = <ibc_node::ibc::events::ChainError as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("in call_ibc: [substrate_events] >> ChainError Event");

                    let data = String::from_utf8(event.0).unwrap();

                    result_events.push(IbcEvent::ChainError(data));
                    break 'outer;
                }
                "AppModule" => {
                    let event = <ibc_node::ibc::events::AppModule as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [substrate_events] >> AppModule Event");

                    let app_module = ibc_relayer_types::events::ModuleEvent {
                        kind: String::from_utf8(event.0.kind).expect("convert kind error"),
                        module_name: event.0.module_name.into(),
                        attributes: event
                            .0
                            .attributes
                            .into_iter()
                            .map(|attribute| attribute.into())
                            .collect(),
                    };

                    result_events.push(IbcEvent::AppModule(app_module));

                    break 'outer;
                }
                _ => {
                    println!("In call_ibc: [subscribe_events] >> other event");
                    continue;
                }
            }
        }

        if counter == COUNTER {
            break 'outer;
        } else {
            counter += 1;
        }
    }

    Ok(result_events)
}
fn inner_convert_event(raw_events: Vec<RawEventDetails>, module: &str) -> Vec<RawEventDetails> {
    raw_events
        .into_iter()
        .filter(|raw| raw.pallet == module)
        .collect::<Vec<_>>()
}

/// convert substrate event to ibc event
pub fn from_substrate_event_to_ibc_event(raw_events: Vec<RawEventDetails>) -> Vec<IbcEvent> {
    let ret = inner_convert_event(raw_events, "Ibc");

    ret.into_iter()
        .map(|raw_event| {
            let variant = raw_event.variant;
            let ibc_event = match variant.as_str() {
                "CreateClient" => {
                    let event = <ibc_node::ibc::events::CreateClient as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> CreateClient Event");

                    let client_id = event.client_id;
                    let client_type = event.client_type;
                    let consensus_height = event.consensus_height;

                    use ibc_relayer_types::core::ics02_client::events::Attributes;
                    IbcEvent::CreateClient(
                        ibc_relayer_types::core::ics02_client::events::CreateClient::from(
                            Attributes {
                                client_id: client_id.into(),
                                client_type: client_type.into(),
                                consensus_height: consensus_height.into(),
                            },
                        ),
                    )
                }
                "UpdateClient" => {
                    let event = <ibc_node::ibc::events::UpdateClient as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> UpdateClient Event");

                    let client_id = event.client_id;
                    let client_type = event.client_type;
                    let consensus_height = event.consensus_height;

                    use ibc_relayer_types::core::ics02_client::events::Attributes;
                    IbcEvent::UpdateClient(
                        ibc_relayer_types::core::ics02_client::events::UpdateClient::from(
                            Attributes {
                                client_id: client_id.into(),
                                client_type: client_type.into(),
                                consensus_height: consensus_height.into(),
                            },
                        ),
                    )
                }
                "ClientMisbehaviour" => {
                    let event =
                        <ibc_node::ibc::events::ClientMisbehaviour as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> ClientMisbehaviour Event");

                    let client_id = event.client_id;
                    let client_type = event.client_type;
                    let consensus_height = event.consensus_height;

                    use ibc_relayer_types::core::ics02_client::events::Attributes;
                    IbcEvent::ClientMisbehaviour(
                        ibc_relayer_types::core::ics02_client::events::ClientMisbehaviour::from(
                            Attributes {
                                client_id: client_id.into(),
                                client_type: client_type.into(),
                                consensus_height: consensus_height.into(),
                            },
                        ),
                    )
                }
                "OpenInitConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenInitConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenInitConnection Event");

                    let connection_id = event.connection_id.into();
                    let client_id = event.client_id;
                    let counterparty_connection_id =
                        event.counterparty_connection_id.map(|val| val.into());
                    let counterparty_client_id = event.counterparty_client_id;

                    use ibc_relayer_types::core::ics03_connection::events::Attributes;
                    IbcEvent::OpenInitConnection(
                        ibc_relayer_types::core::ics03_connection::events::OpenInit::from(
                            Attributes {
                                connection_id,
                                client_id: client_id.into(),
                                counterparty_connection_id,
                                counterparty_client_id: counterparty_client_id.into(),
                            },
                        ),
                    )
                }
                "OpenTryConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenTryConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenTryConnection Event");

                    let connection_id = event.connection_id.into();
                    let client_id = event.client_id;
                    let counterparty_connection_id =
                        event.counterparty_connection_id.map(|val| val.into());
                    let counterparty_client_id = event.counterparty_client_id;

                    use ibc_relayer_types::core::ics03_connection::events::Attributes;
                    IbcEvent::OpenTryConnection(
                        ibc_relayer_types::core::ics03_connection::events::OpenTry::from(
                            Attributes {
                                connection_id,
                                client_id: client_id.into(),
                                counterparty_connection_id,
                                counterparty_client_id: counterparty_client_id.into(),
                            },
                        ),
                    )
                }
                "OpenAckConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenAckConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenAckConnection Event");

                    let connection_id = event.connection_id.into();
                    let client_id = event.client_id;
                    let counterparty_connection_id =
                        event.counterparty_connection_id.map(|val| val.into());
                    let counterparty_client_id = event.counterparty_client_id;

                    use ibc_relayer_types::core::ics03_connection::events::Attributes;
                    IbcEvent::OpenAckConnection(
                        ibc_relayer_types::core::ics03_connection::events::OpenAck::from(
                            Attributes {
                                connection_id,
                                client_id: client_id.into(),
                                counterparty_connection_id,
                                counterparty_client_id: counterparty_client_id.into(),
                            },
                        ),
                    )
                }
                "OpenConfirmConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenConfirmConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenConfirmConnection Event");

                    let connection_id = event.connection_id.into();
                    let client_id = event.client_id;
                    let counterparty_connection_id =
                        event.counterparty_connection_id.map(|val| val.into());
                    let counterparty_client_id = event.counterparty_client_id;

                    use ibc_relayer_types::core::ics03_connection::events::Attributes;
                    IbcEvent::OpenConfirmConnection(
                        ibc_relayer_types::core::ics03_connection::events::OpenConfirm::from(
                            Attributes {
                                connection_id,
                                client_id: client_id.into(),
                                counterparty_connection_id,
                                counterparty_client_id: counterparty_client_id.into(),
                            },
                        ),
                    )
                }
                "OpenInitChannel" => {
                    let event = <ibc_node::ibc::events::OpenInitChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenInitChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    IbcEvent::OpenInitChannel(
                        ibc_relayer_types::core::ics04_channel::events::OpenInit {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    )
                }
                "OpenTryChannel" => {
                    let event = <ibc_node::ibc::events::OpenTryChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenTryChannel Event");

                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    IbcEvent::OpenTryChannel(
                        ibc_relayer_types::core::ics04_channel::events::OpenTry {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    )
                }
                "OpenAckChannel" => {
                    let event = <ibc_node::ibc::events::OpenAckChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenAckChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    IbcEvent::OpenAckChannel(
                        ibc_relayer_types::core::ics04_channel::events::OpenAck {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    )
                }
                "OpenConfirmChannel" => {
                    let event =
                        <ibc_node::ibc::events::OpenConfirmChannel as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [subscribe_events] >> OpenConfirmChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    IbcEvent::OpenConfirmChannel(
                        ibc_relayer_types::core::ics04_channel::events::OpenConfirm {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    )
                }
                "CloseInitChannel" => {
                    let event = <ibc_node::ibc::events::CloseInitChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [subscribe_events] >> CloseInitChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    IbcEvent::CloseInitChannel(
                        ibc_relayer_types::core::ics04_channel::events::CloseInit {
                            port_id: port_id.into(),
                            channel_id: channel_id.unwrap_or_default(),
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    )
                }
                "CloseConfirmChannel" => {
                    let event =
                        <ibc_node::ibc::events::CloseConfirmChannel as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();

                    println!("In call_ibc: [subscribe_events] >> CloseConfirmChannel Event");

                    let port_id = event.port_id;
                    let channel_id = event.channel_id.map(|val| val.into());
                    let connection_id = event.connection_id;
                    let counterparty_port_id = event.counterparty_port_id;
                    let counterparty_channel_id =
                        event.counterparty_channel_id.map(|val| val.into());

                    IbcEvent::CloseConfirmChannel(
                        ibc_relayer_types::core::ics04_channel::events::CloseConfirm {
                            port_id: port_id.into(),
                            channel_id,
                            connection_id: connection_id.into(),
                            counterparty_port_id: counterparty_port_id.into(),
                            counterparty_channel_id,
                        },
                    )
                }
                "SendPacket" => {
                    let event = <ibc_node::ibc::events::SendPacket as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [substrate_events] >> SendPacket Event");

                    let send_packet = ibc_relayer_types::core::ics04_channel::events::SendPacket {
                        packet: event.packet.into(),
                    };

                    IbcEvent::SendPacket(send_packet)
                }
                "ReceivePacket" => {
                    let event = <ibc_node::ibc::events::ReceivePacket as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [substrate_events] >> ReceivePacket Event");

                    let receive_packet =
                        ibc_relayer_types::core::ics04_channel::events::ReceivePacket {
                            packet: event.packet.into(),
                        };

                    IbcEvent::ReceivePacket(receive_packet)
                }
                "WriteAcknowledgement" => {
                    let event =
                        <ibc_node::ibc::events::WriteAcknowledgement as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [substrate_events] >> WriteAcknowledgement Event");

                    let write_acknowledgement =
                        ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement {
                            packet: event.packet.into(),
                            ack: event.ack,
                        };

                    IbcEvent::WriteAcknowledgement(write_acknowledgement)
                }
                "AcknowledgePacket" => {
                    let event =
                        <ibc_node::ibc::events::AcknowledgePacket as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [substrate_events] >> AcknowledgePacket Event");

                    let acknowledge_packet =
                        ibc_relayer_types::core::ics04_channel::events::AcknowledgePacket {
                            packet: event.packet.into(),
                        };

                    IbcEvent::AcknowledgePacket(acknowledge_packet)
                }
                "TimeoutPacket" => {
                    let event = <ibc_node::ibc::events::TimeoutPacket as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [substrate_events] >> TimeoutPacket Event");

                    let timeout_packet =
                        ibc_relayer_types::core::ics04_channel::events::TimeoutPacket {
                            packet: event.packet.into(),
                        };

                    IbcEvent::TimeoutPacket(timeout_packet)
                }
                "TimeoutOnClosePacket" => {
                    let event =
                        <ibc_node::ibc::events::TimeoutOnClosePacket as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    println!("In call_ibc: [substrate_events] >> TimeoutOnClosePacket Event");

                    let timeout_on_close_packet =
                        ibc_relayer_types::core::ics04_channel::events::TimeoutOnClosePacket {
                            packet: event.packet.into(),
                        };

                    IbcEvent::TimeoutOnClosePacket(timeout_on_close_packet)
                }
                "AppModule" => {
                    let event = <ibc_node::ibc::events::AppModule as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    println!("In call_ibc: [substrate_events] >> AppModule Event");

                    let app_module = ibc_relayer_types::events::ModuleEvent {
                        kind: String::from_utf8(event.0.kind).expect("convert kind error"),
                        module_name: event.0.module_name.into(),
                        attributes: event
                            .0
                            .attributes
                            .into_iter()
                            .map(|attribute| attribute.into())
                            .collect(),
                    };

                    IbcEvent::AppModule(app_module)
                }
                "ChainError" => {
                    let event = <ibc_node::ibc::events::ChainError as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();

                    println!("in call_ibc: [substrate_events] >> ChainError Event");

                    let data = String::from_utf8(event.0).unwrap();

                    IbcEvent::ChainError(data)
                }
                _ => unimplemented!(),
            };
            ibc_event
        })
        .collect::<Vec<_>>()
}
