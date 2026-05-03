# BMS Table RS 架构优化实现计划

> **面向 AI 代理的工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实现此计划。步骤使用复选框（`- [ ]`）语法来跟踪进度。

**目标：** 优化代码组织、类型安全、错误语义

**架构：** 迁移 `BmsTableRaw` 至 fetch 模块，新增专用错误枚举，增强 ChartItem 反序列化

**技术栈：** Rust, thiserror, serde, reqwest

---

## 文件变更总览

| 文件 | 变更 |
|------|------|
| Cargo.toml | 新增 thiserror 依赖 |
| src/lib.rs | 移除 `BmsTableRaw`，导出 fetch::Error |
| src/de.rs | 新增 `ChartItem` 的 `TryFrom<Value>` |
| src/fetch.rs | 新增 `Error` 枚举，移动 `BmsTableRaw` |
| src/fetch/reqwest.rs | 适配新错误类型 |

---

## 任务 1：添加 thiserror 依赖

**文件：**
- 修改：`Cargo.toml`

- [ ] **步骤 1：添加 thiserror 依赖**

```toml
[dependencies]
anyhow = "1"
thiserror = { version = "2", optional = true }
serde = { version = "1", features = ["derive"], optional = true }
serde_json = { version = "1", optional = true }

scraper = { version = "0.26", optional = true }
url = { version = "2", features = ["serde"], optional = true }

reqwest = { version = "0.13", features = ["cookies"], optional = true }
```

- [ ] **步骤 2：更新 serde feature 启用 thiserror**

```toml
serde = ["dep:serde", "dep:serde_json", "dep:thiserror"]
```

- [ ] **步骤 3：Commit**

```bash
git add Cargo.toml
git commit -m "deps: add thiserror for dedicated error types"
```

---

## 任务 2：创建 Error 枚举并迁移 BmsTableRaw

**文件：**
- 修改：`src/fetch.rs`

- [ ] **步骤 1：在 fetch.rs 顶部添加 thiserror import**

在 `#![cfg(feature = "scraper")]` 后添加：
```rust
#[cfg(feature = "serde")]
use thiserror::Error;
```

- [ ] **步骤 2：添加 Error 枚举（在 `pub mod reqwest;` 之前）**

```rust
/// Fetch module error types.
#[cfg(feature = "serde")]
#[derive(Debug, Error)]
pub enum Error {
    /// Network error during fetch.
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON parse error with context.
    #[error("parse error: {context}")]
    Parse { context: String },

    /// Field validation error.
    #[error("validation error: {field} - {reason}")]
    Validation { field: &'static str, reason: String },

    /// Missing required field.
    #[error("missing required field: {0}")]
    MissingField(&'static str),

    /// Cyclic header resolution detected.
    #[error("cycle detected in header resolution")]
    CyclicHeader,
}
```

- [ ] **步骤 3：将 `BmsTableRaw` 从 lib.rs 移动到 fetch.rs（在 Error 枚举之后）**

```rust
/// Complete set of original JSON strings.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BmsTableRaw {
    /// Full URL of the header JSON
    #[cfg(feature = "scraper")]
    pub header_json_url: url::Url,
    /// Raw header JSON string
    pub header_raw: String,
    /// Full URL of the chart data JSON
    #[cfg(feature = "scraper")]
    pub data_json_url: url::Url,
    /// Raw chart data JSON string
    pub data_raw: String,
}
```

- [ ] **步骤 4：Commit**

```bash
git add src/fetch.rs
git commit -m "feat(fetch): add Error enum and move BmsTableRaw"
```

---

## 任务 3：更新 lib.rs

**文件：**
- 修改：`src/lib.rs`

- [ ] **步骤 1：移除 BmsTableRaw 定义（~lines 210-223）**

删除：
```rust
/// Complete set of original JSON strings.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BmsTableRaw {
    /// Full URL of the header JSON
    #[cfg(feature = "scraper")]
    pub header_json_url: url::Url,
    /// Raw header JSON string
    pub header_raw: String,
    /// Full URL of the chart data JSON
    #[cfg(feature = "scraper")]
    pub data_json_url: url::Url,
    /// Raw chart data JSON string
    pub data_raw: String,
}
```

- [ ] **步骤 2：更新 fetch 模块导出（在 `pub mod fetch;` 之后添加）**

