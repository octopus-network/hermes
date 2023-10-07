use ibc_test_framework::ibc::denom::TaggedDenom;
use ibc_test_framework::prelude::*;
use ibc_test_framework::util::random::random_u128_range;
use std::thread;

#[test]
fn test_ibc_transfer_timeout_cosmos() -> Result<(), Error> {
    run_binary_channel_test(&IbcTransferTimeoutCosmosTest)
}

pub struct IbcTransferTimeoutCosmosTest;

impl TestOverrides for IbcTransferTimeoutCosmosTest {
    fn should_spawn_supervisor(&self) -> bool {
        false
    }
}

impl BinaryChannelTest for IbcTransferTimeoutCosmosTest {
    fn run<ChainA: ChainHandle, ChainB: ChainHandle>(
        &self,
        _config: &TestConfig,
        relayer: RelayerDriver,
        chains: ConnectedChains<ChainA, ChainB>,
        channel: ConnectedChannel<ChainA, ChainB>,
    ) -> Result<(), Error> {
        let token_contract = chains
            .node_a
            .chain_driver()
            .setup_ibc_transfer_for_near(&channel.channel_id_a.0)?;

        let wallet_a = chains.node_a.wallets().user1().cloned();
        let wallet_b = chains.node_b.wallets().user1().cloned();

        let balance_b = chains
            .node_b
            .chain_driver()
            .query_balance(&wallet_b.address(), &chains.node_b.denom())?;

        let b_to_a_amount = random_u128_range(1000, 5000);

        let new_path = format!("{}/{}", "transfer", channel.channel_id_a.as_ref());

        let ibc_denom_a: TaggedDenom<ChainA> = MonoTagged::new(Denom::Ibc {
            path: new_path,
            denom: "samoleans".to_string(),
            hashed: token_contract.to_string(),
        });

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {}/samoleans",
            chains.chain_id_b(),
            chains.chain_id_a(),
            b_to_a_amount,
        );

        chains
            .node_b
            .chain_driver()
            .ibc_transfer_token_with_memo_and_timeout(
                &channel.port_b.as_ref(),
                &channel.channel_id_b.as_ref(),
                &wallet_b.as_ref(),
                &wallet_a.address(),
                &chains.node_b.denom().with_amount(b_to_a_amount).as_ref(),
                None,
                Some(Duration::from_secs(5)),
            )?;

        chains.node_b.chain_driver().assert_eventual_wallet_amount(
            &wallet_b.address(),
            &(balance_b.clone() - b_to_a_amount).as_ref(),
        )?;

        // Sleep to wait for IBC packet to timeout before start relaying
        thread::sleep(Duration::from_secs(6));

        relayer.with_supervisor(|| {
            chains.node_a.chain_driver().assert_eventual_wallet_amount(
                &wallet_a.address(),
                &ibc_denom_a.with_amount(0u128).as_ref(),
            )?;

            chains
                .node_b
                .chain_driver()
                .assert_eventual_wallet_amount(&wallet_b.address(), &balance_b.as_ref())?;

            Ok(())
        })
    }
}
