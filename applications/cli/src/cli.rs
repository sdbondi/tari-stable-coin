// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use clap::Parser;
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Debug, clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
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
    #[clap(short = 'u', long, default_value = "http://localhost:19100")]
    pub service_url: Url,
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum Command {
    #[clap(subcommand)]
    Issuer(IssuerSubcommand),
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum IssuerSubcommand {
    Init,
    Create(IssuerCreateSubcommand),
}

#[derive(Clone, Debug, clap::Args)]
pub struct IssuerCreateSubcommand {
    #[clap(short, long)]
    pub public_key: String,
}
