//! Standalone gateway for the sdkwork-missory application HTTP plane.
//!
//! Process concerns only: fail-closed environment validation, tracing, dev
//! request-context injection, binding, and graceful shutdown.

use std::time::Duration;

use axum::extract::Request;
use axum::http::{header, HeaderValue, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::Json;
use sdkwork_missory_contract::context::MissoryRequestContext;

/// Environment variables governing startup.
const ENVIRONMENT_KEY: &str = "SDKWORK_MISSORY_ENVIRONMENT";
const RUNTIME_TARGET_KEY: &str = "SDKWORK_MISSORY_RUNTIME_TARGET";
const BIND_KEY: &str = "SDKWORK_MISSORY_APPLICATION_PUBLIC_INGRESS_BIND";
/// Header carrying the resolved tenant id (pre-IAM local topology).
const TENANT_HEADER: &str = "x-sdkwork-tenant-id";
/// Header carrying the resolved user id (pre-IAM local topology).
const USER_HEADER: &str = "x-sdkwork-user-id";

/// Allowed environments (`CONFIG_SPEC.md` §2).
const ALLOWED_ENVIRONMENTS: [&str; 5] = ["development", "test", "staging", "demo", "production"];

fn default_bind() -> &'static str {
    "127.0.0.1:8460"
}

/// Resolves and validates the startup environment; fails closed when absent.
fn resolve_environment() -> Result<String, String> {
    let runtime_target = std::env::var(RUNTIME_TARGET_KEY).unwrap_or_default();
    if runtime_target == "test-runner" {
        return Ok("test".to_string());
    }
    match std::env::var(ENVIRONMENT_KEY) {
        Ok(value) if ALLOWED_ENVIRONMENTS.contains(&value.as_str()) => Ok(value),
        Ok(other) => Err(format!(
            "{ENVIRONMENT_KEY} must be one of development|test|staging|demo|production (got {other})"
        )),
        Err(_) => Err(format!(
            "refusing to start: {ENVIRONMENT_KEY} is required (development|test|staging|demo|production)"
        )),
    }
}

fn parse_header_id(request: &Request, name: &str) -> Option<u64> {
    request
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|raw| raw.parse::<u64>().ok())
}

/// Injects the request context from tenant/user headers until the IAM adapter
/// replaces this middleware (TECH_ARCHITECTURE.md §8).
async fn inject_request_context(mut request: Request, next: Next) -> Response {
    if request
        .extensions()
        .get::<MissoryRequestContext>()
        .is_none()
    {
        let Some(user_id) = parse_header_id(&request, USER_HEADER) else {
            return problem_unauthorized();
        };
        let tenant_id = parse_header_id(&request, TENANT_HEADER).unwrap_or(1);
        request
            .extensions_mut()
            .insert(MissoryRequestContext::new(tenant_id, 0, user_id));
    }
    next.run(request).await
}

fn problem_unauthorized() -> Response {
    let body = Json(serde_json::json!({
        "type": "about:blank",
        "title": "Authentication Required",
        "status": 401,
        "detail": format!("missing {USER_HEADER} header (pre-IAM dev topology)"),
        "code": 40101,
        "traceId": uuid_like_trace(),
    }));
    let mut response = (StatusCode::UNAUTHORIZED, body).into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/problem+json"),
    );
    response
}

/// Placeholder trace minting before the shared trace middleware lands.
fn uuid_like_trace() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|delta| delta.as_millis())
        .unwrap_or_default();
    format!("gw-{millis}")
}

#[cfg(unix)]
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("install ctrl_c handler");
    };
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutdown signal received; draining connections");
    tokio::time::sleep(Duration::from_millis(50)).await;
}

#[cfg(not(unix))]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("install ctrl_c handler");
    tracing::info!("shutdown signal received; draining connections");
    tokio::time::sleep(Duration::from_millis(50)).await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
    let environment = match resolve_environment() {
        Ok(value) => value,
        Err(reason) => {
            tracing::error!("{reason}");
            std::process::exit(2);
        }
    };
    tracing::info!(environment = %environment, "starting sdkwork-missory standalone gateway");

    let router = sdkwork_api_missory_assembly::assemble_api_router_from_env()
        .expect("assemble api router")
        .layer(middleware::from_fn(inject_request_context));

    let bind = std::env::var(BIND_KEY).unwrap_or_else(|_| default_bind().to_string());
    let listener = match tokio::net::TcpListener::bind(&bind).await {
        Ok(listener) => listener,
        Err(error) => {
            tracing::error!("failed to bind {bind}: {error}");
            std::process::exit(2);
        }
    };
    tracing::info!(bind = %bind, "missory app-api listening");
    if let Err(error) = axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        tracing::error!("server error: {error}");
        std::process::exit(1);
    }
}
