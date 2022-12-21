// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0
use crate::{
    client_proxy::{retrieve_waypoint, ClientProxy},
    common::{
        config::ConfigPath,
        types::{CliError, Command},
    },
};

use diem_types::chain_id::ChainId;

use async_trait::async_trait;
use clap::Parser;

use std::str::{self, FromStr};

/// Mint coins to an account
///
/// This mints coins to an account.
/// If the account does not exist on chain it will be created.
#[derive(Debug, Parser)]
pub struct MintAccount {
    ///Account to receive coins, authentication key is required to mint from faucet
    account: String,

    ///Amount of coins to mint
    amount: u64,

    ///Currency of coins to mint
    currency: String,
}

#[async_trait]
impl Command<String> for MintAccount {
    fn command_name(&self) -> &'static str {
        "MintAccount"
    }

    async fn execute(self) -> Result<String, CliError> {
        let config_path = ConfigPath::default();
        let config = config_path.load().unwrap();

        let chain_str = String::from(&config.chain);
        let chain_id = ChainId::from_str(&chain_str).unwrap();
        let rpc = &config.rpc_endpoint;
        let faucet_url = config.faucet_endpoint.clone();
        let waypoint = retrieve_waypoint(&config.waypoint_url).await.unwrap();

        // Diem root account/Faucet, TreasuryCompliance and DD use the same keypair for now
        let diem_root_account_file = "".to_string();
        let treasury_compliance_account_file = diem_root_account_file.clone();
        let dd_account_file = diem_root_account_file.clone();

        let mut client_proxy = ClientProxy::new(
            chain_id,
            rpc,
            &diem_root_account_file,
            &treasury_compliance_account_file,
            &dd_account_file,
            Some(faucet_url),
            waypoint,
            false,
        )
        .await
        .expect("Failed to construct client.");

        match client_proxy
            .mint_coins(self.account.clone(), self.amount, self.currency.clone())
            .await
        {
            Ok(_) => {}
            Err(e) => {
                panic!("Error minting coins: {}", e)
            }
        };

        let amount = self.amount.to_string();
        let currency = self.currency;
        let account = self.account;
        let result = format!("Successfully minted {amount} {currency} to {account}");
        Ok(result)
    }
}
