//! BMS difficulty table parsing
//!
//! Provides building a complete BMS difficulty table data structure from header JSON and chart data JSON,
//! covering the header, courses, trophies, and chart items.
//! Also includes HTML parsing for extracting the bmstable header URL from a page.
//!
//! # Features
//!
//! - Parse header JSON into [`BmsTableHeader`], preserving unrecognized fields in `extra` for forward compatibility;
//! - Parse chart data into [`BmsTableData`], supporting a plain array of [`ChartItem`] structure;
//! - Courses automatically convert `md5`/`sha256` lists into chart items, filling missing `level` with "0";
//! - Extract the header JSON URL from HTML `<meta name="bmstable">`.
//!
//! # Usage
//!
//! ```rust
//! # fn main() -> Result<(), serde_json::Error> {
//! use bms_table::{BmsTable, BmsTableHeader, BmsTableData, CourseGroup};
//!
//! let header_json = r#"{ "name": "Test", "symbol": "t", "data_url": "charts.json", "course": [], "level_order": [] }"#;
//! let data_json = r#"[]"#;
//! let header: BmsTableHeader = serde_json::from_str(header_json)?;
//! let data: BmsTableData = serde_json::from_str(data_json)?;
//! let table = BmsTable { header, data };
//! assert!(table.header.course.flatten().is_empty());
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::must_use_candidate)]
#![deny(rustdoc::broken_intra_doc_links)]

mod de;
mod error;

/// Re-export for convenience.
pub use crate::error::BmsTableError;

use std::collections::BTreeMap;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::de::{de_numstring, deserialize_level_order};

// Data types

/// Top-level BMS difficulty table data structure.
///
/// Packs header metadata and chart data together to simplify passing and use in applications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BmsTable {
    /// Header information and extra fields
    pub header: BmsTableHeader,
    /// Table data containing the chart list
    pub data: BmsTableData,
}

impl BmsTable {
    /// Creates a new `BmsTable` from its header and data.
    #[must_use]
    pub const fn new(header: BmsTableHeader, data: BmsTableData) -> Self {
        Self { header, data }
    }
}

/// BMS header information.
///
/// Strictly parses common fields and preserves unrecognized fields in `extra` for forward compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BmsTableHeader {
    /// Table name, e.g. "Satellite"
    pub name: String,
    /// Table symbol, e.g. "sl"
    pub symbol: String,
    /// URL of chart data file (preserves the original string from header JSON)
    pub data_url: String,
    /// Tag label text; falls back to `symbol` when absent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Play mode hint; same semantics as bmson's `mode_hint`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Course information, preserving the original JSON nesting shape.
    ///
    /// Supports flat arrays (`"course": [{...}]`) and arbitrarily nested arrays
    /// (`"course": [[{...}]]`, `"course": [[{...}], [{...}]]`, etc.).
    /// Deserialized as a [`CourseGroup`] tree.
    #[serde(default)]
    pub course: CourseGroup,
    /// Difficulty level order containing numbers and strings
    #[serde(default, deserialize_with = "deserialize_level_order")]
    pub level_order: Vec<String>,
    /// Extra data (unrecognized fields from header JSON)
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl BmsTableHeader {
    /// Creates a new `BmsTableHeader` with the given required fields and defaults for everything else.
    #[must_use]
    pub const fn new(name: String, symbol: String, data_url: String) -> Self {
        Self {
            name,
            symbol,
            data_url,
            tag: None,
            mode: None,
            course: CourseGroup::Courses(Vec::new()),
            level_order: Vec::new(),
            extra: BTreeMap::new(),
        }
    }

    /// Returns the index of a level in `level_order`, or `None` if not found.
    ///
    /// This is useful for comparing difficulty levels: a lower index means an easier level.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use bms_table::BmsTableHeader;
    /// let header = BmsTableHeader {
    /// #   name: "Test".into(),
    /// #   symbol: "t".into(),
    /// #   data_url: "d.json".into(),
    /// #   tag: None,
    /// #   mode: None,
    /// #   course: Default::default(),
    ///     level_order: vec!["1".into(), "2".into(), "3".into()],
    /// #   extra: Default::default(),
    /// };
    /// assert_eq!(header.level_index("2"), Some(1));
    /// assert_eq!(header.level_index("4"), None);
    /// ```
    #[must_use]
    pub fn level_index(&self, level: &str) -> Option<usize> {
        self.level_order.iter().position(|l| l == level)
    }
}

