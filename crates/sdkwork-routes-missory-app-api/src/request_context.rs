//! Request-context resolution for the app-api surface.
//!
//! Phase-1 adoption note: until the `sdkwork-web-framework` / IAM dual-token
//! adapter is wired (see TECH_ARCHITECTURE.md §8), the gateway injects the
//! context as an axum extension. Handlers require the extension and never read
//! raw identity headers themselves.

use axum::Extension;
use sdkwork_missory_contract::context::MissoryRequestContext;

use crate::error::ApiProblem;

/// Header carrying the resolved tenant id in local/dev topologies.
pub const MISSORY_CONTEXT_HEADER_TENANT: &str = "x-sdkwork-tenant-id";
/// Header carrying the resolved user id in local/dev topologies.
pub const MISSORY_CONTEXT_HEADER_USER: &str = "x-sdkwork-user-id";

/// Returns the injected request context or a 401 problem.
pub fn require_app_context(
    context: Option<Extension<MissoryRequestContext>>,
) -> Result<MissoryRequestContext, ApiProblem> {
    context
        .map(|Extension(context)| context)
        .ok_or_else(ApiProblem::unauthorized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_missing_context_when_requiring_then_unauthorized_problem_is_returned() {
        let result = require_app_context(None);
        assert_eq!(result.unwrap_err().code, 40101);
    }

    #[test]
    fn given_present_context_when_requiring_then_context_is_returned() {
        let context = MissoryRequestContext::new(1, 0, 42);
        let result = require_app_context(Some(Extension(context)));
        assert_eq!(result.expect("context").user_id, 42);
    }
}