```rust
pub mod fetch;
pub use fetch::Error as FetchError;
```

- [ ] **步骤 3：Commit**

```bash
git add src/lib.rs
git commit -m "refactor(lib): remove BmsTableRaw, export fetch::Error"
```

---

## 任务 4：增强 ChartItem 反序列化

**文件：**
- 修改：`src/de.rs`

- [ ] **步骤 1：添加 BTreeMap import**

```rust
use std::collections::BTreeMap;
```

- [ ] **步骤 2：在 `de_numstring` 函数后添加 ChartItemRaw 和 TryFrom 实现**

```rust
/// Internal helper type for ChartItem deserialization that supports md5/sha256 lists.
#[derive(Deserialize)]
struct ChartItemRaw {
    #[serde(default, deserialize_with = "de_numstring")]
    level: String,
    #[serde(default)]
    md5: Option<String>,
    #[serde(default)]
    sha256: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    subtitle: Option<String>,
    #[serde(default)]
    artist: Option<String>,
    #[serde(default)]
    subartist: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    url_diff: Option<String>,
    #[serde(default)]
    md5_list: Vec<String>,
    #[serde(default)]
    sha256_list: Vec<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl TryFrom<ChartItemRaw> for ChartItem {
    type Error = String;

    fn try_from(raw: ChartItemRaw) -> Result<Self, Self::Error> {
        let mut charts = Vec::with_capacity(1 + raw.md5_list.len() + raw.sha256_list.len());

        // Direct chart
        charts.push(ChartItem {
            level: raw.level,
            md5: raw.md5,
            sha256: raw.sha256,
            title: raw.title,
            subtitle: raw.subtitle,
            artist: raw.artist,
            subartist: raw.subartist,
            url: raw.url,
            url_diff: raw.url_diff,
            extra: raw.extra,
        });

        // md5_list -> charts
        charts.extend(raw.md5_list.into_iter().map(|md5| ChartItem {
            level: "0".to_string(),
            md5: Some(md5),
            sha256: None,
            title: None,
            subtitle: None,
            artist: None,
            subartist: None,
            url: None,
            url_diff: None,
            extra: BTreeMap::new(),
        }));

        // sha256_list -> charts
        charts.extend(raw.sha256_list.into_iter().map(|sha256| ChartItem {
            level: "0".to_string(),
            md5: None,
            sha256: Some(sha256),
            title: None,
            subtitle: None,
            artist: None,
            subartist: None,
            url: None,
            url_diff: None,
            extra: BTreeMap::new(),
        }));

        Ok(Self {
            level: charts[0].level.clone(),
            md5: charts[0].md5.clone(),
            sha256: charts[0].sha256.clone(),
            title: charts[0].title.clone(),
            subtitle: charts[0].subtitle.clone(),
            artist: charts[0].artist.clone(),
            subartist: charts[0].subartist.clone(),
            url: charts[0].url.clone(),
            url_diff: charts[0].url_diff.clone(),
            extra: charts[0].extra.clone(),
        })
    }
}
```

**注意：** 上述实现有问题——返回的是单个 ChartItem 但扩展的是 Vec。需要修正逻辑。

- [ ] **步骤 2（修正）：使用正确的实现方式**

```rust
/// Internal helper type for ChartItem deserialization that supports md5/sha256 lists.
#[derive(Deserialize)]
struct ChartItemRaw {
    #[serde(default, deserialize_with = "de_numstring")]
    level: String,
    #[serde(default)]
    md5: Option<String>,
    #[serde(default)]
    sha256: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    subtitle: Option<String>,
    #[serde(default)]
    artist: Option<String>,
    #[serde(default)]
    subartist: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    url_diff: Option<String>,
    #[serde(default, rename = "md5", alias = "md5_list")]
    md5_list: Vec<String>,
    #[serde(default, rename = "sha256", alias = "sha256_list")]
    sha256_list: Vec<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl TryFrom<ChartItemRaw> for ChartItem {
    type Error = String;

    fn try_from(raw: ChartItemRaw) -> Result<Self, Self::Error> {
        // Validate: if md5_list or sha256_list has entries, other fields should be empty/default
        let has_lists = !raw.md5_list.is_empty() || !raw.sha256_list.is_empty();

        // Build primary chart from raw fields (only if not purely a list container)
        let primary = if has_lists {
            ChartItem {
                level: if raw.level.is_empty() { "0".to_string() } else { raw.level },
                md5: raw.md5,
                sha256: raw.sha256,
                title: raw.title,
                subtitle: raw.subtitle,
                artist: raw.artist,
                subartist: raw.subartist,
                url: raw.url,
                url_diff: raw.url_diff,
                extra: raw.extra,
            }
        } else {
            ChartItem {
                level: raw.level,
                md5: raw.md5,
                sha256: raw.sha256,
                title: raw.title,
                subtitle: raw.subtitle,
                artist: raw.artist,
                subartist: raw.subartist,
                url: raw.url,
                url_diff: raw.url_diff,
                extra: raw.extra,
            }
        };

        Ok(primary)
    }
}

impl TryFrom<Value> for ChartItem {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let raw: ChartItemRaw = serde_json::from_value(value).map_err(|e| e.to_string())?;
        Self::try_from(raw)
    }
}
```

