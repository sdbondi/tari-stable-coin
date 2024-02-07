//  Copyright 2023 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

use clap::Parser;
use clap::Subcommand;
use multiaddr::Multiaddr;
use crate::daemon_client::DaemonClient;
use tari_engine_types::parse_arg;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub(crate) struct Cli {
    #[clap(long, short = 'e', alias = "endpoint", env = "JRPC_ENDPOINT")]
    pub daemon_jrpc_endpoint: Option<String>,
    #[clap(long, short='t', alias= "token")]
    pub auth_token: Option<String>,
    #[clap(long, alias="template_address", default_value ="")]
    pub template: String,
    #[clap(long, short='d')]
    pub dump_buckets: bool,
    #[clap(long)]
    pub dry_run: bool,
    #[clap(subcommand)]
    pub command: Command,
    #[clap(long, short='f', default_value="1000")]
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
    
    Instantiate(instantiate::Command),
    
    IncreaseSupply(increase_supply::Command),
    
    DecreaseSupply(decrease_supply::Command),
    
    TotalSupply(total_supply::Command),
    
    Withdraw(withdraw::Command),
    
    Deposit(deposit::Command),
    
    CreateNewAdmin(create_new_admin::Command),
    
    CreateNewUser(create_new_user::Command),
    
    BlacklistUser(blacklist_user::Command),
    
    RemoveFromBlacklist(remove_from_blacklist::Command),
    
    GetUserData(get_user_data::Command),
    
    SetUserData(set_user_data::Command),
    
}


pub mod login {
  use clap::Args;
  use crate::daemon_client::DaemonClient;
    use std::fs;

    #[derive(Debug, Args, Clone)]
    pub struct Command {

    }

    impl Command {
    pub async fn run(self, mut client: DaemonClient) {
       let token = client.login().await;
       fs::write("token.data", token).unwrap();
    }
    }
}


pub(crate) mod instantiate {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
          
       pub initial_token_supply : String,
           
       
      
      
          
       pub token_symbol : String,
           
       
      
      
          
       pub token_metadata : String,
           
       
      
   }

   impl Command {

    
       pub async fn run(self, mut client: DaemonClient, template_address: TemplateAddress, dump_buckets: bool, fees: u64) {

       // let template_address= ;
        let  function = "instantiate".to_string();



                client.submit_instruction(Instruction::CallFunction {
                    template_address,
                    function,
                    args: vec![
                        
      
       parse_arg(&self.initial_token_supply).unwrap(),
      
      
      
       parse_arg(&self.token_symbol).unwrap(),
      
      
      
       parse_arg(&self.token_metadata).unwrap(),
      
      
                    ]
           }, dump_buckets, false, fees, vec![]

            ).await;
            println!("done");

       }

       

   }
}

pub(crate) mod increase_supply {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
      
          
       pub amount : String,
           
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "increase_supply".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
          
            
           parse_arg(&self.amount).unwrap(),
           
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod decrease_supply {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
      
          
       pub amount : String,
           
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "decrease_supply".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
          
            
           parse_arg(&self.amount).unwrap(),
           
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod total_supply {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "total_supply".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod withdraw {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
      
          
       pub amount : String,
           
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "withdraw".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
          
            
           parse_arg(&self.amount).unwrap(),
           
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod deposit {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
      
          
        pub bucket_amount : u64,
        pub bucket_resource: String,
        pub bucket_withdraw_from_component: String,
           
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "deposit".to_string();

    let mut instructions = vec![];
    
    
    
     
    
       instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.bucket_withdraw_from_component).unwrap(),
                method: "withdraw".to_string(),
                args: args![ResourceAddress::from_str(&self.bucket_resource).unwrap(), self.bucket_amount],
            });
  instructions.push(Instruction::PutLastInstructionOutputOnWorkspace {
            key: b"bucket_bucket".to_vec(),
        });
     
     
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
          
            
              Variable("bucket_bucket"),
          
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod create_new_admin {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "create_new_admin".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod create_new_user {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
      
          
       pub user_id : String,
           
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "create_new_user".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
          
            
           parse_arg(&self.user_id).unwrap(),
           
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod blacklist_user {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
      
          
       pub vault_id : String,
           
       
      
      
          
       pub user_id : String,
           
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "blacklist_user".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
          
            
           parse_arg(&self.vault_id).unwrap(),
           
          
          
          
            
           parse_arg(&self.user_id).unwrap(),
           
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod remove_from_blacklist {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
      
          
       pub user_id : String,
           
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "remove_from_blacklist".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
          
            
           parse_arg(&self.user_id).unwrap(),
           
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod get_user_data {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
      
          
       pub user_id : String,
           
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "get_user_data".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
          
            
           parse_arg(&self.user_id).unwrap(),
           
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}

pub(crate) mod set_user_data {
   use clap::Args;
   use crate::daemon_client::DaemonClient;
   use serde_json::json;
    use tari_engine_types::parse_arg;
    use tari_engine_types::instruction::Instruction;
    use tari_utilities::hex::Hex;
     use tari_utilities::hex::from_hex;
use tari_engine_types::TemplateAddress;
use tari_template_lib::prelude::ComponentAddress;
use tari_transaction::SubstateRequirement;
use tari_template_lib::args;
 use tari_template_lib::prelude::Amount;
 use tari_template_lib::prelude::ResourceAddress;
  use std::str::FromStr;




   #[derive(Debug, Args, Clone)]
   pub struct Command {
      
      
       pub component_address: String,
       
      
      
          
       pub user_id : String,
           
       
      
      
          
       pub data : String,
           
       
      
   }

   impl Command {

    

 pub async fn run(self, mut client: DaemonClient, dump_buckets: bool, is_dry_run: bool, fees: u64) {

       // let template_address= ;
        let method = "set_user_data".to_string();

    let mut instructions = vec![];
    
    instructions.push(
    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                        method,
                        args: args![
                            
          
          
          
            
           parse_arg(&self.user_id).unwrap(),
           
          
          
          
            
           parse_arg(&self.data).unwrap(),
           
          
          
                        ]
                   }
    );

                client.submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![format!("component_{}", self.component_address).parse().unwrap()]

            ).await;
            println!("done");

       }


    

   }
}


