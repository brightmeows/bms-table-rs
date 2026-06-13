//! Deserialization implementation module
//!
//! Centralizes all `Deserialize` implementations and helper raw types here, keeping `lib.rs` focused on type definitions.

use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::BTreeMap;

use crate::{ChartItem, CourseInfo, Trophy};

/// Field-level deserialization: converts `level_order` numbers or strings to strings,
/// uses `to_string()` for other types, and returns an empty array by default.
pub(crate) fn deserialize_level_order<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let values = Option::<Vec<Value>>::deserialize(deserializer)?.unwrap_or_default();
    Ok(values
        .into_iter()
        .map(|v| match v {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s,
            other => other.to_string(),
        })
        .collect())
}

/// Internal helper type: used to construct `CourseInfo` more simply and handle md5/sha256 lists.
#[derive(Deserialize)]
pub(crate) struct CourseInfoRaw {
    /// Course name
    name: String,
    /// Constraint list
    #[serde(default)]
    constraint: Vec<String>,
    /// Trophy list
    #[serde(default)]
    trophy: Vec<Trophy>,
    /// MD5 list converted into chart items
    #[serde(default, rename = "md5")]
    md5list: Vec<String>,
    /// SHA256 list converted into chart items
    #[serde(default, rename = "sha256")]
    sha256list: Vec<String>,
    /// Raw chart objects (filled with default level if missing)
    #[serde(default)]
    charts: Vec<Value>,
}

impl TryFrom<CourseInfoRaw> for CourseInfo {
    type Error = serde_json::Error;

    fn try_from(raw: CourseInfoRaw) -> Result<Self, Self::Error> {
        let mut charts: Vec<ChartItem> =
            Vec::with_capacity(raw.charts.len() + raw.md5list.len() + raw.sha256list.len());

        // Deserialize raw chart values directly — missing or null `level` is
        // handled by `de_numstring` (returns `""`), so no explicit default needed.
        for chart_value in raw.charts {
            let item: ChartItem = serde_json::from_value(chart_value)?;
            charts.push(item);
        }

        // md5list -> charts (level defaults to empty string)
        charts.extend(raw.md5list.into_iter().map(|md5| ChartItem {
            level: String::new(),
            md5: Some(md5),
            sha256: None,
            title: None,
            artist: None,
            url: None,
            url_diff: None,
            comment: None,
            extra: BTreeMap::new(),
        }));

        // sha256list -> charts (level defaults to empty string)
        charts.extend(raw.sha256list.into_iter().map(|sha256| ChartItem {
            level: String::new(),
            md5: None,
            sha256: Some(sha256),
            title: None,
            artist: None,
            url: None,
            url_diff: None,
            comment: None,
            extra: BTreeMap::new(),
        }));

        Ok(Self {
            name: raw.name,
            constraint: raw.constraint,
            trophy: raw.trophy,
            charts,
        })
    }
}

/// Deserializes a value into a `String`, accepting strings and numbers.
///
/// `null` is treated as missing (returns empty string) for leniency.
pub(crate) fn de_numstring<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<Value>::deserialize(deserializer)?;
    let Some(value) = opt else {
        return Ok(String::new());
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
