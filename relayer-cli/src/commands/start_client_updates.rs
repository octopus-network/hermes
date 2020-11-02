
use abscissa_core::{Command, Options, Runnable};

use relayer::config::Config;

use crate::prelude::*;
use ibc::ics24_host::identifier::ClientId;
use relayer::tx::client::{ClientOptions, create_client, update_client};

#[derive(Command, Debug, Options)]
pub struct StartClientUpdatesCmd {
    #[options(free, help = "identifier of the destination chain")]
    dest_chain_id: String,

    #[options(free, help = "identifier of the source chain")]
    src_chain_id: String,

    #[options(
    free,
    help = "identifier of the client to be updated on destination chain"
    )]
    dest_client_id: ClientId,

    #[options(help = "account sequence of the signer", short = "s")]
    account_sequence: u64,

    #[options(
    help = "json key file for the signer, must include mnemonic",
    short = "k"
    )]
    seed_file: String,
}

impl Runnable for StartClientUpdatesCmd {
    fn run(&self) {
        let config = app_config();

        // Validate parameters
        let dest_chain_id = self.dest_chain_id
            .parse()
            .map_err(|_| "bad destination chain identifier".to_string()).unwrap();

        let src_chain_id = self.src_chain_id
            .parse()
            .map_err(|_| "bad source chain identifier".to_string()).unwrap();

        // Get the source and destination chain configuration
        let dest_chain_config = config
            .chains
            .iter()
            .find(|c| c.id == dest_chain_id)
            .ok_or_else(|| "missing destination chain configuration".to_string()).unwrap();

        let src_chain_config = config
            .chains
            .iter()
            .find(|c| c.id == src_chain_id)
            .ok_or_else(|| "missing source chain configuration".to_string()).unwrap();

        let signer_seed = std::fs::read_to_string(self.seed_file.clone()).unwrap();

        let opts = ClientOptions{
            dest_client_id: self.dest_client_id.clone(),
            dest_chain_config: dest_chain_config.clone(),
            src_chain_config: src_chain_config.clone(),
            signer_seed,
            account_sequence: self.account_sequence
        };

        status_info!("start client updates for ", "{:?}", opts);

        let config = app_config().clone();

        let res = start_client_udpates(config, opts);
        match res {
            Ok(()) => status_info!("all good", ""),
            Err(e) => status_info!("start_client_udpates failed, error: ", "{}", e),
        }
    }
}

use std::thread;
use std::time::Duration;

fn start_client_udpates(config: Config, opts: ClientOptions) -> Result<(), BoxError> {
    let mut new_opts = opts.clone();

    if let Ok(res) = create_client(opts) {
        new_opts.account_sequence += 1;
    }

    loop {
        println!("update client Tx nonce {:?}\n", new_opts.account_sequence);
        update_client(new_opts.clone()).unwrap();
        thread::sleep(Duration::from_millis(1000));
        new_opts.account_sequence = new_opts.clone().account_sequence + 1;
    }
}