/// BMS table data.
///
/// Wraps the chart array (`[...]`). The input JSON is expected to be a plain array of [`ChartItem`]
/// objects; the `{ "charts": [...] }` wrapper form is **not** supported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BmsTableData {
    /// Charts
    pub charts: Vec<ChartItem>,
}

/// Recursive course tree supporting arbitrary nesting depth.
///
/// - [`Courses`][CourseGroup::Courses] — a leaf node containing a list of [`CourseInfo`] entries
/// - [`SubGroups`][CourseGroup::SubGroups] — a branch node containing nested sub-groups
///
/// Together with `CourseGroup` as the `course` field type, this preserves
/// the original JSON nesting shape round-trip:
///
/// | JSON | Rust |
/// |---|---|
/// | `"course": []` | `Courses(vec![])` |
/// | `"course": [{...}]` | `Courses(vec![CourseInfo])` |
/// | `"course": [[{...}]]` | `SubGroups(vec![Courses(vec![CourseInfo])])` |
/// | `"course": [[{...}], [{...}]]` | `SubGroups(vec![Courses(..), Courses(..)])` |
/// | `"course": [[[{...}]]]` | `SubGroups(vec![SubGroups(vec![Courses(..)])])` |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CourseGroup {
    /// A leaf node containing a list of course entries.
    Courses(Vec<CourseInfo>),
    /// A branch node containing nested sub-groups.
    SubGroups(Vec<CourseGroup>),
}

impl Default for CourseGroup {
    fn default() -> Self {
        Self::Courses(Vec::new())
    }
}

impl CourseGroup {
    /// Flattens all [`CourseInfo`] references in this sub-tree.
    pub fn flatten(&self) -> Vec<&CourseInfo> {
        match self {
            Self::Courses(v) => v.iter().collect(),
            Self::SubGroups(v) => v.iter().flat_map(CourseGroup::flatten).collect(),
        }
    }

    /// Flattens this sub-tree into owned [`CourseInfo`] values.
    pub fn into_flatten(self) -> Vec<CourseInfo> {
        match self {
            Self::Courses(v) => v,
            Self::SubGroups(v) => v.into_iter().flat_map(CourseGroup::into_flatten).collect(),
        }
    }
}

impl From<CourseInfo> for CourseGroup {
    fn from(info: CourseInfo) -> Self {
        Self::Courses(vec![info])
    }
}

/// Course information.
///
/// Describes a course's name, constraints, trophies and chart set. During parsing, `md5`/`sha256` lists are automatically converted into `ChartItem`s, and charts missing `level` are filled with default value `"0"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "crate::de::CourseInfoRaw")]
pub struct CourseInfo {
    /// Course name, e.g. "Satellite Skill Analyzer 2nd sl0"
    pub name: String,
    /// Constraint list, e.g. ["`grade_mirror`", "`gauge_lr2`", "ln"]
    #[serde(default)]
    pub constraint: Vec<String>,
    /// List of trophies, defining requirements for different ranks
    #[serde(default)]
    pub trophy: Vec<Trophy>,
    /// List of charts included in the course
    #[serde(default)]
    pub charts: Vec<ChartItem>,
}

