// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use crate::handlers::auth::AuthClaims;
use crate::handlers::HandlerContext;
use stable_coin_issuer_client::types::{
    SettingsGetResponse, SettingsJson, SettingsPutRequest, SettingsPutResponse,
};
use stable_coin_store::Settings;

pub async fn get(
    context: &HandlerContext,
    _claims: AuthClaims,
    _req: serde_json::Value,
) -> Result<SettingsGetResponse, anyhow::Error> {
    let settings = Settings::load(context.store()).await?;

    Ok(SettingsGetResponse {
        settings: SettingsJson {
            indexer_json_rpc_url: settings.indexer_json_rpc_url,
            issuer_template: settings.issuer_template,
        },
    })
}

pub async fn put(
    context: &HandlerContext,
    _claims: AuthClaims,
    req: SettingsPutRequest,
) -> Result<SettingsPutResponse, anyhow::Error> {
    Settings {
        indexer_json_rpc_url: req.settings.indexer_json_rpc_url,
        issuer_template: req.settings.issuer_template,
    }
    .save(context.store())
    .await?;

    let settings = Settings::load(context.store()).await?;
    Ok(SettingsPutResponse {
        settings: SettingsJson {
            indexer_json_rpc_url: settings.indexer_json_rpc_url,
            issuer_template: settings.issuer_template,
        },
    })
}
