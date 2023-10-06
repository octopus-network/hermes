use core::time::Duration;

use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::chain::chain_type::ChainType;
use crate::chain::cli::transfer::{local_transfer_token, transfer_from_chain};
use crate::chain::driver::ChainDriver;
use crate::chain::exec::simple_exec;
use crate::chain::tagged::TaggedChainDriverExt;
use crate::error::Error;
use crate::ibc::token::TaggedTokenRef;
use crate::relayer::transfer::{batched_ibc_token_transfer, ibc_token_transfer};
use crate::types::id::{TaggedChannelIdRef, TaggedPortIdRef};
use crate::types::tagged::*;
use crate::types::wallet::{Wallet, WalletAddress};
use tracing::info;

pub trait ChainTransferMethodsExt<Chain> {
    /**
       Submits an IBC token transfer transaction to `Chain` to any other
       `Counterparty` chain.

       The following parameters are accepted:

       - A `PortId` on `Chain` that corresponds to a channel connected to
         `Counterparty`.

       - A `ChannelId` on `Chain` that corresponds to a channel connected to
         `Counterparty`.

       - The [`Wallet`] of the sender on `Chain`.

       - The [`WalletAddress`] address of the recipient on `Counterparty`.

       - The denomination of the amount on `Chain`.

       - The transfer amount.
    */
    fn ibc_transfer_token<Counterparty>(
        &self,
        port_id: &TaggedPortIdRef<Chain, Counterparty>,
        channel_id: &TaggedChannelIdRef<Chain, Counterparty>,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Counterparty, &WalletAddress>,
        token: &TaggedTokenRef<Chain>,
    ) -> Result<Packet, Error>;

    fn ibc_transfer_token_with_memo_and_timeout<Counterparty>(
        &self,
        port_id: &TaggedPortIdRef<Chain, Counterparty>,
        channel_id: &TaggedChannelIdRef<Chain, Counterparty>,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Counterparty, &WalletAddress>,
        token: &TaggedTokenRef<Chain>,
        memo: Option<String>,
        timeout: Option<Duration>,
    ) -> Result<Packet, Error>;

    fn ibc_transfer_token_multiple<Counterparty>(
        &self,
        port_id: &TaggedPortIdRef<Chain, Counterparty>,
        channel_id: &TaggedChannelIdRef<Chain, Counterparty>,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Counterparty, &WalletAddress>,
        token: &TaggedTokenRef<Chain>,
        num_msgs: usize,
        memo: Option<String>,
    ) -> Result<(), Error>;

    fn local_transfer_token(
        &self,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Chain, &WalletAddress>,
        token: &TaggedTokenRef<Chain>,
    ) -> Result<(), Error>;

    fn transfer_from_chain<Counterparty>(
        &self,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Counterparty, &WalletAddress>,
        port: &PortId,
        channel: &ChannelId,
        token: &TaggedTokenRef<Chain>,
        timeout_height: &Height,
    ) -> Result<(), Error>;

    fn setup_near_ibc_transfer(&self, channel: &ChannelId) -> Result<(), Error>;
}

impl<'a, Chain: Send> ChainTransferMethodsExt<Chain> for MonoTagged<Chain, &'a ChainDriver> {
    fn ibc_transfer_token<Counterparty>(
        &self,
        port_id: &TaggedPortIdRef<Chain, Counterparty>,
        channel_id: &TaggedChannelIdRef<Chain, Counterparty>,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Counterparty, &WalletAddress>,
        token: &TaggedTokenRef<Chain>,
    ) -> Result<Packet, Error> {
        if self.0.chain_type == ChainType::Near {
            let res = simple_exec(
                &self.chain_id().to_string(),
                "near",
                &[
            "call",
            "oct.beta_oct_relay.testnet",
            "ft_transfer_call",
            &format!("{{\"receiver_id\":\"{}.ef.transfer.v5.nearibc.testnet\",\"amount\":\"{}\",\"memo\":null,\"msg\":\"{{\\\"receiver\\\":\\\"{}\\\",\\\"timeout_seconds\\\":\\\"1000\\\"}}\"}}", channel_id.0.as_str(), &format!("{}000000000000000000", token.0.amount.0), recipient.0.as_str()),
            "--accountId",
            &sender.0.address.0,
            "--gas",
            "200000000000000",
            "--depositYocto",
            "1"],
            )?
            .stdout;
            info!(
                "ft_transfer_call: {:?} channel_id: {:?}, amount: {:?}, recipient: {:?}",
                res,
                channel_id.0.as_str(),
                &format!("{}000000000000000000", token.0.amount.0),
                recipient.0.as_str()
            );

            Ok(Packet::default())
        } else {
            let rpc_client = self.rpc_client()?;
            self.value().runtime.block_on(ibc_token_transfer(
                rpc_client.as_ref(),
                &self.tx_config(),
                port_id,
                channel_id,
                sender,
                recipient,
                token,
                None,
                None,
            ))
        }
    }

