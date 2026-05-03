# BMS Table RS 架构优化设计

**日期**: 2026-05-04
**状态**: 已批准

## 目标

优化代码组织、类型安全、错误语义。

## 改动点

### 1. 类型重组

| 类型 | 原位置 | 新位置 |
|------|--------|--------|
| `BmsTableRaw` | lib.rs | fetch.rs |
| `FetchedTable` | fetch.rs | fetch.rs（保留） |
| `FetchedTableList` | fetch.rs | fetch.rs（保留） |

**原则**：fetch 过程元数据就近放置。

### 2. 新增专用错误枚举

```rust
// src/fetch.rs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("parse error: {context}")]
    Parse { context: String },

    #[error("validation error: {field} - {reason}")]
    Validation { field: &'static str, reason: String },

    #[error("missing required field: {0}")]
    MissingField(&'static str),

    #[error("cycle detected in header resolution")]
    CyclicHeader,
}
```

所有 `anyhow::Result<T>` 在 fetch 模块内替换为 `Result<T, Error>`。

### 3. ChartItem 反序列化增强

`ChartItem` 反序列化时支持 md5/sha256 列表自动转为 chart items：

```json
{ "md5": ["abc123", "def456"] }
```

→ 两个 `ChartItem`，分别带 `md5: Some("abc123")` 和 `md5: Some("def456")`。

### 4. 特征标志重组

```toml
[features]
default = ["serde", "scraper", "reqwest"]
serde = ["dep:serde", "dep:serde_json", "dep:thiserror"]
scraper = ["serde", "dep:scraper", "dep:url"]
reqwest = ["scraper", "dep:reqwest"]
```

显式声明依赖关系。

### 5. BmsTableHeader 验证

解析时检查 `data_url` 非空。

## 文件变更

| 文件 | 变更 |
|------|------|
| src/lib.rs | 移除 `BmsTableRaw`，保留核心类型 |
| src/de.rs | 新增 `ChartItem` 的 `TryFrom<Value>` 实现 |
| src/fetch.rs | 新增 `Error` 枚举，移动 `BmsTableRaw`，替换 `Result` 类型 |
| src/fetch/reqwest.rs | 适配新错误类型 |
| Cargo.toml | 新增 `thiserror` 依赖 |

## 兼容性

破坏性变更。调用方需更新错误处理代码。
