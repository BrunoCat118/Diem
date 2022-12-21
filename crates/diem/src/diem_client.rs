// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use diem_client::{views, Client, Response, WaitForTransactionError};

use anyhow::Result;
use diem_types::{
    account_address::AccountAddress, ledger_info::LedgerInfoWithSignatures,
    transaction::SignedTransaction, trusted_state::TrustedState, waypoint::Waypoint,
};
use reqwest::Url;
use std::time::Duration;

/// A client connection to an AdmissionControl (AC) service. `DiemClient` also
/// handles verifying the server's responses, retrying on non-fatal failures, and
/// ratcheting our latest verified state, which includes the latest verified
/// version and latest verified epoch change ledger info.
///
/// ### Note
///
/// `DiemClient` will reject out-of-date responses. For example, this can happen if
///
/// 1. We make a request to the remote AC service.
/// 2. The remote service crashes and it forgets the most recent state or an
///    out-of-date replica takes its place.
/// 3. We make another request to the remote AC service. In this case, the remote
///    AC will be behind us and we will reject their response as stale.
#[allow(dead_code)]
pub struct DiemClient {
    client: Client,
    /// The latest verified chain state.
    trusted_state: TrustedState,
    /// The most recent epoch change ledger info. This is `None` if we only know
    /// about our local [`Waypoint`] and have not yet ratcheted to the remote's
    /// latest state.
    latest_epoch_change_li: Option<LedgerInfoWithSignatures>,
}

impl DiemClient {
    /// Construct a new Client instance.
    pub fn new(url: Url, waypoint: Waypoint) -> Result<Self> {
        let initial_trusted_state = TrustedState::from_epoch_waypoint(waypoint);
        let client = Client::new(url.to_string());

        Ok(DiemClient {
            client,
            trusted_state: initial_trusted_state,
            latest_epoch_change_li: None,
        })
    }

    /// Submits a transaction and bumps the sequence number for the sender, pass in `None` for
    /// sender_account if sender's address is not managed by the client.
    pub async fn submit_transaction(&self, transaction: &SignedTransaction) -> Result<()> {
        self.client
            .submit(transaction)
            .await
            .map_err(Into::into)
            .map(Response::into_inner)
    }

    /// Retrieves account information
    /// - If `with_state_proof`, will also retrieve state proof from node and update trusted_state accordingly
    pub async fn get_account(
        &self,
        account: &AccountAddress,
    ) -> Result<Option<views::AccountView>> {
        self.client
            .get_account(*account)
            .await
            .map_err(Into::into)
            .map(Response::into_inner)
    }

    /// Get transaction from validator by account and sequence number.
    pub async fn get_txn_by_acc_seq(
        &self,
        account: &AccountAddress,
        sequence_number: u64,
        fetch_events: bool,
    ) -> Result<Option<views::TransactionView>> {
        self.client
            .get_account_transaction(*account, sequence_number, fetch_events)
            .await
            .map_err(Into::into)
            .map(Response::into_inner)
    }

    pub async fn wait_for_transaction(
        &self,
        txn: &SignedTransaction,
        timeout: Duration,
    ) -> Result<views::TransactionView, WaitForTransactionError> {
        self.client
            .wait_for_signed_transaction(txn, Some(timeout), None)
            .await
            .map(Response::into_inner)
    }

    /// Gets the currency info stored on-chain
    pub async fn get_currency_info(&self) -> Result<Vec<views::CurrencyInfoView>> {
        self.client
            .get_currencies()
            .await
            .map_err(Into::into)
            .map(Response::into_inner)
    }
}
