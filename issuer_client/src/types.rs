// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use serde::{Deserialize, Serialize};
use tari_template_lib::models::TemplateAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsGetResponse {
    #[serde(flatten)]
    pub settings: SettingsJson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsPutRequest {
    #[serde(flatten)]
    pub settings: SettingsJson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsPutResponse {
    #[serde(flatten)]
    pub settings: SettingsJson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsJson {
    pub indexer_json_rpc_url: String,
    pub issuer_template: Option<TemplateAddress>,
}
