# bms-table

## Commands

### Pre-commit (auto on commit)

```bash
pre-commit run --all-files    # manually trigger all hooks at once
```

Hooks configured: `cargo fmt --check`, `cargo clippy --all-targets --quiet`, `cargo doc --no-deps --quiet`.

### CI / manual only

```bash
cargo test --quiet
cargo deny check
```

## Crate

| Crate | Directory | Summary |
|---|---|---|
| `bms-table` | `./src` | BMS difficulty table parser |

All dependencies defined in `[dependencies]` in root `Cargo.toml`.

## Commit format

Conventional Commits matching `release-plz.toml` changelog groups:
`feat:` / `fix:` / `refactor:` / `perf:` / `test:` / `docs:` / `ci:` / `security:` / `deprecated:` / `revert:`

- Title/body in English.
- Use `()` for scope, e.g. `feat(bms-parser):`.
- Use `!` for BREAKING CHANGE, e.g. `feat!:` or `feat(scope)!:`.

## API convention

Expose public operations through associated functions on a relevant type
(typically a domain struct or a zero-sized `*Builder`). Avoid free functions
as public API — namespacing on a type is mandatory for consistency.
Prefer a `*Builder` only when the API has configurable parameters; a
bare struct with methods is sufficient when there is no state to configure.

## Comment style

- Use doc comments (`///` for items, `//!` for modules) for all API
  documentation — clippy enforces docs on all items.

## MSRV

- Minimum Rust version: **1.85** (derived from edition 2024).

## Testing

- Test naming: `<scenario>_<expectation>`.
- One assertion per test. Prefer testing edge cases through public types
  over internal `Wrap` structs.
