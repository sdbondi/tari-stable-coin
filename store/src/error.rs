// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("DB error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("Failed to decode {operation} {item}: {details}")]
    Decode {
        operation: &'static str,
        item: &'static str,
        details: String,
    },
    #[error("Failed to encode {operation} {item}: {details}")]
    Encode {
        operation: &'static str,
        item: &'static str,
        details: String,
    },
}
