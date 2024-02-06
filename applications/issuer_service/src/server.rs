// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause
//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use std::future::Future;
use std::{net::SocketAddr, sync::Arc};

use crate::handlers;
use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::post,
    Router,
};
use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    JrpcResult, JsonRpcAnswer, JsonRpcExtractor, JsonRpcResponse,
};
use log::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use tower_http::cors::CorsLayer;

use super::handlers::{settings, Handler, HandlerContext, HandlerError};
use crate::handlers::auth::AuthClaims;
use crate::handlers::{auth, issuer};

const LOG_TARGET: &str = "tari::dan::wallet_daemon::json_rpc";

pub async fn listen<TShutdown: Future<Output = ()> + Send + 'static>(
    preferred_address: SocketAddr,
    context: HandlerContext,
    shutdown_signal: TShutdown,
) -> Result<(), anyhow::Error> {
    let router = Router::new()
        .route("/", post(protected_handler))
        .route("/auth/login", post(auth::login))
        .route("/json_rpc", post(protected_handler))
        .layer(Extension(Arc::new(context)))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(preferred_address).await?;
    let local_addr = listener.local_addr()?;
    let server = axum::serve(listener, router).with_graceful_shutdown(shutdown_signal);

    info!(target: LOG_TARGET, "🌐 JSON-RPC listening on {}", local_addr);
    server.await?;

    info!(target: LOG_TARGET, "💤 Stopping JSON-RPC");
    Ok(())
}

async fn protected_handler(
    Extension(context): Extension<Arc<HandlerContext>>,
    auth_claims: AuthClaims,
    value: JsonRpcExtractor,
) -> JrpcResult {
    info!(target: LOG_TARGET, "🌐 JSON-RPC request: {}", value.method);
    debug!(target: LOG_TARGET, "🌐 JSON-RPC request: {:?}", value);
    match value.method.as_str().split_once('.') {
        Some(("auth", method)) => match method {
            "get_claims" => call_handler(context, auth_claims, value, auth::get_claims).await,
            _ => Ok(value.method_not_found(&value.method)),
        },
        Some(("settings", method)) => match method {
            "get" => call_handler(context, auth_claims, value, settings::get).await,
            "put" => call_handler(context, auth_claims, value, settings::put).await,
            _ => Ok(value.method_not_found(&value.method)),
        },
        Some(("issuer", method)) => match method {
            "create" => call_handler(context, auth_claims, value, issuer::create).await,
            _ => Ok(value.method_not_found(&value.method)),
        },
        _ => Ok(value.method_not_found(&value.method)),
    }
}

async fn call_handler<H, TReq, TResp>(
    context: Arc<HandlerContext>,
    claims: AuthClaims,
    value: JsonRpcExtractor,
    mut handler: H,
) -> JrpcResult
where
    TReq: DeserializeOwned,
    TResp: Serialize,
    H: for<'a> Handler<'a, TReq, Response = TResp>,
{
    let answer_id = value.get_answer_id();

    let resp = handler
        .handle(
            &context,
            claims,
            value.parse_params().map_err(|e| {
                match &e.result {
                    JsonRpcAnswer::Result(_) => {
                        unreachable!("parse_params() error should not return a result")
                    }
                    JsonRpcAnswer::Error(e) => {
                        warn!(target: LOG_TARGET, "🌐 JSON-RPC params error: {}", e);
                    }
                }
                e
            })?,
        )
        .await
        .map_err(|e| resolve_handler_error(answer_id, &e))?;
    Ok(JsonRpcResponse::success(answer_id, resp))
}

fn resolve_handler_error(answer_id: i64, e: &HandlerError) -> JsonRpcResponse {
    match e {
        HandlerError::Anyhow(e) => resolve_any_error(answer_id, e),
        HandlerError::NotFound => JsonRpcResponse::error(
            answer_id,
            JsonRpcError::new(
                JsonRpcErrorReason::ApplicationError(404),
                e.to_string(),
                json!({}),
            ),
        ),
    }
}

fn resolve_any_error(answer_id: i64, e: &anyhow::Error) -> JsonRpcResponse {
    warn!(target: LOG_TARGET, "🌐 JSON-RPC error: {}", e);
    if let Some(handler_err) = e.downcast_ref::<HandlerError>() {
        return resolve_handler_error(answer_id, handler_err);
    }

    JsonRpcResponse::error(
        answer_id,
        JsonRpcError::new(
            JsonRpcErrorReason::ApplicationError(500),
            e.to_string(),
            json!({}),
        ),
    )
}
