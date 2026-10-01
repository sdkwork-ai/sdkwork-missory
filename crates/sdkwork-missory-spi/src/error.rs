//! Store-layer errors.

/// Store failure kinds surfaced to the service layer.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MissoryStoreError {
    /// The requested record does not exist in the owner scope.
    #[error("record not found: {0}")]
    NotFound(String),
    /// The write violates a uniqueness or state invariant.
    #[error("conflict: {0}")]
    Conflict(String),
    /// The storage backend failed; details are backend-safe only.
    #[error("storage failure: {0}")]
    Storage(String),
}

impl MissoryStoreError {
    /// Convenience constructor for [`MissoryStoreError::NotFound`].
    pub fn not_found(what: impl Into<String>) -> Self {
        Self::NotFound(what.into())
    }

    /// Convenience constructor for [`MissoryStoreError::Conflict`].
    pub fn conflict(what: impl Into<String>) -> Self {
        Self::Conflict(what.into())
    }

    /// Convenience constructor for [`MissoryStoreError::Storage`].
    pub fn storage(what: impl Into<String>) -> Self {
        Self::Storage(what.into())
    }
}

/// Result alias for store port operations.
pub type MissoryStoreResult<T> = Result<T, MissoryStoreError>;