/// Chart data item.
///
/// Describes metadata and resource links for a single BMS file.
///
/// Only the most commonly used spec-defined fields (`md5`, `sha256`, `level`,
/// `title`, `artist`, `url`, `url_diff`, `comment`) are exposed as first-class
/// fields. Other spec-defined optional fields (such as `name_diff`, `url_pack`,
/// `name_pack`, `org_md5`, `mode`, `ipfs`, `ipfs_diff`, `lr2_bmsid`, etc.) are
/// **not** individually promoted — they are preserved via [`extra`](ChartItem::extra)
/// for forward compatibility. This keeps the struct lean while remaining
/// fully compatible with all real-world tables.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChartItem {
    /// Difficulty level, e.g. "0"
    #[serde(default, deserialize_with = "de_numstring")]
    pub level: String,
    /// MD5 hash of the file
    pub md5: Option<String>,
    /// SHA256 hash of the file
    pub sha256: Option<String>,
    /// Song title
    pub title: Option<String>,
    /// Artist name
    pub artist: Option<String>,
    /// File download URL
    pub url: Option<String>,
    /// Differential file download URL (optional)
    pub url_diff: Option<String>,
    /// Comment text
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Extra data (unrecognized fields)
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl ChartItem {
    /// Creates a new `ChartItem` with the given level and all other fields set to defaults.
    #[must_use]
    pub const fn new(level: String) -> Self {
        Self {
            level,
            md5: None,
            sha256: None,
            title: None,
            artist: None,
            url: None,
            url_diff: None,
            comment: None,
            extra: BTreeMap::new(),
        }
    }
}

/// Trophy information.
///
/// Defines conditions to achieve specific trophies, including maximum miss rate and minimum score rate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trophy {
    /// Trophy name, e.g. "silvermedal" or "goldmedal"
    pub name: String,
    /// Maximum miss rate (percent), e.g. 5.0 means at most 5% miss rate
    pub missrate: f64,
    /// Minimum score rate (percent), e.g. 70.0 means at least 70% score rate
    pub scorerate: f64,
}

/// BMS difficulty table list item.
///
/// Represents the basic information of a difficulty table in a list. `name`, `symbol`, and `url` are
/// the core fields; other fields such as `tag1`, `tag2`, `comment`, `date`, `state`, and `tag_order`
/// are collected into [`extra`](BmsTableInfo::extra).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BmsTableInfo {
    /// Table name, e.g. ".WAS Difficulty Table"
    pub name: String,
    /// Table symbol, e.g. "．" or "\[F\]"
    pub symbol: String,
    /// Table URL
    pub url: url::Url,
    /// Extra fields collection (stores all data except required fields)
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Wrapper type for the list of BMS difficulty tables.
///
/// Transparently serialized as an array: serialization/deserialization behaves the same as the internal `Vec<BmsTableInfo>`, resulting in a JSON array rather than an object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BmsTableList {
    /// List of entries
    pub entries: Vec<BmsTableInfo>,
}

// HTML parsing helper

/// HTML parsing for BMS difficulty tables.
///
/// Provides extraction of the header JSON URL from
/// `<meta name="bmstable" content="...">` in HTML page content.
pub struct BmsTableHtml;

impl BmsTableHtml {
    /// Extract the header JSON URL from `<meta name="bmstable" content="...">` in HTML.
    ///
    /// Scans `<meta>` tags looking for elements with `name="bmstable"` or `property="bmstable"`
    /// and reads their `content` attribute.
    ///
    /// # Errors
    ///
    /// Returns [`BmsTableError::MetaTagNotFound`] when the target tag is not found or `content` is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use bms_table::BmsTableHtml;
    /// let html = r#"
    /// <!DOCTYPE html>
    /// <html>
    ///   <head>
    ///     <meta name="bmstable" content="header.json">
    ///   </head>
    ///   <body></body>
    /// </html>
    /// "#;
    /// let url = BmsTableHtml::extract_url(html).unwrap();
    /// assert_eq!(url, "header.json");
    /// ```
    pub fn extract_url(html_content: &str) -> Result<String, BmsTableError> {
        let document = Html::parse_document(html_content);
        let meta_selector =
            Selector::parse("meta").map_err(|e| BmsTableError::SelectorParse(e.to_string()))?;

        for element in document.select(&meta_selector) {
            let is_bmstable = element
                .value()
                .attr("name")
                .is_some_and(|v| v.eq_ignore_ascii_case("bmstable"))
                || element
                    .value()
                    .attr("property")
                    .is_some_and(|v| v.eq_ignore_ascii_case("bmstable"));
            if is_bmstable
                && let Some(content_attr) = element.value().attr("content")
                && !content_attr.is_empty()
            {
                return Ok(content_attr.to_string());
            }
        }

        Err(BmsTableError::MetaTagNotFound)
    }
}
