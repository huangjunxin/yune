# Repository Guide

**Yune** is a Rust input-method engine that uses **librime as a compatibility oracle** while building toward an AI-native input engine librime cannot provide. It has a deterministic core (`yune-core`), a librime-shaped C ABI (`yune-rime-api`), a typed schema-subset parser (`yune-schema`), a CLI surrogate (`yune-cli`), and a TypeScript browser runtime (`packages/yune-typeduck-runtime`).

**Core value:** existing RIME schemas and frontends should behave predictably through Yune, with every compatibility difference measured against librime before it is accepted.

## Canonical docs — read these

- **[docs/CONVENTIONS.md](docs/CONVENTIONS.md) — start here.** The single reference for architecture, stack, repo structure, coding & testing conventions, C ABI rules, integrations, and current risks.
- [docs/roadmap.md](docs/roadmap.md) — milestones and what's next (current direction: **web-first**).
- [docs/decisions.md](docs/decisions.md) — the decision log (standing principles + `D-*` entries).
- [docs/requirements.md](docs/requirements.md) — requirement IDs and their status.
- [docs/plans/](docs/plans/) — per-stage execution plans (active at the top, finished ones under `plans/archive/`).

## Key constraints

- **Compatibility oracle:** upstream <https://github.com/rime/librime>, plus the TypeDuck fork <https://github.com/TypeDuck-HK/librime> @ `v1.1.2` for Windows-specific behavior. It is *not* a local checkout path.
- **Idiomatic Rust over a C++ clone:** preserve librime-*observable* behavior at the ABI boundary; keep internals clean, typed Rust.
- **Own each slice:** new behavior gets an owning module *and* owning tests; keep `lib.rs`/`main.rs` as facades.
- **C ABI:** `RimeApi` field order *is* the ABI — append new function-table entries at the exact position they occupy in the fork's `rime_api.h`, never mid-struct.
- **Tests are oracle-driven and non-circular:** capture expected bytes from the oracle, run the real path, never derive the expected value from Yune itself. Uncaptured cases use `#[ignore = "blocked: …"]` with a `panic!()` body — no silent gaps.
- **Security:** runtime resource identifiers are logical IDs, not arbitrary filesystem paths.

## Quality gate

`cargo fmt` · `cargo clippy --workspace --all-targets -- -D warnings` · focused tests · `cargo test --workspace` when shared behavior changes. For the TypeScript package: `npm --prefix packages/yune-typeduck-runtime test` and `… run build`.

(The GSD planning system has been retired; planning/decisions/conventions now live under `docs/`, not `.planning/`.)
