//! RFC 9457 problem responses mapped from service errors (`API_SPEC.md` §15.3).

use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use sdkwork_missory_contract::error::{MissoryServiceError, MissoryServiceErrorKind};
use serde_json::json;

/// Numeric problem code carried by every error body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProblemCode(pub i32);

/// An SDKWork problem response: `application/problem+json` with numeric `code`
/// and a server-minted `traceId`.
#[derive(Debug, Clone)]
pub struct ApiProblem {
    /// HTTP status.
    pub status: StatusCode,
    /// Numeric problem code (`API_SPEC.md` §15.3).
    pub code: i32,
    /// Short title.
    pub title: String,
    /// Client-safe detail.
    pub detail: String,
}

impl ApiProblem {
    /// 401 problem for a missing request context.
    pub fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: 40101,
            title: "Authentication Required".to_string(),
            detail: "missing Missory request context".to_string(),
        }
    }

    /// 400 problem for malformed JSON bodies.
    pub fn invalid_body(detail: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: 40002,
            title: "Malformed Request".to_string(),
            detail,
        }
    }

    /// Builds the response body with a fresh server-minted trace id.
    fn body(&self) -> serde_json::Value {
        json!({
            "type": "about:blank",
            "title": self.title,
            "status": self.status.as_u16(),
            "detail": self.detail,
            "code": self.code,
            "traceId": uuid::Uuid::new_v4().to_string(),
        })
    }
}

impl IntoResponse for ApiProblem {
    fn into_response(self) -> Response {
        let mut response = (self.status, Json(self.body())).into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json"),
        );
        response
    }
}

/// Maps a contract error onto the problem envelope.
impl From<MissoryServiceError> for ApiProblem {
    fn from(error: MissoryServiceError) -> Self {
        let (status, code, title) = match error.kind {
            MissoryServiceErrorKind::NotFound => (StatusCode::NOT_FOUND, 40401, "Not Found"),
            MissoryServiceErrorKind::Conflict => (StatusCode::CONFLICT, 40901, "Conflict"),
            MissoryServiceErrorKind::Validation => {
                if error.code == "invalid_parameter" {
                    (StatusCode::BAD_REQUEST, 40003, "Invalid Parameter")
                } else {
                    (StatusCode::BAD_REQUEST, 40001, "Validation Error")
                }
            }
            MissoryServiceErrorKind::Forbidden => {
                (StatusCode::FORBIDDEN, 40301, "Permission Required")
            }
            MissoryServiceErrorKind::Storage => {
                (StatusCode::INTERNAL_SERVER_ERROR, 50001, "Internal Error")
            }
            MissoryServiceErrorKind::NotImplemented => {
                (StatusCode::NOT_IMPLEMENTED, 50001, "Not Implemented")
            }
        };
        Self {
            status,
            code,
            title: title.to_string(),
            detail: error.detail,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_not_found_error_when_mapping_then_404_problem_is_built() {
        let problem: ApiProblem = MissoryServiceError::not_found("person", 7).into();
        assert_eq!(problem.status, StatusCode::NOT_FOUND);
        assert_eq!(problem.code, 40401);
    }

    #[test]
    fn given_invalid_parameter_when_mapping_then_40003_problem_is_built() {
        let problem: ApiProblem = MissoryServiceError::invalid_parameter("bad").into();
        assert_eq!(problem.status, StatusCode::BAD_REQUEST);
        assert_eq!(problem.code, 40003);
    }
}
