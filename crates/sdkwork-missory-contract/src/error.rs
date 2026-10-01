//! Typed service errors for the Missory contract boundary.

use std::fmt;

/// Classification of a service failure, mapped to HTTP status/problem codes by the
/// route layer (`API_SPEC.md` section 15.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissoryServiceErrorKind {
    /// The referenced resource does not exist in the owner scope.
    NotFound,
    /// The request would violate a uniqueness or state invariant.
    Conflict,
    /// The request payload or parameters are invalid.
    Validation,
    /// The caller is not allowed to touch the target resource.
    Forbidden,
    /// The storage layer failed; details are safe for clients.
    Storage,
    /// The capability exists in the contract but is not implemented yet.
    NotImplemented,
}

impl fmt::Display for MissoryServiceErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::NotFound => "not_found",
            Self::Conflict => "conflict",
            Self::Validation => "validation",
            Self::Forbidden => "forbidden",
            Self::Storage => "storage",
            Self::NotImplemented => "not_implemented",
        };
        f.write_str(label)
    }
}

/// Typed Missory service error carrying a stable string code and client-safe detail.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("missory service error [{kind}]: {detail} (code: {code})")]
pub struct MissoryServiceError {
    /// Failure classification.
    pub kind: MissoryServiceErrorKind,
    /// Stable string error code (for example `not_found`, `invalid_parameter`).
    pub code: String,
    /// Client-safe detail message; never carries secrets or SQL.
    pub detail: String,
}

impl MissoryServiceError {
    /// Builds an error from explicit parts.
    pub fn new(
        kind: MissoryServiceErrorKind,
        code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            detail: detail.into(),
        }
    }

    /// Resource-not-found error.
    pub fn not_found(resource: &str, id: impl fmt::Display) -> Self {
        Self::new(
            MissoryServiceErrorKind::NotFound,
            "not_found",
            format!("{resource} {id} was not found"),
        )
    }

    /// Invalid parameter or payload error.
    pub fn invalid_parameter(detail: impl Into<String>) -> Self {
        Self::new(
            MissoryServiceErrorKind::Validation,
            "invalid_parameter",
            detail,
        )
    }

    /// Validation error with the canonical `validation` code.
    pub fn validation(detail: impl Into<String>) -> Self {
        Self::new(MissoryServiceErrorKind::Validation, "validation", detail)
    }

    /// Conflict error (duplicate or state invariant violation).
    pub fn conflict(detail: impl Into<String>) -> Self {
        Self::new(MissoryServiceErrorKind::Conflict, "conflict", detail)
    }

    /// Forbidden error.
    pub fn forbidden(detail: impl Into<String>) -> Self {
        Self::new(MissoryServiceErrorKind::Forbidden, "forbidden", detail)
    }

    /// Storage error with a generic client-safe detail.
    pub fn storage(detail: impl Into<String>) -> Self {
        Self::new(MissoryServiceErrorKind::Storage, "storage", detail)
    }

    /// Not-implemented error for a named capability.
    pub fn not_implemented(operation: &str) -> Self {
        Self::new(
            MissoryServiceErrorKind::NotImplemented,
            "not_implemented",
            format!("{operation} is not implemented yet"),
        )
    }
}

/// Result alias for Missory contract operations.
pub type MissoryServiceResult<T> = Result<T, MissoryServiceError>;
