# BMS 难度表解析库

[<img alt="codeberg" src="https://img.shields.io/badge/Codeberg-brightmeows/bms--table--rs-218b7e?logo=codeberg&logoColor=white" height="20">](https://codeberg.org/brightmeows/bms-table-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bms-table.svg?logo=rust" height="20">](https://crates.io/crates/bms-table)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bms_table-66c2a5?logo=docs.rs" height="20">](https://docs.rs/bms-table)
[<img alt="downloads" src="https://img.shields.io/crates/d/bms-table" height="20">](https://crates.io/crates/bms-table)
[<img alt="license" src="https://img.shields.io/badge/License-Apache--2.0-blue.svg" height="20">](LICENSE)

使用 Rust 实现的 BMS 难度表解析库。支持从表头 JSON 和谱面数据 JSON 构建完整数据结构，覆盖表头、段位、奖杯与谱面条目。

## 功能特性

- 从 HTML `<meta name="bmstable">` 提取表头 JSON 地址。
- 解析表头 JSON 为 `BmsTableHeader`，未识别字段保留到 `extra`。
- 解析谱面数据为 `BmsTableData`，支持 `ChartItem` 数组。
- 将段位中的 `md5`/`sha256` 列表自动转换为 `ChartItem`，`level` 缺省为 `"0"`。

## API 概览

- `BmsTable`：顶层数据结构，包含 `header` 与 `data`。
- `BmsTableHeader`：表头元数据；未识别字段保留到 `extra`。
- `BmsTableData`：谱面数据数组。
- `CourseGroup`：递归课程序列树，支持任意嵌套深度（`Courses` 叶节点 / `SubGroups` 分支节点）。
- `CourseInfo`：段位信息，支持 `md5`/`sha256` 列表自动转换为谱面。
- `ChartItem`：谱面条目；规范字段外加 `extra` 前向兼容。
- `Trophy`：奖杯要求（最大 miss 率、最低得分率）。
- `BmsTableInfo` / `BmsTableList`：难度表列表 JSON 的数据类型。
- `BmsTableHtml`：HTML 解析操作，详见 `extract_url`（返回**借用**的 `&str`，零拷贝）。

## 示例程序

`examples/` 目录下包含基于 HTTP 的网络获取示例（使用 `reqwest` 作为 dev-dependency）：

- `examples/single_fetch.rs`：单个难度表获取并打印概要。
- `examples/single_fetch_list.rs`：单次抓取难度表列表并打印前若干条目。
- `examples/multi_fetch.rs`：并发抓取多个难度表并输出进度与结果。

## 文档与链接

- `docs.rs`：https://docs.rs/bms-table
- 仓库：https://codeberg.org/brightmeows/bms-table-rs

## 许可

本项目基于 Apache-2.0 许可证开源。
