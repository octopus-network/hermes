use tendermint_vp_sdk::{
    channel_msg::*, client_msg::*, connection_msg::*, packet_msg::recv_packet, packet_msg::*,
    query_ic::*, start_msg::*,
};
mod common;

use common::data::*;
use std::fs;
const PATH: &str = "tests/resource/canister_id";

#[test]
fn test0() {
    let canister_id = fs::read_to_string(PATH).expect("Read file error");
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let ret = start_vp(&canister_id, false).await;
        assert!(ret.is_ok());

        let raw_create_client = get_ibc0_create_client();
        let ret = create_client(&canister_id, false, raw_create_client).await;
        assert!(ret.is_ok());

        let raw_connection_open_init = get_ibc0_connection_open_init();
        let ret = connection_open_init(&canister_id, false, raw_connection_open_init).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc0_update_client1();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc0_update_client2();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_connection_open_ack = get_ibc0_connection_open_ack();
        let ret = connection_open_ack(&canister_id, false, raw_connection_open_ack).await;
        assert!(ret.is_ok());

        let raw_channel_open_init = get_ibc0_channel_open_init();
        let ret = channel_open_init(&canister_id, false, raw_channel_open_init).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc0_update_client3();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_chann_open_ack = get_ibc0_channel_open_ack();
        let ret = channel_open_ack(&canister_id, false, raw_chann_open_ack).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc0_update_client4();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_ack_packet = get_ibc0_ack();
        let ret = ack_packet(&canister_id, false, raw_ack_packet).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc0_update_client5();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_recv_packet = get_ibc0_recv();
        let ret = recv_packet(&canister_id, false, raw_recv_packet).await;
        assert!(ret.is_ok());
    });
}

#[test]
fn test1() {
    let canister_id = fs::read_to_string(PATH).expect("Read file error");
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let ret = start_vp(&canister_id, false).await;
        assert!(ret.is_ok());

        let raw_create_client = get_ibc1_create_client();
        let ret = create_client(&canister_id, false, raw_create_client).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client1();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_connection_open_try = get_ibc1_connection_open_try();
        let ret = connection_open_try(&canister_id, false, raw_connection_open_try).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client2();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_connection_open_confirm = get_ibc1_connection_open_confirm();
        let ret = connection_open_confirm(&canister_id, false, raw_connection_open_confirm).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client3();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_channel_open_try = get_ibc1_channel_open_try();
        let ret = channel_open_try(&canister_id, false, raw_channel_open_try).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client4();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_channel_open_confirm = get_ibc1_channel_open_confirm();
        let ret = channel_open_confirm(&canister_id, false, raw_channel_open_confirm).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client5();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_recv_packet = get_ibc1_recv_packet();
        let ret = recv_packet(&canister_id, false, raw_recv_packet).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client6();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_ack_packet = get_ibc1_ack_packet();
        let ret = ack_packet(&canister_id, false, raw_ack_packet).await;
        assert!(ret.is_ok());

        let ret = query_sequence_times(&canister_id, false, 1).await;
        assert!(ret.is_ok());
    });
}
