use super::super::config::{ibc_node, MyConfig};
use anyhow::Result;
use futures::StreamExt;
use ibc_relayer_types::events::IbcEvent;
use ibc_relayer_types::Height;
use subxt::events::EventDetails;
use subxt::OnlineClient;

/// Subscribe ibc events
pub async fn subscribe_ibc_event(
    client: OnlineClient<MyConfig>,
) -> Result<Vec<IbcEvent>, subxt::error::Error> {
    const COUNTER: i32 = 3;

    // Subscribe to any events that occur:
    let mut event_sub = client.events().subscribe().await?;

    let mut result_events = Vec::new();
    let mut counter = 0;

    // Our subscription will see the events emitted as a result of this:
    'outer: while let Some(events) = event_sub.next().await {
        let events = events?;
        let event_length = events.len();
        'inner: for event in events.iter() {
            let event = event?;
            let pallet = event.pallet_name();
            let variant = event.variant_name();
            println!("    {pallet}::{variant}");

            match variant {
                "CreateClient" => {
                    if let Some(event) = event.as_event::<ibc_node::ibc::events::CreateClient>()? {
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
                }
                "UpdateClient" => {
                    if let Some(event) = event.as_event::<ibc_node::ibc::events::UpdateClient>()? {
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
                }
                "ClientMisbehaviour" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::ClientMisbehaviour>()?
                    {
                        println!("In call_ibc: [subscribe_events] >> ClientMisbehaviour Event");

                        let client_id = event.client_id;
                        let client_type = event.client_type;
                        let consensus_height = Height::new(0, 9).unwrap(); // todo(davirain)

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
                }
                "OpenInitConnection" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::OpenInitConnection>()?
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenInitConnection Event");

                        let connection_id = Some(event.connection_id.into());
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
                }
                "OpenTryConnection" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::OpenTryConnection>()?
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenTryConnection Event");

                        let connection_id = Some(event.connection_id.into());
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
                }
                "OpenAckConnection" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::OpenAckConnection>()?
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenAckConnection Event");

                        let connection_id = Some(event.connection_id.into());
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
                }
                "OpenConfirmConnection" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::OpenConfirmConnection>()?
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenConfirmConnection Event");

                        let connection_id = Some(event.connection_id.into());
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
                }
                "OpenInitChannel" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::OpenInitChannel>()?
                    {
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
                }
                "OpenTryChannel" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::OpenTryChannel>()?
                    {
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
                }
                "OpenAckChannel" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::OpenAckChannel>()?
                    {
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
                }
                "OpenConfirmChannel" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::OpenConfirmChannel>()?
                    {
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
                }
                "CloseInitChannel" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::CloseInitChannel>()?
                    {
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
                }
                "CloseConfirmChannel" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::CloseConfirmChannel>()?
                    {
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
                }
                "SendPacket" => {
                    if let Some(event) = event.as_event::<ibc_node::ibc::events::SendPacket>()? {
                        println!("In call_ibc: [substrate_events] >> SendPacket Event");

                        let send_packet =
                            ibc_relayer_types::core::ics04_channel::events::SendPacket {
                                packet: event.packet.into(),
                            };

                        result_events.push(IbcEvent::SendPacket(send_packet));
                        break 'outer;
                    }
                }
                "ReceivePacket" => {
                    if let Some(event) = event.as_event::<ibc_node::ibc::events::ReceivePacket>()? {
                        println!("In call_ibc: [substrate_events] >> ReceivePacket Event");

                        let receive_packet =
                            ibc_relayer_types::core::ics04_channel::events::ReceivePacket {
                                packet: event.packet.into(),
                            };

                        result_events.push(IbcEvent::ReceivePacket(receive_packet));

                        break 'outer;
                    }
                }
                "WriteAcknowledgement" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::WriteAcknowledgement>()?
                    {
                        println!("In call_ibc: [substrate_events] >> WriteAcknowledgement Event");

                        let write_acknowledgement =
                            ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement {
                                packet: event.packet.into(),
                                ack: event.ack,
                            };

                        result_events.push(IbcEvent::WriteAcknowledgement(write_acknowledgement));

                        break 'outer;
                    }
                }
                "AcknowledgePacket" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::AcknowledgePacket>()?
                    {
                        println!("In call_ibc: [substrate_events] >> AcknowledgePacket Event");

                        let acknowledge_packet =
                            ibc_relayer_types::core::ics04_channel::events::AcknowledgePacket {
                                packet: event.packet.into(),
                            };

                        result_events.push(IbcEvent::AcknowledgePacket(acknowledge_packet));

                        break 'outer;
                    }
                }
                "TimeoutPacket" => {
                    if let Some(event) = event.as_event::<ibc_node::ibc::events::TimeoutPacket>()? {
                        println!("In call_ibc: [substrate_events] >> TimeoutPacket Event");

                        let timeout_packet =
                            ibc_relayer_types::core::ics04_channel::events::TimeoutPacket {
                                packet: event.packet.into(),
                            };

                        result_events.push(IbcEvent::TimeoutPacket(timeout_packet));

                        break 'outer;
                    }
                }
                "TimeoutOnClosePacket" => {
                    if let Some(event) =
                        event.as_event::<ibc_node::ibc::events::TimeoutOnClosePacket>()?
                    {
                        println!("In call_ibc: [substrate_events] >> TimeoutOnClosePacket Event");

                        let timeout_on_close_packet =
                            ibc_relayer_types::core::ics04_channel::events::TimeoutOnClosePacket {
                                packet: event.packet.into(),
                            };

                        result_events.push(IbcEvent::TimeoutOnClosePacket(timeout_on_close_packet));

                        break 'outer;
                    }
                }
                "ChainError" => {
                    if let Some(_event) = event.as_event::<ibc_node::ibc::events::Empty>()? {
                        println!("in call_ibc: [substrate_events] >> ChainError Event");

                        let data = String::from("substrate chain eorrr");

                        result_events.push(IbcEvent::ChainError(data));
                        break 'outer;
                    }
                }
                "AppModule" => {
                    if let Some(event) = event.as_event::<ibc_node::ibc::events::AppModule>()? {
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
fn inner_convert_event(raw_events: Vec<EventDetails>, module: &str) -> Vec<EventDetails> {
    raw_events
        .into_iter()
        .filter(|raw| raw.pallet_name() == module)
        .collect::<Vec<_>>()
}

/// convert substrate event to ibc event
pub fn from_substrate_event_to_ibc_event(raw_events: Vec<EventDetails>) -> Vec<Option<IbcEvent>> {
    let ret = inner_convert_event(raw_events, "Ibc");

    ret.into_iter()
        .map(|raw_event| {
            let variant = raw_event.variant_name();
            let ibc_event = match variant {
                "CreateClient" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::CreateClient>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> CreateClient Event");

                        let client_id = event.client_id;
                        let client_type = event.client_type;
                        let consensus_height = event.consensus_height;

                        use ibc_relayer_types::core::ics02_client::events::Attributes;
                        Some(IbcEvent::CreateClient(
                            ibc_relayer_types::core::ics02_client::events::CreateClient::from(
                                Attributes {
                                    client_id: client_id.into(),
                                    client_type: client_type.into(),
                                    consensus_height: consensus_height.into(),
                                },
                            ),
                        ))
                    } else {
                        None
                    }
                }
                "UpdateClient" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::UpdateClient>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> UpdateClient Event");

                        let client_id = event.client_id;
                        let client_type = event.client_type;
                        let consensus_height = event.consensus_height;

                        use ibc_relayer_types::core::ics02_client::events::Attributes;
                        Some(IbcEvent::UpdateClient(
                            ibc_relayer_types::core::ics02_client::events::UpdateClient::from(
                                Attributes {
                                    client_id: client_id.into(),
                                    client_type: client_type.into(),
                                    consensus_height: consensus_height.into(),
                                },
                            ),
                        ))
                    } else {
                        None
                    }
                }
                "ClientMisbehaviour" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::ClientMisbehaviour>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> ClientMisbehaviour Event");

                        let client_id = event.client_id;
                        let client_type = event.client_type;
                        let consensus_height = Height::new(0, 9).unwrap(); // todo(davirian)

                        use ibc_relayer_types::core::ics02_client::events::Attributes;
                        Some(IbcEvent::ClientMisbehaviour(
                            ibc_relayer_types::core::ics02_client::events::ClientMisbehaviour::from(
                                Attributes {
                                    client_id: client_id.into(),
                                    client_type: client_type.into(),
                                    consensus_height: consensus_height.into(),
                                },
                            ),
                        ))
                    } else {
                        None
                    }
                }
                "OpenInitConnection" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::OpenInitConnection>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenInitConnection Event");

                        let connection_id = Some(event.connection_id.into());
                        let client_id = event.client_id;
                        let counterparty_connection_id =
                            event.counterparty_connection_id.map(|val| val.into());
                        let counterparty_client_id = event.counterparty_client_id;

                        use ibc_relayer_types::core::ics03_connection::events::Attributes;
                        Some(IbcEvent::OpenInitConnection(
                            ibc_relayer_types::core::ics03_connection::events::OpenInit::from(
                                Attributes {
                                    connection_id,
                                    client_id: client_id.into(),
                                    counterparty_connection_id,
                                    counterparty_client_id: counterparty_client_id.into(),
                                },
                            ),
                        ))
                    } else {
                        None
                    }
                }
                "OpenTryConnection" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::OpenTryConnection>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenTryConnection Event");

                        let connection_id = Some(event.connection_id.into());
                        let client_id = event.client_id;
                        let counterparty_connection_id =
                            event.counterparty_connection_id.map(|val| val.into());
                        let counterparty_client_id = event.counterparty_client_id;

                        use ibc_relayer_types::core::ics03_connection::events::Attributes;
                        Some(IbcEvent::OpenTryConnection(
                            ibc_relayer_types::core::ics03_connection::events::OpenTry::from(
                                Attributes {
                                    connection_id,
                                    client_id: client_id.into(),
                                    counterparty_connection_id,
                                    counterparty_client_id: counterparty_client_id.into(),
                                },
                            ),
                        ))
                    } else {
                        None
                    }
                }
                "OpenAckConnection" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::OpenAckConnection>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenAckConnection Event");

                        let connection_id = Some(event.connection_id.into());
                        let client_id = event.client_id;
                        let counterparty_connection_id =
                            event.counterparty_connection_id.map(|val| val.into());
                        let counterparty_client_id = event.counterparty_client_id;

                        use ibc_relayer_types::core::ics03_connection::events::Attributes;
                        Some(IbcEvent::OpenAckConnection(
                            ibc_relayer_types::core::ics03_connection::events::OpenAck::from(
                                Attributes {
                                    connection_id,
                                    client_id: client_id.into(),
                                    counterparty_connection_id,
                                    counterparty_client_id: counterparty_client_id.into(),
                                },
                            ),
                        ))
                    } else {
                        None
                    }
                }
                "OpenConfirmConnection" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::OpenConfirmConnection>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenConfirmConnection Event");

                        let connection_id = Some(event.connection_id.into());
                        let client_id = event.client_id;
                        let counterparty_connection_id =
                            event.counterparty_connection_id.map(|val| val.into());
                        let counterparty_client_id = event.counterparty_client_id;

                        use ibc_relayer_types::core::ics03_connection::events::Attributes;
                        Some(IbcEvent::OpenConfirmConnection(
                            ibc_relayer_types::core::ics03_connection::events::OpenConfirm::from(
                                Attributes {
                                    connection_id,
                                    client_id: client_id.into(),
                                    counterparty_connection_id,
                                    counterparty_client_id: counterparty_client_id.into(),
                                },
                            ),
                        ))
                    } else {
                        None
                    }
                }
                "OpenInitChannel" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::OpenInitChannel>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenInitChannel Event");

                        let port_id = event.port_id;
                        let channel_id = event.channel_id.map(|val| val.into());
                        let connection_id = event.connection_id;
                        let counterparty_port_id = event.counterparty_port_id;
                        let counterparty_channel_id =
                            event.counterparty_channel_id.map(|val| val.into());

                        Some(IbcEvent::OpenInitChannel(
                            ibc_relayer_types::core::ics04_channel::events::OpenInit {
                                port_id: port_id.into(),
                                channel_id,
                                connection_id: connection_id.into(),
                                counterparty_port_id: counterparty_port_id.into(),
                                counterparty_channel_id,
                            },
                        ))
                    } else {
                        None
                    }
                }
                "OpenTryChannel" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::OpenTryChannel>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenTryChannel Event");

                        let port_id = event.port_id;
                        let channel_id = event.channel_id.map(|val| val.into());
                        let connection_id = event.connection_id;
                        let counterparty_port_id = event.counterparty_port_id;
                        let counterparty_channel_id =
                            event.counterparty_channel_id.map(|val| val.into());

                        Some(IbcEvent::OpenTryChannel(
                            ibc_relayer_types::core::ics04_channel::events::OpenTry {
                                port_id: port_id.into(),
                                channel_id,
                                connection_id: connection_id.into(),
                                counterparty_port_id: counterparty_port_id.into(),
                                counterparty_channel_id,
                            },
                        ))
                    } else {
                        None
                    }
                }
                "OpenAckChannel" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::OpenTryChannel>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenAckChannel Event");

                        let port_id = event.port_id;
                        let channel_id = event.channel_id.map(|val| val.into());
                        let connection_id = event.connection_id;
                        let counterparty_port_id = event.counterparty_port_id;
                        let counterparty_channel_id =
                            event.counterparty_channel_id.map(|val| val.into());

                        Some(IbcEvent::OpenAckChannel(
                            ibc_relayer_types::core::ics04_channel::events::OpenAck {
                                port_id: port_id.into(),
                                channel_id,
                                connection_id: connection_id.into(),
                                counterparty_port_id: counterparty_port_id.into(),
                                counterparty_channel_id,
                            },
                        ))
                    } else {
                        None
                    }
                }
                "OpenConfirmChannel" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::OpenConfirmChannel>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> OpenConfirmChannel Event");

                        let port_id = event.port_id;
                        let channel_id = event.channel_id.map(|val| val.into());
                        let connection_id = event.connection_id;
                        let counterparty_port_id = event.counterparty_port_id;
                        let counterparty_channel_id =
                            event.counterparty_channel_id.map(|val| val.into());

                        Some(IbcEvent::OpenConfirmChannel(
                            ibc_relayer_types::core::ics04_channel::events::OpenConfirm {
                                port_id: port_id.into(),
                                channel_id,
                                connection_id: connection_id.into(),
                                counterparty_port_id: counterparty_port_id.into(),
                                counterparty_channel_id,
                            },
                        ))
                    } else {
                        None
                    }
                }
                "CloseInitChannel" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::CloseInitChannel>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> CloseInitChannel Event");

                        let port_id = event.port_id;
                        let channel_id = event.channel_id.map(|val| val.into());
                        let connection_id = event.connection_id;
                        let counterparty_port_id = event.counterparty_port_id;
                        let counterparty_channel_id =
                            event.counterparty_channel_id.map(|val| val.into());

                        Some(IbcEvent::CloseInitChannel(
                            ibc_relayer_types::core::ics04_channel::events::CloseInit {
                                port_id: port_id.into(),
                                channel_id: channel_id.unwrap_or_default(),
                                connection_id: connection_id.into(),
                                counterparty_port_id: counterparty_port_id.into(),
                                counterparty_channel_id,
                            },
                        ))
                    } else {
                        None
                    }
                }
                "CloseConfirmChannel" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::CloseConfirmChannel>()
                        .unwrap()
                    {
                        println!("In call_ibc: [subscribe_events] >> CloseConfirmChannel Event");

                        let port_id = event.port_id;
                        let channel_id = event.channel_id.map(|val| val.into());
                        let connection_id = event.connection_id;
                        let counterparty_port_id = event.counterparty_port_id;
                        let counterparty_channel_id =
                            event.counterparty_channel_id.map(|val| val.into());

                        Some(IbcEvent::CloseConfirmChannel(
                            ibc_relayer_types::core::ics04_channel::events::CloseConfirm {
                                port_id: port_id.into(),
                                channel_id,
                                connection_id: connection_id.into(),
                                counterparty_port_id: counterparty_port_id.into(),
                                counterparty_channel_id,
                            },
                        ))
                    } else {
                        None
                    }
                }
                "SendPacket" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::SendPacket>()
                        .unwrap()
                    {
                        println!("In call_ibc: [substrate_events] >> SendPacket Event");

                        let send_packet =
                            ibc_relayer_types::core::ics04_channel::events::SendPacket {
                                packet: event.packet.into(),
                            };

                        Some(IbcEvent::SendPacket(send_packet))
                    } else {
                        None
                    }
                }
                "ReceivePacket" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::ReceivePacket>()
                        .unwrap()
                    {
                        println!("In call_ibc: [substrate_events] >> ReceivePacket Event");

                        let receive_packet =
                            ibc_relayer_types::core::ics04_channel::events::ReceivePacket {
                                packet: event.packet.into(),
                            };

                        Some(IbcEvent::ReceivePacket(receive_packet))
                    } else {
                        None
                    }
                }
                "WriteAcknowledgement" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::WriteAcknowledgement>()
                        .unwrap()
                    {
                        println!("In call_ibc: [substrate_events] >> WriteAcknowledgement Event");

                        let write_acknowledgement =
                            ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement {
                                packet: event.packet.into(),
                                ack: event.ack,
                            };

                        Some(IbcEvent::WriteAcknowledgement(write_acknowledgement))
                    } else {
                        None
                    }
                }
                "AcknowledgePacket" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::AcknowledgePacket>()
                        .unwrap()
                    {
                        println!("In call_ibc: [substrate_events] >> AcknowledgePacket Event");

                        let acknowledge_packet =
                            ibc_relayer_types::core::ics04_channel::events::AcknowledgePacket {
                                packet: event.packet.into(),
                            };

                        Some(IbcEvent::AcknowledgePacket(acknowledge_packet))
                    } else {
                        None
                    }
                }
                "TimeoutPacket" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::TimeoutPacket>()
                        .unwrap()
                    {
                        println!("In call_ibc: [substrate_events] >> TimeoutPacket Event");

                        let timeout_packet =
                            ibc_relayer_types::core::ics04_channel::events::TimeoutPacket {
                                packet: event.packet.into(),
                            };

                        Some(IbcEvent::TimeoutPacket(timeout_packet))
                    } else {
                        None
                    }
                }
                "TimeoutOnClosePacket" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::TimeoutOnClosePacket>()
                        .unwrap()
                    {
                        println!("In call_ibc: [substrate_events] >> TimeoutOnClosePacket Event");

                        let timeout_on_close_packet =
                            ibc_relayer_types::core::ics04_channel::events::TimeoutOnClosePacket {
                                packet: event.packet.into(),
                            };

                        Some(IbcEvent::TimeoutOnClosePacket(timeout_on_close_packet))
                    } else {
                        None
                    }
                }
                "AppModule" => {
                    if let Some(event) = raw_event
                        .as_event::<ibc_node::ibc::events::AppModule>()
                        .unwrap()
                    {
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

                        Some(IbcEvent::AppModule(app_module))
                    } else {
                        None
                    }
                }
                "ChainError" => {
                    println!("in call_ibc: [substrate_events] >> ChainError Event");

                    let data = String::from("chain error substrate");

                    Some(IbcEvent::ChainError(data))
                }
                _ => unimplemented!(),
            };
            ibc_event
        })
        .collect::<Vec<_>>()
}
