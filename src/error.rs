//! Error types for BMS table parsing.

use thiserror::Error;

/// Errors that can occur during BMS table parsing.
#[derive(Debug, Error)]
pub enum BmsTableError {
    /// The `<meta name="bmstable">` HTML tag was not found or its `content` attribute is empty.
    #[error("bmstable meta tag not found in HTML")]
    MetaTagNotFound,
}
