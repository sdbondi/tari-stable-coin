// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use crate::handlers::auth::AuthClaims;
use crate::handlers::HandlerContext;

pub async fn create(
    context: &HandlerContext,
    _auth_claims: AuthClaims,
    req: serde_json::Value,
) -> Result<(), anyhow::Error> {
    // let store = context.store();

    Ok(())
}
