use ibc_test_framework::ibc::denom::TaggedDenom;
use ibc_test_framework::prelude::*;
use ibc_test_framework::util::random::random_u128_range;

// report this
// Expected task to eventually succeeed, but failed after 90 attempts:
// wallet reach cosmos1pznpav7hle84cq7443fgetdxnuvt9nwnhqxfkk amount 3000000000000000000ibc/93B4B75C6D876BD9168CB4FA8B78D3D9C916FD3100EAF8A6AD3B3093661E8B9E
#[test]
fn test_ibc_transfers() -> Result<(), Error> {
    run_binary_channel_test(&IbcTransferTest)
}

/**
   Test that IBC token transfer can still work with a single
   chain that is connected to itself.
*/
#[test]
fn test_self_connected_ibc_transfer() -> Result<(), Error> {
    run_self_connected_binary_chain_test(&RunBinaryConnectionTest::new(&RunBinaryChannelTest::new(
        &RunWithSupervisor::new(&IbcTransferTest),
    )))
}

/**
   Run the IBC transfer test as an N-ary chain test case with SIZE=2.

   The work on N-ary chain is currently still work in progress, so we put
   this behind the "experimental" feature flag so that normal developers
   are not obligated to understand how this test works yet.
*/
#[test]
fn test_nary_ibc_transfer() -> Result<(), Error> {
    run_binary_as_nary_channel_test(&IbcTransferTest)
}

#[test]
fn test_self_connected_nary_ibc_transfer() -> Result<(), Error> {
    run_self_connected_nary_chain_test(&RunNaryConnectionTest::new(&RunNaryChannelTest::new(
        &RunBinaryAsNaryChannelTest::new(&IbcTransferTest),
    )))
}

pub struct IbcTransferTest;

impl TestOverrides for IbcTransferTest {}

impl BinaryChannelTest for IbcTransferTest {
    fn run<ChainA: ChainHandle, ChainB: ChainHandle>(
        &self,
        _config: &TestConfig,
        _relayer: RelayerDriver,
        chains: ConnectedChains<ChainA, ChainB>,
        channel: ConnectedChannel<ChainA, ChainB>,
    ) -> Result<(), Error> {
        let token_contract = chains
            .node_a
            .chain_driver()
            .setup_ibc_transfer_for_near(&channel.channel_id_a.0)?;

        let denom_a = chains.node_a.denom();

        let wallet_a = chains.node_a.wallets().user1().cloned();
        let wallet_b = chains.node_b.wallets().user1().cloned();
        let wallet_c = chains.node_a.wallets().user2().cloned();
        let wallet_d = chains.node_b.wallets().user2().cloned();

        let balance_a = chains
            .node_a
            .chain_driver()
            .query_balance(&wallet_a.address(), &denom_a)?;

        let a_to_b_amount = random_u128_range(1, 10);

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
            chains.chain_id_a(),
            chains.chain_id_b(),
            a_to_b_amount,
            denom_a
        );

        chains.node_a.chain_driver().ibc_transfer_token(
            &channel.port_a.as_ref(),
            &channel.channel_id_a.as_ref(),
            &wallet_a.as_ref(),
            &wallet_b.address(),
            &denom_a.with_amount(a_to_b_amount).as_ref(),
        )?;

        let denom_b = derive_ibc_denom(
            &channel.port_b.as_ref(),
            &channel.channel_id_b.as_ref(),
            &denom_a,
        )?;

        info!(
            "Waiting for user on chain B to receive IBC transferred amount of {}",
            a_to_b_amount
        );

        chains.node_a.chain_driver().assert_eventual_wallet_amount(
            &wallet_a.address(),
            &(balance_a - a_to_b_amount * 10u128.pow(18)).as_ref(),
        )?;

        chains.node_b.chain_driver().assert_eventual_wallet_amount(
            &wallet_b.address(),
            &denom_b.with_amount(a_to_b_amount * 10u128.pow(18)).as_ref(),
        )?;

        info!(
            "successfully performed IBC transfer from chain {} to chain {}",
            chains.chain_id_a(),
            chains.chain_id_b(),
        );

        let balance_c = chains
            .node_a
            .chain_driver()
            .query_balance(&wallet_c.address(), &denom_a)?;

        let b_to_a_amount = random_u128_range(1, a_to_b_amount);

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {}",
            chains.chain_id_b(),
            chains.chain_id_a(),
            b_to_a_amount,
        );

        chains.node_b.chain_driver().ibc_transfer_token(
            &channel.port_b.as_ref(),
            &channel.channel_id_b.as_ref(),
            &wallet_b.as_ref(),
            &wallet_c.address(),
            &denom_b.with_amount(b_to_a_amount * 10u128.pow(18)).as_ref(),
        )?;

        chains.node_b.chain_driver().assert_eventual_wallet_amount(
            &wallet_b.address(),
            &denom_b
                .with_amount(a_to_b_amount * 10u128.pow(18) - b_to_a_amount * 10u128.pow(18))
                .as_ref(),
        )?;

        chains.node_a.chain_driver().assert_eventual_wallet_amount(
            &wallet_c.address(),
            &(balance_c + b_to_a_amount * 10u128.pow(18)).as_ref(),
        )?;

        info!(
            "successfully performed reverse IBC transfer from chain {} back to chain {}",
            chains.chain_id_b(),
            chains.chain_id_a(),
        );

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

        chains.node_b.chain_driver().ibc_transfer_token(
            &channel.port_b.as_ref(),
            &channel.channel_id_b.as_ref(),
            &wallet_b.as_ref(),
            &wallet_c.address(),
            &chains.node_b.denom().with_amount(b_to_a_amount).as_ref(),
        )?;

        chains.node_b.chain_driver().assert_eventual_wallet_amount(
            &wallet_b.address(),
            &(balance_b - b_to_a_amount).as_ref(),
        )?;

        chains.node_a.chain_driver().assert_eventual_wallet_amount(
            &wallet_c.address(),
            &ibc_denom_a.with_amount(b_to_a_amount).as_ref(),
        )?;

        info!(
            "successfully performed IBC transfer from chain {} to chain {}/samoleans",
            chains.chain_id_b(),
            chains.chain_id_a(),
        );

        let balance_d = chains
            .node_b
            .chain_driver()
            .query_balance(&wallet_d.address(), &chains.node_b.denom())?;

        let a_to_b_amount = random_u128_range(1, b_to_a_amount);

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {}/samoleans",
            chains.chain_id_a(),
            chains.chain_id_b(),
            a_to_b_amount,
        );

        chains.node_a.chain_driver().ibc_transfer_token(
            &channel.port_a.as_ref(),
            &channel.channel_id_a.as_ref(),
            &wallet_c.as_ref(),
            &wallet_d.address(),
            &ibc_denom_a.with_amount(a_to_b_amount).as_ref(),
        )?;

        info!(
            "Waiting for user on chain B to receive IBC transferred amount of {}",
            a_to_b_amount
        );

        chains.node_a.chain_driver().assert_eventual_wallet_amount(
            &wallet_c.address(),
            &ibc_denom_a
                .with_amount(b_to_a_amount - a_to_b_amount)
                .as_ref(),
        )?;

        chains.node_b.chain_driver().assert_eventual_wallet_amount(
            &wallet_d.address(),
            &(balance_d + a_to_b_amount).as_ref(),
        )?;

        info!(
            "successfully performed reverse IBC transfer from chain {} back to chain {}/samoleans",
            chains.chain_id_a(),
            chains.chain_id_b(),
        );

        Ok(())
    }
}
