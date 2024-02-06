// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use crate::error::StoreError;
use crate::serialization::deserialize_hex_try_from;
use sqlx::{Executor, Row, SqliteExecutor};
use std::fmt::Display;
use tari_template_lib::models::TemplateAddress;

#[derive(Debug, Clone)]
pub struct Settings {
    pub indexer_json_rpc_url: String,
    pub issuer_template: Option<TemplateAddress>,
}

impl Settings {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn load<'a, E: SqliteExecutor<'a>>(tx: E) -> Result<Self, StoreError> {
        let setting_parts = sqlx::query_as!(SingleSetting, "SELECT name, value FROM settings")
            .fetch_all(tx)
            .await?;

        let mut settings = Self::new();
        for setting in setting_parts {
            let Some(value) = setting.value else {
                continue;
            };
            match setting.name.as_str() {
                "indexer.json_rpc_url" => settings.indexer_json_rpc_url = value,
                "issuer.template" => {
                    settings.issuer_template = Some(deserialize_hex_try_from(&value)?)
                }
                _ => {}
            }
        }

        Ok(settings)
    }

    pub async fn save<'a, E: SqliteExecutor<'a> + Copy>(&self, tx: E) -> Result<(), StoreError> {
        self.upsert_setting(tx, "indexer.json_rpc_url", Some(&self.indexer_json_rpc_url))
            .await?;
        self.upsert_setting(tx, "issuer.template", self.issuer_template.as_ref())
            .await?;
        Ok(())
    }

    async fn upsert_setting<'a, E: SqliteExecutor<'a>, T: ToString>(
        &self,
        tx: E,
        key: &str,
        value: Option<&T>,
    ) -> Result<(), StoreError> {
        sqlx::query(&format!("INSERT INTO settings (name, value) VALUES ('{key}', $1) ON CONFLICT (name) DO UPDATE SET value = $1"))
            .bind(value.map(|v| v.to_string()))
            .execute(tx)
            .await?;

        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            indexer_json_rpc_url: "http://localhost:18300".to_string(),
            issuer_template: None,
        }
    }
}

struct SingleSetting {
    name: String,
    value: Option<String>,
}
