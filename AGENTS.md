# bms-table

## Commands

### Pre-commit (auto on commit)

```bash
pre-commit run --all-files --quiet    # manually trigger all hooks at once
```

Hooks configured: `cargo fmt --check`, `cargo clippy --quiet`, `cargo doc --no-deps --quiet`.

### CI / manual only

```bash
cargo test --quiet
cargo deny check
```

## Crate

| Crate | Directory | Summary |
|---|---|---|
| `bms-table` | `./src` | BMS difficulty table parser & fetcher |

All dependencies defined in `[dependencies]` in root `Cargo.toml`.

## Commit format

Conventional Commits matching `release-plz.toml` changelog groups:
`feat:` / `fix:` / `refactor:` / `perf:` / `test:` / `docs:` / `ci:` / `security:` / `deprecated:` / `revert:`

- Title/body in English.
- Use `()` for scope, e.g. `feat(bms-parser):`.
- Use `!` for BREAKING CHANGE, e.g. `feat!:` or `feat(scope)!:`.

## Comment style

- Use doc comments (`///` for items, `//!` for modules) for all API
  documentation — clippy enforces docs on all items.

## MSRV

- Minimum Rust version: **1.85** (derived from edition 2024).

## Testing

- Test naming: `<scenario>_<expectation>`.
- One assertion per test. Prefer testing edge cases through public types
  over internal `Wrap` structs.