- [ ] **步骤 3：更新 ChartItem 的 Deserialize 实现**

找到 `impl<'de> Deserialize<'de> for ChartItem`，替换为：
```rust
impl<'de> Deserialize<'de> for ChartItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
```

- [ ] **步骤 4：Commit**

```bash
git add src/de.rs
git commit -m "feat(de): enhance ChartItem deserialization with md5/sha256 list support"
```

---

## 任务 5：更新 reqwest.rs 使用新错误类型

**文件：**
- 修改：`src/fetch/reqwest.rs`

- [ ] **步骤 1：更新 import**

```rust
use crate::{
    BmsTable, BmsTableData, BmsTableHeader, BmsTableList,
    fetch::{
        Error as FetchError, FetchedTable, FetchedTableList, HeaderQueryContent,
        TableFetcher, header_query_with_fallback, parse_json_str_with_fallback,
    },
};
```

- [ ] **步骤 2：将 `anyhow::Result` 替换为 `Result<T, FetchError>`**

在 `Fetcher` impl 块中的返回类型：
```rust
// fetch_table 返回类型
pub async fn fetch_table(&self, web_url: impl IntoUrl) -> Result<FetchedTable, FetchError>

// fetch_table_list 返回类型
pub async fn fetch_table_list(&self, web_url: impl IntoUrl) -> Result<FetchedTableList, FetchError>

// fetch_text 返回类型
async fn fetch_text(&self, url: reqwest::Url, fetch_ctx: &'static str) -> Result<String, FetchError>

// fetch_json_with_fallback 返回类型
async fn fetch_json_with_fallback<T: DeserializeOwned>(
    &self,
    url: reqwest::Url,
    fetch_ctx: &'static str,
    parse_ctx: &'static str,
) -> Result<(T, String), FetchError>
```

- [ ] **步骤 3：替换 `anyhow::Context` 为 `FetchError` 变体**

`fetch_text` 中的 `.context()` 改为：
```rust
.map_err(|e| FetchError::Network(e))
```

`fetch_table` 中的循环检测改为：
```rust
return Err(FetchError::CyclicHeader.into());
```

`header_json_url.join(&header_url_string)` 错误改为：
```rust
Err(FetchError::Validation {
    field: "data_url",
    reason: format!("failed to resolve header url: {}", e),
})
```

- [ ] **步骤 4：更新 `TableFetcher` trait 返回类型**

```rust
fn fetch_table(
    &self,
    web_url: url::Url,
) -> impl Future<Output = Result<FetchedTable, FetchError>> + Send + '_;

fn fetch_table_list(
    &self,
    web_url: url::Url,
) -> impl Future<Output = Result<FetchedTableList, FetchError>> + Send + '_;
```

- [ ] **步骤 5：Commit**

```bash
git add src/fetch/reqwest.rs
git commit -m "refactor(reqwest): use dedicated FetchError instead of anyhow"
```

---

## 任务 6：验证构建

- [ ] **步骤 1：运行 cargo check**

```bash
cargo check --all-features
```

- [ ] **步骤 2：运行测试**

```bash
cargo test --all-features
```

- [ ] **步骤 3：Commit（如果构建成功）**

```bash
git add -A
git commit -m "chore: verify build and tests pass"
```

---

## 依赖关系

任务执行顺序：**1 → 2 → 3 → 4 → 5 → 6**
