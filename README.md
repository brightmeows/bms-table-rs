# BMS Difficulty Table Fetching and Parsing Library

[![Crates.io](https://img.shields.io/crates/v/bms-table)](https://crates.io/crates/bms-table)
[![Documentation](https://docs.rs/bms-table/badge.svg)](https://docs.rs/bms-table)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/MiyakoMeow/bms-table-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/MiyakoMeow/bms-table-rs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/MiyakoMeow/bms-table-rs/graph/badge.svg)](https://codecov.io/gh/MiyakoMeow/bms-table-rs)
[![Cargo Deny](https://github.com/MiyakoMeow/bms-table-rs/actions/workflows/cargo-deny.yml/badge.svg)](https://github.com/MiyakoMeow/bms-table-rs/actions/workflows/cargo-deny.yml)
[![Downloads](https://img.shields.io/crates/d/bms-table)](https://crates.io/crates/bms-table)

A Rust library to fetch and parse BMS difficulty tables. It can build a complete data structure from a web page or a header JSON, covering the header, courses, trophies and chart items, and it provides APIs to fetch lists of tables.

## Features

- Extract the header JSON URL from HTML `<meta name="bmstable">` (requires `scraper`).
- Parse the header JSON into `BmsTableHeader`; unrecognized fields are preserved in `extra`.
- Parse chart data into `BmsTableData`, supporting a plain array of `ChartItem` structure.
- Automatically convert `md5`/`sha256` lists in courses to `ChartItem`; when `level` is missing, fill with "0".
- One-stop network fetching APIs (enable `reqwest`, which implicitly enables `scraper`).
- Fetch a list of difficulty tables.

## Feature Flags

- `serde`: serialization/deserialization support (enabled by default).
- `scraper`: HTML parsing and bmstable header URL extraction (enabled by default; implicitly enabled by `reqwest`).
- `reqwest`: network fetching implementation (enabled by default; requires the `tokio` runtime).

## API Overview

- `BmsTable`: top-level data structure containing `header` and `data`.
- `BmsTableHeader`: header metadata; unrecognized fields are preserved in `extra`.
- `BmsTableData`: chart data as an array.
- `CourseInfo`: course information; supports automatically converting `md5`/`sha256` lists to chart items.
- `ChartItem`: a chart item; empty strings are deserialized as `None`.
- `Trophy`: trophy requirements (max miss rate, minimum score rate).
- `fetch::reqwest::Fetcher`: one-stop network fetcher wrapper around a reusable `reqwest::Client`.
- `Fetcher::fetch_table(url)`: fetch and parse a complete table from a web page or a header JSON source.
- `Fetcher::fetch_table_with_raw(url)`: return both the parsed table and the original header/data JSON texts.
- `Fetcher::fetch_table_list(url)`: fetch a list of difficulty tables.
- `Fetcher::fetch_table_list_with_raw(url)`: return the list items along with the original JSON text.
- `fetch::get_web_header_json_value(str)`: parse a response string into header JSON or its URL (`HeaderQueryContent`).
- `fetch::extract_bmstable_url(html)`: extract the bmstable header URL from HTML.

## Examples

- `examples/single_fetch_list.rs`: fetch the table list once and print the first few items.
- `examples/multi_fetch.rs`: concurrently fetch multiple tables and print progress and results.

## Docs & Links

- `docs.rs`: https://docs.rs/bms-table
- Repository: https://github.com/MiyakoMeow/bms-table-rs

## License

Licensed under the Apache-2.0 license.
