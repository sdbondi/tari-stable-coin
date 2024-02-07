//  Copyright 2023 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

mod cli;
mod daemon_client;



use crate::cli::Cli;
use crate::cli::Command;
use crate::daemon_client::DaemonClient;

use std::fs;
use tari_utilities::hex::from_hex;


#[tokio::main]
async fn main() {
    let cli = Cli::init();
    let jrpc = cli.daemon_jrpc_endpoint.clone().unwrap_or_else(|| "http://127.0.0.1:18016".to_string());
    let token = cli.auth_token.as_ref().map(|a| a.to_string()).or(fs::read_to_string("token.data").ok());

    let client= DaemonClient::new(jrpc, token, cli.default_account.clone());
    let template_address = from_hex(&cli.template).unwrap().try_into().unwrap();
    match cli.command {
        Command::Login(com) => {
              com.run(client).await;
        }
        
        Command::Create(com) => {
            
            com.run(client, template_address, cli.dump_buckets, cli.max_fee).await;
            
        }
        
        Command::TransferTo(com) => {
            
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee).await;
            
        }
        
        Command::Deposit(com) => {
            
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee).await;
            
        }
        
        Command::DepositAuthBadge(com) => {
            
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee).await;
            
        }
        
        Command::FreezeAccount(com) => {
            
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee).await;
            
        }
        
        Command::UnfreezeAccount(com) => {
            
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee).await;
            
        }
        
    }
}
