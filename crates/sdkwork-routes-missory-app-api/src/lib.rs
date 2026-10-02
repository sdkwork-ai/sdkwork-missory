//! App API route boundary for SDKWork Missory (`/app/v3/api/missory/*`).
//!
//! Route adapters depend only on the `MissoryAppApi` contract port; business
//! rules live in the service crate and persistence in SPI adapters.

use std::sync::Arc;

use axum::Router;
use sdkwork_missory_contract::ports::MissoryAppApi;

pub mod error;
pub mod manifest;
pub mod paths;
pub mod query;
pub mod request_context;
pub mod response;
pub mod routes;

pub use error::ApiProblem;
pub use manifest::gateway_route_manifest;
pub use request_context::{
    require_app_context, MISSORY_CONTEXT_HEADER_TENANT, MISSORY_CONTEXT_HEADER_USER,
};
pub use routes::{build_router, AppState};

/// Locked app-api prefix (`API_SPEC.md` section 3).
pub const APP_API_PREFIX: &str = "/app/v3/api";

/// Builds the app-api router backed by the shared contract port.
pub fn build_router_with_app_api(api: Arc<dyn MissoryAppApi>) -> Router {
    build_router(api)
}

/// Gateway mount entrypoint used by the API assembly.
pub fn gateway_mount(api: Arc<dyn MissoryAppApi>) -> Router {
    build_router_with_app_api(api)
}
