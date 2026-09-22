# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.15.0](https://github.com/brightmeows/bms-table-rs/compare/v0.14.0...v0.15.0) - 2026-09-15

### Bug Fixes

- *(ci)* add --all-targets to clippy pre-commit hook
- remove unsupported --quiet flag from AGENTS.md pre-commit command
- *(de)* correct stale comment about level default value
- [**breaking**] default missing/null level to "0" per BMS table spec
- remove unreachable Value::Null arm in de_numstring
- wrap release-plz config keys in [workspace] section
- remove invalid [package] section from release-plz.toml

### CI

- ignore the AI reviewer check in the auto-merge gate ([#82](https://github.com/brightmeows/bms-table-rs/pull/82))
- run the mirror cron daily instead of weekly ([#80](https://github.com/brightmeows/bms-table-rs/pull/80))
- add Codeberg mirror sync workflow
- restore GitHub Actions suite
- use codeberg-small runner for release workflow
- install cargo-semver-checks in release-plz workflow
- document cargo-binstall dependency assumption in release-plz workflow
- *(release-plz)* auto-trigger release on push with version skip check
- *(release-plz)* redirect temp compilation to cached target subdirs
- *(release-plz)* unify cache keys across both workflows
- *(release-plz)* add Rust version to target cache key
- *(release-plz)* split cargo cache into registry and target steps
- *(release-plz)* add cargo cache to release-pr workflow, merge install+run steps
- sync pre-commit config from bmsrs
- use rust-latest tag instead of rust-24.04
- bump PR workflow runner from codeberg-tiny to codeberg-small
- switch image from ghcr.io to docker.io for cache stability
- add pre-commit hooks and AGENTS.md from bmsrs
- add release-plz config and Codeberg CI workflows

### Documentation

- point repository metadata back to GitHub
- add Changelog policy note to AGENTS.md
- *(example)* add missing doc comments in multi_fetch
- remove phantom v1.0.0 entry from CHANGELOG
- fix stale method name in readmes
- migrate badges and links from GitHub to Codeberg

### Features

- add ChartItem::empty() const constructor
- add Default impls for ChartItem and BmsTableData, Trophy::new
- *(header)* add effective_tag for spec-compliant tag fallback
- add BmsTableInfo::new constructor
- [**breaking**] promote ChartItem spec fields, add level_index, into_flatten, rename listes

### Other

- *(cargo)* allow multiple_crate_versions ([#81](https://github.com/brightmeows/bms-table-rs/pull/81))
- *(deps)* bump reqwest from 0.13.4 to 0.13.5 ([#76](https://github.com/brightmeows/bms-table-rs/pull/76))
- *(deps)* bump serde_json from 1.0.150 to 1.0.151 ([#75](https://github.com/brightmeows/bms-table-rs/pull/75))
- *(deps)* bump anyhow from 1.0.102 to 1.0.104 ([#77](https://github.com/brightmeows/bms-table-rs/pull/77))
- *(deps)* bump serde from 1.0.228 to 1.0.229 ([#79](https://github.com/brightmeows/bms-table-rs/pull/79))
- *(deps)* bump tokio from 1.52.3 to 1.53.1 ([#78](https://github.com/brightmeows/bms-table-rs/pull/78))
- release v0.14.0
- remove unused license allowances, document level_index O(n)
- ignore .worktrees/
- release v0.13.0
- *(deps)* remove unused tokio test-util feature, enable sync explicitly
- *(deps)* bump scraper to 0.27, clean deny.toml
- *(deps)* bump poseidon/wait-for-status-checks from 0.6.0 to 0.7.0 ([#74](https://github.com/brightmeows/bms-table-rs/pull/74))
- *(deps)* bump actions/checkout from 6 to 7 ([#73](https://github.com/brightmeows/bms-table-rs/pull/73))

### Performance

- switch CI container image from docker.io to ghcr.io

### Refactoring

- [**breaking**] change BmsTableData/BmsTableList to newtypes with Deref
- *(examples)* make examples self-contained, remove shared module
- mark all public types as #[non_exhaustive]
- unify optional ChartItem field serialization
- *(lints)* adopt bmsrs-style lint config with pedantic group
- move all lint configuration to Cargo.toml
- [**breaking**] rename into_flatten to into_flattened, fix test warnings
- *(de)* rename de_numstring to deserialize_level for naming consistency
- *(course-info)* [**breaking**] preserve md5/sha256/extra fields, remove CourseInfoRaw
- *(example)* remove unnecessary empty-string filter
- clean up examples and README clarity
- code quality improvements across the board
- *(de)* remove Value clone in CourseInfoRaw, unify level default to ""
- [**breaking**] replace scraper with htmlparser, return &str from extract_url
- [**breaking**] remove url_pack/name_pack/org_md5/mode from ChartItem, replace anyhow with thiserror
- *(de)* replace String error type with serde_json::Error in CourseInfoRaw TryFrom
- *(header)* keep only new constructor on BmsTableHeader
- *(course-group)* rename Flat/Nested to Courses/SubGroups, change course to CourseGroup
- rename BmsTableHtml::try_extract_bmstable_from_html to extract_url
- *(examples)* remove dead code and suppress cross-example warnings
- clean up module visibility, add constructors, simplify CourseInfo deser
- refactor!(header): replace Vec<Vec<CourseInfo>> with recursive Vec<CourseGroup>
- *(api)* add tag/mode to header, drop subtitle/subartist from chart
- [**breaking**] remove fetcher, flatten crate, drop feature flags

### Testing

- rename tests to follow <scenario>_<expectation> convention

## [0.14.0](https://codeberg.org/brightmeows/bms-table-rs/compare/v0.13.0...v0.14.0) - 2026-06-15

### Bug Fixes

- *(ci)* add --all-targets to clippy pre-commit hook
- remove unsupported --quiet flag from AGENTS.md pre-commit command
- *(de)* correct stale comment about level default value
- [**breaking**] default missing/null level to "0" per BMS table spec
- remove unreachable Value::Null arm in de_numstring

### CI

- document cargo-binstall dependency assumption in release-plz workflow

### Documentation

- add Changelog policy note to AGENTS.md
- *(example)* add missing doc comments in multi_fetch
- remove phantom v1.0.0 entry from CHANGELOG

### Features

- add ChartItem::empty() const constructor
- add Default impls for ChartItem and BmsTableData, Trophy::new
- *(header)* add effective_tag for spec-compliant tag fallback
- add BmsTableInfo::new constructor
- [**breaking**] promote ChartItem spec fields, add level_index, into_flatten, rename listes

### Other

- remove unused license allowances, document level_index O(n)

### Refactoring

- [**breaking**] change BmsTableData/BmsTableList to newtypes with Deref
- *(examples)* make examples self-contained, remove shared module
- mark all public types as #[non_exhaustive]
- unify optional ChartItem field serialization
- *(lints)* adopt bmsrs-style lint config with pedantic group
- move all lint configuration to Cargo.toml
- [**breaking**] rename into_flatten to into_flattened, fix test warnings
- *(de)* rename de_numstring to deserialize_level for naming consistency
- *(course-info)* [**breaking**] preserve md5/sha256/extra fields, remove CourseInfoRaw
- *(example)* remove unnecessary empty-string filter
- clean up examples and README clarity
- code quality improvements across the board
- *(de)* remove Value clone in CourseInfoRaw, unify level default to ""
- [**breaking**] replace scraper with htmlparser, return &str from extract_url
- [**breaking**] remove url_pack/name_pack/org_md5/mode from ChartItem, replace anyhow with thiserror
- *(de)* replace String error type with serde_json::Error in CourseInfoRaw TryFrom

### Testing

- rename tests to follow <scenario>_<expectation> convention

## [0.13.0](https://codeberg.org/brightmeows/bms-table-rs/compare/v0.12.1...v0.13.0) - 2026-06-13

### Bug Fixes

- wrap release-plz config keys in [workspace] section
- remove invalid [package] section from release-plz.toml
- doc error ([#58](https://codeberg.org/brightmeows/bms-table-rs/pulls/58))

### CI

- add pre-commit hooks and AGENTS.md from bmsrs
- add release-plz config and Codeberg CI workflows
- add coverage workflow and unify toolchain versions ([#65](https://codeberg.org/brightmeows/bms-table-rs/pulls/65))

### Documentation

- fix stale method name in readmes
- migrate badges and links from GitHub to Codeberg

### Other

- *(deps)* remove unused tokio test-util feature, enable sync explicitly
- *(deps)* bump scraper to 0.27, clean deny.toml
- *(deps)* bump reqwest from 0.13.3 to 0.13.4 ([#72](https://codeberg.org/brightmeows/bms-table-rs/pulls/72))
- *(deps)* bump serde_json from 1.0.149 to 1.0.150 ([#71](https://codeberg.org/brightmeows/bms-table-rs/pulls/71))
- *(deps)* bump tokio from 1.52.2 to 1.52.3 ([#69](https://codeberg.org/brightmeows/bms-table-rs/pulls/69))
- *(deps)* bump tokio from 1.52.1 to 1.52.2 ([#68](https://codeberg.org/brightmeows/bms-table-rs/pulls/68))
- *(deps)* bump reqwest from 0.13.2 to 0.13.3 ([#64](https://codeberg.org/brightmeows/bms-table-rs/pulls/64))
- *(deps)* bump tokio from 1.52.0 to 1.52.1 ([#63](https://codeberg.org/brightmeows/bms-table-rs/pulls/63))
- *(deps)* bump tokio from 1.51.1 to 1.52.0 ([#62](https://codeberg.org/brightmeows/bms-table-rs/pulls/62))
- *(deps)* bump tokio from 1.51.0 to 1.51.1 ([#61](https://codeberg.org/brightmeows/bms-table-rs/pulls/61))
- *(deps)* bump tokio from 1.50.0 to 1.51.0 ([#60](https://codeberg.org/brightmeows/bms-table-rs/pulls/60))
- *(deps)* bump scraper from 0.25.0 to 0.26.0 ([#59](https://codeberg.org/brightmeows/bms-table-rs/pulls/59))
- *(deps)* Update Rust crate tokio to v1.50.0 ([#55](https://codeberg.org/brightmeows/bms-table-rs/pulls/55))
- *(deps)* Update Rust crate anyhow to v1.0.102 ([#54](https://codeberg.org/brightmeows/bms-table-rs/pulls/54))
- *(deps)* Update Rust crate reqwest to v0.13.2 ([#53](https://codeberg.org/brightmeows/bms-table-rs/pulls/53))
- *(deps)* Update Rust crate anyhow to v1.0.101 ([#52](https://codeberg.org/brightmeows/bms-table-rs/pulls/52))

### Refactoring

- *(header)* keep only new constructor on BmsTableHeader
- *(course-group)* rename Flat/Nested to Courses/SubGroups, change course to CourseGroup
- rename BmsTableHtml::try_extract_bmstable_from_html to extract_url
- *(examples)* remove dead code and suppress cross-example warnings
- clean up module visibility, add constructors, simplify CourseInfo deser
- refactor!(header): replace Vec<Vec<CourseInfo>> with recursive Vec<CourseGroup>
- *(api)* add tag/mode to header, drop subtitle/subartist from chart
- [**breaking**] remove fetcher, flatten crate, drop feature flags
## [0.12.1](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.12.0...v0.12.1) - 2026-01-20


### Chore

- update deps ([#49](https://github.com/MiyakoMeow/bms-table-rs/pull/49))


### Features

- switch to release-plz ([#50](https://github.com/MiyakoMeow/bms-table-rs/pull/50))


## [0.12.0](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.11.0...v0.12.0) (2025-12-27)


### ⚠ BREAKING CHANGES

* **fetch:** renewed API ([#44](https://github.com/MiyakoMeow/bms-table-rs/issues/44))
* **reqwest:** use reqwest::IntoUrl ([#27](https://github.com/MiyakoMeow/bms-table-rs/issues/27))

### Features

* **reqwest:** use reqwest::IntoUrl ([#27](https://github.com/MiyakoMeow/bms-table-rs/issues/27)) ([2d7629a](https://github.com/MiyakoMeow/bms-table-rs/commit/2d7629a62378356da860c3bfff694cbe6ad41010))


### Bug Fixes

* add linter ([#37](https://github.com/MiyakoMeow/bms-table-rs/issues/37)) ([cf9dcf3](https://github.com/MiyakoMeow/bms-table-rs/commit/cf9dcf3c75ed02b87b589eeb692ccb7d66630977))
* clippy ([#42](https://github.com/MiyakoMeow/bms-table-rs/issues/42)) ([ebd689b](https://github.com/MiyakoMeow/bms-table-rs/commit/ebd689ba151b957d7d95d438d204406ac1f03119))
* **doc:** Option ([#38](https://github.com/MiyakoMeow/bms-table-rs/issues/38)) ([19043ad](https://github.com/MiyakoMeow/bms-table-rs/commit/19043adc40e8e2cbe344885df5f81d657435356f))
* **error:** duplicated message ([#24](https://github.com/MiyakoMeow/bms-table-rs/issues/24)) ([28e1838](https://github.com/MiyakoMeow/bms-table-rs/commit/28e18388806ce616fd8b3335187a8d58a88db703))
* **fetch:** small fix & tips ([#39](https://github.com/MiyakoMeow/bms-table-rs/issues/39)) ([8ebbe1e](https://github.com/MiyakoMeow/bms-table-rs/commit/8ebbe1e561d892b799bff37c75f33fc5e53db1e6))


### Miscellaneous Chores

* change release ([#43](https://github.com/MiyakoMeow/bms-table-rs/issues/43)) ([6548fd2](https://github.com/MiyakoMeow/bms-table-rs/commit/6548fd2726641dde1e6f5e9a7a8dbe7d92f17477))


### Code Refactoring

* **fetch:** renewed API ([#44](https://github.com/MiyakoMeow/bms-table-rs/issues/44)) ([65790da](https://github.com/MiyakoMeow/bms-table-rs/commit/65790da415a6dbb61c6f897be06869c5804341a2))

## [0.11.0](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.10.4...v0.11.0) (2025-11-16)


### ⚠ BREAKING CHANGES

* add fallback ([#23](https://github.com/MiyakoMeow/bms-table-rs/issues/23))
* index -> list ([#22](https://github.com/MiyakoMeow/bms-table-rs/issues/22))

### rename

* index -&gt; list ([#22](https://github.com/MiyakoMeow/bms-table-rs/issues/22)) ([e106dfa](https://github.com/MiyakoMeow/bms-table-rs/commit/e106dfa31fb86c816bc865fb86ad536c3641032f))


### Features

* add fallback ([#23](https://github.com/MiyakoMeow/bms-table-rs/issues/23)) ([8e9eac4](https://github.com/MiyakoMeow/bms-table-rs/commit/8e9eac4c3716a372e91fac08870626e120f6a7f1))
* **reqwest:** use serde when possible ([#20](https://github.com/MiyakoMeow/bms-table-rs/issues/20)) ([252d426](https://github.com/MiyakoMeow/bms-table-rs/commit/252d4268e6fe0ab8df027d7bf1c011ef905036d2))

## [0.10.4](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.10.3...v0.10.4) (2025-11-12)


### Bug Fixes

* **doc:** clearer describe ([#17](https://github.com/MiyakoMeow/bms-table-rs/issues/17)) ([548abee](https://github.com/MiyakoMeow/bms-table-rs/commit/548abeeec324ef29e0e028e55d6a1accb0198ef8))

## [0.10.3](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.10.2...v0.10.3) (2025-11-12)


### Bug Fixes

* **README:** accepted structure ([#15](https://github.com/MiyakoMeow/bms-table-rs/issues/15)) ([05f1709](https://github.com/MiyakoMeow/bms-table-rs/commit/05f1709764e2508c246dc1dd1f1fd1539821020c))

## [0.10.2](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.10.1...v0.10.2) (2025-11-12)


### Bug Fixes

* **test:** lib.rs offline test ([#13](https://github.com/MiyakoMeow/bms-table-rs/issues/13)) ([4cdc025](https://github.com/MiyakoMeow/bms-table-rs/commit/4cdc0257f719a322d7306f272cc37e16a6d9e013))

## [0.10.1](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.10.0...v0.10.1) (2025-11-12)


### Features

* **reqwest:** client act like a browser ([#10](https://github.com/MiyakoMeow/bms-table-rs/issues/10)) ([c130ce8](https://github.com/MiyakoMeow/bms-table-rs/commit/c130ce84acbf3dbc69597fd917ab00c6c8b2f814))


### Bug Fixes

* **ci:** renew renovate config ([#7](https://github.com/MiyakoMeow/bms-table-rs/issues/7)) ([72e47fd](https://github.com/MiyakoMeow/bms-table-rs/commit/72e47fd0d0983c803135f0a057f56a3877af05c0))
* **ci:** schedule ([fc045b7](https://github.com/MiyakoMeow/bms-table-rs/commit/fc045b70bfad8f2d150212c0f4bde862655244ad))
* **ci:** use helper bot ([#9](https://github.com/MiyakoMeow/bms-table-rs/issues/9)) ([f7345ab](https://github.com/MiyakoMeow/bms-table-rs/commit/f7345abf96dedf7aff8a21206ae88f2d42c0de6f))


### Miscellaneous Chores

* change release to 0.10.1 ([#11](https://github.com/MiyakoMeow/bms-table-rs/issues/11)) ([824b648](https://github.com/MiyakoMeow/bms-table-rs/commit/824b6482e7a4b6332001a5c99c08e3d1b9560c82))

## [0.10.0](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.9.1...v0.10.0) (2025-11-11)


### ⚠ BREAKING CHANGES

* add json_url to BmsTableRaw, feature check

### Features

* add json_url to BmsTableRaw, feature check ([7f116d3](https://github.com/MiyakoMeow/bms-table-rs/commit/7f116d3fc92614c4bb419e72781cd1f1e0402098))


### Bug Fixes

* fmt ([8cc8cf2](https://github.com/MiyakoMeow/bms-table-rs/commit/8cc8cf2ca91242ebde854381d48856b9bd021614))
* **README:** badges ([0dbbaaf](https://github.com/MiyakoMeow/bms-table-rs/commit/0dbbaaf35c48c48fe211d7d54e49d4d575690f1e))
* **README:** badges ([771f5a6](https://github.com/MiyakoMeow/bms-table-rs/commit/771f5a645f1ee12dc88cbb8c7033865c118f3ed9))
* **README:** badges style ([fdbed95](https://github.com/MiyakoMeow/bms-table-rs/commit/fdbed954fe69ec2641742f9a4220a47ab3ea36f9))


### Miscellaneous Chores

* change release to 0.10.0 ([7079781](https://github.com/MiyakoMeow/bms-table-rs/commit/7079781d85f77c30e11efe4ae854394324718e65))

## [0.9.1](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.9.0...v0.9.1) (2025-11-10)


### Features

* **reqwest:** make client more lenient ([dbed960](https://github.com/MiyakoMeow/bms-table-rs/commit/dbed960cb9f443b6e2da859a5f34ee03cda182ef))


### Bug Fixes

* **ci/publish:** spec toolchain ([03bd303](https://github.com/MiyakoMeow/bms-table-rs/commit/03bd30304034200dd3714cc942b385a0d315af89))
* **ci/release:** set type rust ([76a7f13](https://github.com/MiyakoMeow/bms-table-rs/commit/76a7f133f6b6c6af5627ab494860523e75d6fba4))


### Miscellaneous Chores

* change release to 0.9.1 ([b793e48](https://github.com/MiyakoMeow/bms-table-rs/commit/b793e48200d32859d3f6125da0a21b3b51b2db42))

## [0.9.0](https://github.com/MiyakoMeow/bms-table-rs/compare/v0.8.0...v0.9.0) (2025-11-10)


### ⚠ BREAKING CHANGES

* de_string_opt

### del

* de_string_opt ([38ba034](https://github.com/MiyakoMeow/bms-table-rs/commit/38ba03472ca7a135615e1dfd9a5e3a899aaeabdd))


### Features

* **reqwest:** clean control chars ([95ff735](https://github.com/MiyakoMeow/bms-table-rs/commit/95ff735f6650b2ef96faeda6b96cc326f45de4d2))


### Bug Fixes

* **de:** accepts number as level ([86685fd](https://github.com/MiyakoMeow/bms-table-rs/commit/86685fd2cf7bcb1beed68147e17414e1a9e146d5))
* tests ([441fbaa](https://github.com/MiyakoMeow/bms-table-rs/commit/441fbaaf70e48f195bfb8347b5153cd68bbd1601))


### Miscellaneous Chores

* release 0.9.0 ([d972bc1](https://github.com/MiyakoMeow/bms-table-rs/commit/d972bc18d6fb263cb226b754df03301966a92d3a))
