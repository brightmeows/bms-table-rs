//! BMS difficulty table parsing
//!
//! Provides building a complete BMS difficulty table data structure from header JSON and chart data JSON,
//! covering the header, courses, trophies, and chart items.
//! Also includes HTML parsing (via [`htmlparser`](https://docs.rs/htmlparser)) for extracting the bmstable
//! header URL from a page.
//!
//! # Features
//!
//! - Parse header JSON into [`BmsTableHeader`], preserving unrecognized fields in `extra` for forward compatibility;
//! - Parse chart data into [`BmsTableData`], supporting a plain array of [`ChartItem`] structure;
//! - Course `md5`/`sha256` shorthand lists are preserved as independent fields; use [`CourseInfo::all_charts`] for a merged view;
//! - Extract the header JSON URL from HTML `<meta name="bmstable">` (zero-copy, returns `&str`).
//!
//! # Usage
//!
//! ```rust
//! # fn main() -> Result<(), serde_json::Error> {
//! use bms_table::{BmsTable, BmsTableHeader, BmsTableData};
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

use htmlparser::{Token, Tokenizer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::de::{deserialize_level, deserialize_level_order};

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
    /// let mut header = BmsTableHeader::new(
    ///     "Test".into(),
    ///     "t".into(),
    ///     "d.json".into(),
    /// );
    /// header.level_order = vec!["1".into(), "2".into(), "3".into()];
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

impl BmsTableData {
    /// Creates a new `BmsTableData` with the given chart list.
    #[must_use]
    pub const fn new(charts: Vec<ChartItem>) -> Self {
        Self { charts }
    }
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
    #[must_use]
    pub fn flatten(&self) -> Vec<&CourseInfo> {
        match self {
            Self::Courses(v) => v.iter().collect(),
            Self::SubGroups(v) => v.iter().flat_map(CourseGroup::flatten).collect(),
        }
    }

    /// Flattens this sub-tree into owned [`CourseInfo`] values.
    #[must_use]
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
/// Describes a course's name, constraints, trophies and chart set.
/// Charts are specified via three independent fields that can coexist:
/// `charts` (full objects), `md5` (hash shorthand), and `sha256` (hash shorthand).
/// All three are preserved for round-trip fidelity.
/// Use [`CourseInfo::all_charts`] for a merged view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourseInfo {
    /// Course name, e.g. "Satellite Skill Analyzer 2nd sl0"
    pub name: String,
    /// Constraint list, e.g. ["`grade_mirror`", "`gauge_lr2`", "ln"]
    #[serde(default)]
    pub constraint: Vec<String>,
    /// List of trophies, defining requirements for different ranks
    #[serde(default)]
    pub trophy: Vec<Trophy>,
    /// Full chart objects from the `charts` JSON array
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub charts: Vec<ChartItem>,
    /// MD5 hash shorthand list, expanded by [`all_charts`](CourseInfo::all_charts) with `level = "0"`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub md5: Vec<String>,
    /// SHA256 hash shorthand list, expanded by [`all_charts`](CourseInfo::all_charts) with `level = "0"`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sha256: Vec<String>,
    /// Extra data (unrecognized fields from course JSON)
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl CourseInfo {
    /// Creates a new `CourseInfo` with the given name and all other fields empty.
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self {
            name,
            constraint: Vec::new(),
            trophy: Vec::new(),
            charts: Vec::new(),
            md5: Vec::new(),
            sha256: Vec::new(),
            extra: BTreeMap::new(),
        }
    }

    /// Merges all chart sources in order: `charts` → `md5` → `sha256`.
    ///
    /// Hash shorthand entries (`md5`, `sha256`) are expanded into [`ChartItem`]s
    /// with `level` defaulting to `"0"` per the BMS difficulty table spec.
    #[must_use]
    pub fn all_charts(&self) -> Vec<ChartItem> {
        fn from_md5(hash: String) -> ChartItem {
            ChartItem {
                md5: Some(hash),
                ..ChartItem::new(crate::de::default_level())
            }
        }
        fn from_sha256(hash: String) -> ChartItem {
            ChartItem {
                sha256: Some(hash),
                ..ChartItem::new(crate::de::default_level())
            }
        }

        let mut result = self.charts.clone();
        result.extend(self.md5.iter().cloned().map(from_md5));
        result.extend(self.sha256.iter().cloned().map(from_sha256));
        result
    }
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
    ///
    /// Defaults to `"0"` when `null` or absent in JSON, per the BMS
    /// difficulty table spec.
    #[serde(
        default = "crate::de::default_level",
        deserialize_with = "deserialize_level"
    )]
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
    ///
    /// Unlike other optional fields (`md5`, `sha256`, `title`, `artist`, `url`, `url_diff`)
    /// which serialize as `null` when absent, `comment` is skipped entirely when `None`.
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