    fn ibc_transfer_token_with_memo_and_timeout<Counterparty>(
        &self,
        port_id: &TaggedPortIdRef<Chain, Counterparty>,
        channel_id: &TaggedChannelIdRef<Chain, Counterparty>,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Counterparty, &WalletAddress>,
        token: &TaggedTokenRef<Chain>,
        memo: Option<String>,
        timeout: Option<Duration>,
    ) -> Result<Packet, Error> {
        let rpc_client = self.rpc_client()?;
        self.value().runtime.block_on(ibc_token_transfer(
            rpc_client.as_ref(),
            &self.tx_config(),
            port_id,
            channel_id,
            sender,
            recipient,
            token,
            memo,
            timeout,
        ))
    }

    fn ibc_transfer_token_multiple<Counterparty>(
        &self,
        port_id: &TaggedPortIdRef<Chain, Counterparty>,
        channel_id: &TaggedChannelIdRef<Chain, Counterparty>,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Counterparty, &WalletAddress>,
        token: &TaggedTokenRef<Chain>,
        num_msgs: usize,
        memo: Option<String>,
    ) -> Result<(), Error> {
        let rpc_client = self.rpc_client()?;
        self.value().runtime.block_on(batched_ibc_token_transfer(
            rpc_client.as_ref(),
            &self.tx_config(),
            port_id,
            channel_id,
            sender,
            recipient,
            token,
            num_msgs,
            memo,
        ))
    }

    fn local_transfer_token(
        &self,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Chain, &WalletAddress>,
        token: &TaggedTokenRef<Chain>,
    ) -> Result<(), Error> {
        let driver = *self.value();
        local_transfer_token(
            driver.chain_id.as_str(),
            &driver.command_path,
            &driver.home_path,
            &driver.rpc_listen_address(),
            sender.value().address.as_str(),
            recipient.value().as_str(),
            &token.value().to_string(),
        )
    }

    fn transfer_from_chain<Counterparty>(
        &self,
        sender: &MonoTagged<Chain, &Wallet>,
        recipient: &MonoTagged<Counterparty, &WalletAddress>,
        port: &PortId,
        channel: &ChannelId,
        token: &TaggedTokenRef<Chain>,
        timeout_height: &Height,
    ) -> Result<(), Error> {
        let driver = *self.value();
        let timeout_height_str = timeout_height.revision_height() + 100;
        transfer_from_chain(
            driver.chain_id.as_str(),
            &driver.command_path,
            &driver.home_path,
            &driver.rpc_listen_address(),
            sender.value().address.as_str(),
            port.as_ref(),
            channel.as_ref(),
            recipient.value().as_str(),
            &token.value().to_string(),
            &timeout_height_str.to_string(),
        )
    }

    fn setup_near_ibc_transfer(&self, channel: &ChannelId) -> Result<(), Error> {
        if self.0.chain_type == ChainType::Near {
            let res = simple_exec(
                &self.chain_id().to_string(),
                "near",
                &[
                    "call",
                    "v5.nearibc.testnet",
                    "setup_channel_escrow",
                    &format!("{{\"channel_id\":\"{}\"}}", channel.as_str()),
                    "--accountId",
                    "v5.nearibc.testnet",
                    "--gas",
                    "300000000000000",
                    "--amount",
                    "3.1",
                ],
            )?
            .stdout;
            info!("setup_channel_escrow: {}", res);

            let res = simple_exec(
                &self.chain_id().to_string(),
                "near",
                &[
            "call",
            "v5.nearibc.testnet",
            "register_asset_for_channel",
            &format!("{{\"channel_id\":\"{}\",\"trace_path\":\"\",\"base_denom\":\"OCT\",\"token_contract\":\"oct.beta_oct_relay.testnet\"}}", channel.as_str()),
            "--accountId",
            "v5.nearibc.testnet",
            "--gas",
            "300000000000000",
            "--amount",
            "0.1",
                ],
            )?
            .stdout;
            info!("register_asset_for_channel: {}", res);

            let res = simple_exec(
                &self.chain_id().to_string(),
                "near",
                &[
            "call",
            "oct.beta_oct_relay.testnet",
            "storage_deposit",
            &format!("{{\"account_id\":\"{}.ef.transfer.v5.nearibc.testnet\",\"registration_only\":false}}", channel.as_str()),
            "--accountId",
            "v5.nearibc.testnet",
            "--gas",
            "200000000000000",
            "--amount",
            "0.00125",
                ],
            )?
            .stdout;

            info!("storage_deposit: {}", res);
        }
        Ok(())
    }
}
