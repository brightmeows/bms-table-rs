//! Error types for BMS table parsing.

use thiserror::Error;

/// Errors that can occur during BMS table parsing.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BmsTableError {
    /// The `<meta name="bmstable">` HTML tag was not found or its `content` attribute is empty.
    #[error("bmstable meta tag not found in HTML")]
    MetaTagNotFound,
    /// A JSON deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// An HTML tokenizer error (from `htmlparser`).
    #[error("HTML tokenizer error: {0}")]
    TokenizerError(String),
}
