//  Copyright 2023 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

use crate::daemon_client::DaemonClient;
use clap::Parser;
use clap::Subcommand;
use multiaddr::Multiaddr;
use tari_engine_types::parse_arg;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub(crate) struct Cli {
    #[clap(long, short = 'e', alias = "endpoint", env = "JRPC_ENDPOINT")]
    pub daemon_jrpc_endpoint: Option<String>,
    #[clap(long, short = 't', alias = "token", env = "AUTH_TOKEN")]
    pub auth_token: Option<String>,
    #[clap(long, alias = "template_address", default_value = "")]
    pub template: String,
    #[clap(long, short = 'd')]
    pub dump_buckets: bool,
    #[clap(long)]
    pub dry_run: bool,
    #[clap(subcommand)]
    pub command: Command,
    #[clap(long, short = 'f', default_value = "1000")]
    pub max_fee: u64,
    #[clap(long, short = 'a', default_value = "TestAccount_0")]
    pub default_account: String,
}

impl Cli {
    pub fn init() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Subcommand, Clone)]
pub(crate) enum Command {
    Login(login::Command),

    Create(create::Command),

    TransferTo(transfer_to::Command),

    Deposit(deposit::Command),

    DepositAuthBadge(deposit_auth_badge::Command),

    FreezeAccount(freeze_account::Command),

    UnfreezeAccount(unfreeze_account::Command),
}

pub mod login {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use std::fs;

    #[derive(Debug, Args, Clone)]
    pub struct Command {}

    impl Command {
        pub async fn run(self, mut client: DaemonClient) {
            let token = client.login().await;
            fs::write("token.data", token).unwrap();
        }
    }
}

pub(crate) mod create {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use serde_json::json;
    use std::str::FromStr;
    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;
    use tari_template_lib::args;
    use tari_template_lib::prelude::Amount;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::SubstateRequirement;
    use tari_utilities::hex::from_hex;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub issuer_component: String,

        pub user_public_key: String,

        pub admin_proof: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            template_address: TemplateAddress,
            dump_buckets: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let function = "create".to_string();

            client
                .submit_instruction(
                    Instruction::CallFunction {
                        template_address,
                        function,
                        args: vec![
                            parse_arg(&self.issuer_component).unwrap(),
                            parse_arg(&self.user_public_key).unwrap(),
                            parse_arg(&self.admin_proof).unwrap(),
                        ],
                    },
                    dump_buckets,
                    false,
                    fees,
                    vec![],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod transfer_to {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use serde_json::json;
    use std::str::FromStr;
    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;
    use tari_template_lib::args;
    use tari_template_lib::prelude::Amount;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::SubstateRequirement;
    use tari_utilities::hex::from_hex;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub destination_account: String,

        pub withdraw_proof: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "transfer_to".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![
                    parse_arg(&self.destination_account).unwrap(),
                    parse_arg(&self.withdraw_proof).unwrap(),
                ],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod deposit {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use serde_json::json;
    use std::str::FromStr;
    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;
    use tari_template_lib::args;
    use tari_template_lib::prelude::Amount;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::SubstateRequirement;
    use tari_utilities::hex::from_hex;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub proof: String,

        pub funds_amount: u64,
        pub funds_resource: String,
        pub funds_withdraw_from_component: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "deposit".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.funds_withdraw_from_component)
                    .unwrap(),
                method: "withdraw".to_string(),
                args: args![
                    ResourceAddress::from_str(&self.funds_resource).unwrap(),
                    self.funds_amount
                ],
            });
            instructions.push(Instruction::PutLastInstructionOutputOnWorkspace {
                key: b"bucket_funds".to_vec(),
            });

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![parse_arg(&self.proof).unwrap(), Variable("bucket_funds"),],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod deposit_auth_badge {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use serde_json::json;
    use std::str::FromStr;
    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;
    use tari_template_lib::args;
    use tari_template_lib::prelude::Amount;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::SubstateRequirement;
    use tari_utilities::hex::from_hex;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub admin_proof: String,

        pub badge_amount: u64,
        pub badge_resource: String,
        pub badge_withdraw_from_component: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "deposit_auth_badge".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.badge_withdraw_from_component)
                    .unwrap(),
                method: "withdraw".to_string(),
                args: args![
                    ResourceAddress::from_str(&self.badge_resource).unwrap(),
                    self.badge_amount
                ],
            });
            instructions.push(Instruction::PutLastInstructionOutputOnWorkspace {
                key: b"bucket_badge".to_vec(),
            });

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![
                    parse_arg(&self.admin_proof).unwrap(),
                    Variable("bucket_badge"),
                ],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod freeze_account {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use serde_json::json;
    use std::str::FromStr;
    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;
    use tari_template_lib::args;
    use tari_template_lib::prelude::Amount;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::SubstateRequirement;
    use tari_utilities::hex::from_hex;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub _admin_proof: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "freeze_account".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![parse_arg(&self._admin_proof).unwrap(),],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod unfreeze_account {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use serde_json::json;
    use std::str::FromStr;
    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;
    use tari_template_lib::args;
    use tari_template_lib::prelude::Amount;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::SubstateRequirement;
    use tari_utilities::hex::from_hex;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub _admin_proof: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "unfreeze_account".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![parse_arg(&self._admin_proof).unwrap(),],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}
