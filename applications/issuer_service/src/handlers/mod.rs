// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

pub mod auth;
pub mod issuer;

mod context;
pub mod settings;

use crate::handlers::auth::AuthClaims;
use async_trait::async_trait;
pub use context::*;
use std::future::Future;

#[async_trait]
pub trait Handler<'a, TReq> {
    type Response;

    async fn handle(
        &mut self,
        context: &'a HandlerContext,
        auth_claims: AuthClaims,
        req: TReq,
    ) -> Result<Self::Response, HandlerError>;
}

#[async_trait]
impl<'a, F, TReq, TResp, TFut, TErr> Handler<'a, TReq> for F
where
    F: FnMut(&'a HandlerContext, AuthClaims, TReq) -> TFut + Sync + Send,
    TFut: Future<Output = Result<TResp, TErr>> + Send,
    TReq: Send + 'static,
    TErr: Into<HandlerError>,
{
    type Response = TResp;

    async fn handle(
        &mut self,
        context: &'a HandlerContext,
        auth_claims: AuthClaims,
        req: TReq,
    ) -> Result<Self::Response, HandlerError> {
        let resp = self(context, auth_claims, req).await.map_err(Into::into)?;
        Ok(resp)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HandlerError {
    #[error("Error: {0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Not found")]
    NotFound,
}