impl BmsTableInfo {
    /// Creates a new `BmsTableInfo` with the given required fields and an empty `extra`.
    #[must_use]
    pub const fn new(name: String, symbol: String, url: url::Url) -> Self {
        Self {
            name,
            symbol,
            url,
            extra: BTreeMap::new(),
        }
    }
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

impl BmsTableList {
    /// Creates a new `BmsTableList` with the given entries.
    #[must_use]
    pub const fn new(entries: Vec<BmsTableInfo>) -> Self {
        Self { entries }
    }
}

// HTML parsing helper

/// HTML parsing for BMS difficulty tables.
///
/// Provides extraction of the header JSON URL from
/// `<meta name="bmstable" content="...">` in HTML page content.
///
/// The implementation uses the `htmlparser` zero-dependency tokenizer under
/// the hood, returning a **borrowed** slice of the original input (zero-copy).
/// Tag and attribute names are matched case-insensitively.
/// Content inside HTML comments is naturally ignored by the tokenizer.
pub struct BmsTableHtml;

impl BmsTableHtml {
    /// Extract the header JSON URL from `<meta name="bmstable" content="...">` in HTML.
    ///
    /// Returns a borrowed `&str` referencing the original input — no allocation.
    /// Scans `<meta>` tags looking for elements with `name="bmstable"` or
    /// `property="bmstable"` and reads their `content` attribute.
    ///
    /// # Errors
    ///
    /// Returns [`BmsTableError::MetaTagNotFound`] when the target tag is not
    /// found or `content` is empty.
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
    pub fn extract_url<'a>(html_content: &'a str) -> Result<&'a str, BmsTableError> {
        let mut in_meta = false;
        let mut is_bmstable = false;
        let mut content: Option<&'a str> = None;
        let mut tokenizer_error: Option<String> = None;

        for token in Tokenizer::from(html_content) {
            let token = match token {
                Ok(t) => t,
                Err(e) => {
                    if tokenizer_error.is_none() {
                        tokenizer_error = Some(e.to_string());
                    }
                    continue;
                }
            };
            match token {
                Token::ElementStart { local, .. } => {
                    in_meta = local.as_str().eq_ignore_ascii_case("meta");
                    if in_meta {
                        is_bmstable = false;
                        content = None;
                    }
                }
                Token::Attribute { local, value, .. } if in_meta => {
                    let name = local.as_str();
                    if name.eq_ignore_ascii_case("name") || name.eq_ignore_ascii_case("property") {
                        is_bmstable =
                            value.is_some_and(|v| v.as_str().eq_ignore_ascii_case("bmstable"));
                    } else if name.eq_ignore_ascii_case("content") {
                        content = value.map(|v| v.as_str());
                    }
                }
                Token::ElementEnd { .. } if in_meta => {
                    if is_bmstable
                        && let Some(c) = content
                        && !c.is_empty()
                    {
                        return Ok(c);
                    }
                    in_meta = false;
                }
                _ => {}
            }
        }

        Err(tokenizer_error
            .map(BmsTableError::TokenizerError)
            .unwrap_or(BmsTableError::MetaTagNotFound))
    }
}
