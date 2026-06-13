//! Deserialization helpers module
//!
//! Centralizes field-level `Deserialize` helpers for `level` and `level_order`,
//! keeping `lib.rs` focused on type definitions.

use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Field-level deserialization: converts `level_order` entries to strings,
/// accepting only strings and numbers; returns an error for any other type.
/// Returns an empty array by default.
pub(crate) fn deserialize_level_order<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let values = Option::<Vec<Value>>::deserialize(deserializer)?.unwrap_or_default();
    values
        .into_iter()
        .map(|v| match v {
            Value::Number(n) => Ok(n.to_string()),
            Value::String(s) => Ok(s),
            other => Err(serde::de::Error::custom(format!(
                "expected string or number for level_order entry, got {other}"
            ))),
        })
        .collect()
}

/// Default level value per the BMS difficulty table spec.
///
/// Used when `level` is absent or `null` in chart data and
/// md5/sha256 shorthand lists.
pub(crate) fn default_level() -> String {
    "0".to_string()
}

/// Deserializes a value into a `String`, accepting strings and numbers.
///
/// `null` is treated as missing and returns the spec default `"0"`.
pub(crate) fn de_numstring<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<Value>::deserialize(deserializer)?;
    let Some(value) = opt else {
        return Ok(default_level());
    };
    match value {
        Value::String(s) => Ok(s),
        Value::Number(n) => Ok(n.to_string()),
        other => Err(serde::de::Error::custom(format!(
            "expected string or number, got {}",
            other
        ))),
    }
}
