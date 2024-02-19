//  Copyright 2023 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

use multiaddr::Multiaddr;
use reqwest;
use serde_json::json;
use serde_json::Value;
use std::str::FromStr;
use tari_engine_types::instruction::Instruction;
use tari_template_lib::args;
use tari_template_lib::models::Amount;
use tari_transaction::{SubstateRequirement, TransactionId};
use tari_wallet_daemon_client::types::AuthLoginRequest;
use tari_wallet_daemon_client::types::CallInstructionRequest;
use tari_wallet_daemon_client::types::TransactionSubmitRequest;
use tari_wallet_daemon_client::types::TransactionWaitResultRequest;
use tari_wallet_daemon_client::types::TransactionWaitResultResponse;
use tari_wallet_daemon_client::types::{AccountGetResponse, AuthLoginAcceptRequest};
use tari_wallet_daemon_client::ComponentAddressOrName;
use tari_wallet_daemon_client::WalletDaemonClient;

pub struct DaemonClient {
    endpoint: String,
    auth_token: Option<String>,
    default_account: String,
}

impl DaemonClient {
    pub(crate) fn new(
        endpoint: String,
        auth_token: Option<String>,
        default_account: String,
    ) -> Self {
        Self {
            endpoint,
            auth_token,
            default_account,
        }
    }

    pub async fn login(&mut self) -> String {
        let mut client =
            WalletDaemonClient::connect(&self.endpoint, self.auth_token.clone()).unwrap();
        let r = client
            .auth_request(&AuthLoginRequest {
                permissions: vec!["Admin".to_string()],
                duration: None,
            })
            .await
            .unwrap();

        dbg!(&r);

        r.auth_token
    }

    pub async fn grant(&mut self, auth_token: String, name: String) -> String {
        let mut client =
            WalletDaemonClient::connect(&self.endpoint, self.auth_token.clone()).unwrap();
        let r = client
            .auth_accept(&AuthLoginAcceptRequest { auth_token, name })
            .await
            .unwrap();

        dbg!(&r);

        r.permissions_token
    }

    pub async fn submit_instruction(
        &mut self,
        instruction: Instruction,
        dump_buckets: bool,
        is_dry_run: bool,
        fees: u64,
        other_inputs: Vec<SubstateRequirement>,
    ) -> TransactionId {
        self.submit_instructions(
            vec![instruction],
            dump_buckets,
            is_dry_run,
            fees,
            other_inputs,
        )
        .await
    }

    pub async fn submit_instructions(
        &mut self,
        instructions: Vec<Instruction>,
        dump_buckets: bool,
        is_dry_run: bool,
        max_fee: u64,
        other_inputs: Vec<SubstateRequirement>,
    ) -> TransactionId {
        let mut client =
            WalletDaemonClient::connect(&self.endpoint, self.auth_token.clone()).unwrap();
        //let r = client.list_keys().await;

        //dbg!(r);

        // let tx = CallInstructionRequest {
        //     instructions,
        //     fee_account: ComponentAddressOrName::Name(self.default_account.clone()),
        //     dump_outputs_into: if dump_buckets {
        //         Some(ComponentAddressOrName::Name(self.default_account.clone()))
        //     } else {
        //         None
        //     },
        //     max_fee,
        //     inputs: other_inputs,
        //     override_inputs: None,
        //     is_dry_run,
        //     proof_ids: vec![],
        //     new_outputs: None,
        //     min_epoch: None,
        //     max_epoch: None,
        // };
        let AccountGetResponse { account, .. } = client
            .accounts_get(ComponentAddressOrName::Name(self.default_account.clone()))
            .await
            .unwrap();

        let fee_instructions = vec![Instruction::CallMethod {
            component_address: account.address.as_component_address().unwrap(),
            method: "pay_fee".to_string(),
            args: args![Amount::try_from(max_fee).unwrap()],
        }];

        let tx = TransactionSubmitRequest {
            signing_key_index: Some(account.key_index),
            fee_instructions,
            instructions,
            inputs: other_inputs,
            override_inputs: false,
            is_dry_run,
            proof_ids: vec![],
            min_epoch: None,
            max_epoch: None,
        };

        let r2 = client.submit_transaction(tx).await.unwrap();
        dbg!(&r2);
        //"dump_outputs_into": self.default_account,

        r2.transaction_id
    }

    pub async fn wait_for_transaction_result(
        &mut self,
        tx_id: TransactionId,
    ) -> TransactionWaitResultResponse {
        let mut client =
            WalletDaemonClient::connect(&self.endpoint, self.auth_token.clone()).unwrap();
        let result = client
            .wait_transaction_result(TransactionWaitResultRequest {
                transaction_id: tx_id,
                timeout_secs: None,
            })
            .await
            .unwrap();
        result
    }

    pub async fn get_default_account(&mut self) -> AccountGetResponse {
        let mut client =
            WalletDaemonClient::connect(&self.endpoint, self.auth_token.clone()).unwrap();
        client
            .accounts_get(ComponentAddressOrName::Name(self.default_account.clone()))
            .await
            .unwrap()
    }
}
