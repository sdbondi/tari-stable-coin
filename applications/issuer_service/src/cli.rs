// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Debug, clap::Parser)]
pub struct Cli {
    #[clap(flatten)]
    pub common: Common,
}

impl Cli {
    pub fn init() -> Self {
        Self::parse()
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub struct Common {
    #[clap(short = 'i', long, default_value = "http://localhost:18300")]
    pub indexer_json_rpc_url: Url,
    #[clap(short = 'd', long, default_value = "./data/store.sqlite")]
    pub db_path: PathBuf,
    /// The secret used to sign JWTs. This will be removed in future versions.
    #[clap(long)]
    pub jwt_secret: String,
    #[clap(short = 'l', long, default_value = "127.0.0.1:19100")]
    pub server_listen_address: SocketAddr,
}

impl Common {
    pub fn db_url(&self) -> Url {
        format!("sqlite://{}?mode=rwc", self.db_path.display())
            .parse()
            .unwrap()
    }
}
