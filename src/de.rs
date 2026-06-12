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
    type Error = String;

    fn try_from(raw: CourseInfoRaw) -> Result<Self, Self::Error> {
        let mut charts: Vec<ChartItem> =
            Vec::with_capacity(raw.charts.len() + raw.md5list.len() + raw.sha256list.len());

        // Process charts and fill missing level with "0"
        for mut chart_value in raw.charts {
            if chart_value.get("level").is_none() {
                let obj = chart_value
                    .as_object()
                    .ok_or_else(|| "chart_value is not an object".to_string())?
                    .clone();
                let mut obj = obj;
                obj.insert("level".to_string(), Value::String("0".to_string()));
                chart_value = Value::Object(obj);
            }
            let item: ChartItem = serde_json::from_value(chart_value).map_err(|e| e.to_string())?;
            charts.push(item);
        }

        // md5list -> charts
        charts.extend(raw.md5list.into_iter().map(|md5| ChartItem {
            level: "0".to_string(),
            md5: Some(md5),
            sha256: None,
            title: None,
            artist: None,
            url: None,
            url_diff: None,
            extra: BTreeMap::new(),
        }));

        // sha256list -> charts
        charts.extend(raw.sha256list.into_iter().map(|sha256| ChartItem {
            level: "0".to_string(),
            md5: None,
            sha256: Some(sha256),
            title: None,
            artist: None,
            url: None,
            url_diff: None,
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

/// General helper to deserialize empty strings into `None`-like behavior.
pub(crate) fn de_numstring<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<Value>::deserialize(deserializer)?;
    let Some(value) = opt else {
        return Err(serde::de::Error::custom(
            "expected string or number, found None",
        ));
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
