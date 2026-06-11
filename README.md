# BMS Difficulty Table Parsing Library

[![Crates.io](https://img.shields.io/crates/v/bms-table)](https://crates.io/crates/bms-table)
[![Documentation](https://docs.rs/bms-table/badge.svg)](https://docs.rs/bms-table)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/MiyakoMeow/bms-table-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/MiyakoMeow/bms-table-rs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/MiyakoMeow/bms-table-rs/graph/badge.svg)](https://codecov.io/gh/MiyakoMeow/bms-table-rs)
[![Cargo Deny](https://github.com/MiyakoMeow/bms-table-rs/actions/workflows/cargo-deny.yml/badge.svg)](https://github.com/MiyakoMeow/bms-table-rs/actions/workflows/cargo-deny.yml)
[![Downloads](https://img.shields.io/crates/d/bms-table)](https://crates.io/crates/bms-table)

A Rust library to parse BMS difficulty tables. It can build a complete data structure from header JSON and chart data JSON, covering the header, courses, trophies and chart items.

## Features

- Extract the header JSON URL from HTML `<meta name="bmstable">`.
- Parse the header JSON into `BmsTableHeader`; unrecognized fields are preserved in `extra`.
- Parse chart data into `BmsTableData`, supporting a plain array of `ChartItem` structure.
- Automatically convert `md5`/`sha256` lists in courses to `ChartItem`; when `level` is missing, fill with `"0"`.

## API Overview

- `BmsTable`: top-level data structure containing `header` and `data`.
- `BmsTableHeader`: header metadata; unrecognized fields are preserved in `extra`.
- `BmsTableData`: chart data as an array.
- `CourseInfo`: course information; supports automatically converting `md5`/`sha256` lists to chart items.
- `ChartItem`: a chart item; empty strings are deserialized as `Some("")`.
- `Trophy`: trophy requirements (max miss rate, minimum score rate).
- `BmsTableInfo` / `BmsTableList`: data types for difficulty table list JSON.
- `BmsTableHtml`: HTML parsing operations; see `try_extract_bmstable_from_html`.

## Examples

The `examples/` directory contains network fetching examples that demonstrate how to fetch BMS difficulty tables over HTTP using `reqwest` (available as a dev-dependency):

- `examples/single_fetch.rs`: fetch a single BMS difficulty table and print summary.
- `examples/single_fetch_list.rs`: fetch the table list once and print the first few items.
- `examples/multi_fetch.rs`: concurrently fetch multiple tables and print progress and results.

## Docs & Links

- `docs.rs`: https://docs.rs/bms-table
- Repository: https://github.com/MiyakoMeow/bms-table-rs

## License

Licensed under the Apache-2.0 license.
