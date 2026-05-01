# Real Frontend Validation Plan

## Goal

Exercise Yune's RIME ABI through real frontend lifecycle hosts before starting AI-native product work.

## Priority order

1. Native loader smoke test against the built `yune-rime-api` cdylib in a host-shaped process.
2. TypeDuck-Web browser/WebAssembly validation, because it is a real RIME-powered application frontend with a compact API wrapper and worker lifecycle.
3. Squirrel or macOS frontend validation, because the current development environment is macOS and native IME lifecycle behavior is still distinct from browser/WebAssembly behavior.
4. ibus-rime or fcitx-rime validation in a Linux environment after the macOS path is understood.
5. Benchmark harnesses for hot paths discovered during frontend validation.

## TypeDuck-Web validation target

TypeDuck-Web is a useful first real-application target because its wrapper exercises `rime_get_api`, setup, initialize, notification handling, maintenance/deploy, session creation, simulated key sequences, context/commit reads, candidate selection/deletion, page changes, levers customization, and persistent user data through an Emscripten worker and IDBFS.

It should not replace native IME host validation: browser/WebAssembly integration will not cover all Squirrel, ibus, or fcitx lifecycle, dynamic-library loading, threading, packaging, and OS input-context behavior. Treat it as the first real frontend-shaped integration before native host validation.

## Validation scenarios

- Load the dynamic library and resolve `rime_get_api` from the real frontend process shape.
- Run setup, initialize, deploy, select schema, create session, process key sequences, read context/status, commit text, and destroy session.
- Exercise repeated initialize/finalize, stale sessions, notification handler replacement, schema switching, and sync/maintenance tasks.
- Compare observed frontend calls and failure modes against the CLI surrogate and existing dynamic-loader tests.

## Benchmark scenarios

- Session create/destroy latency.
- Per-key `RimeProcessKey` latency for simple ASCII, schema-loaded table lookup, punctuation, paging, and selection paths.
- Schema deployment and dictionary load latency for representative schemas.
- Userdb learning, backup, restore, and sync latency with growing record counts.

## Outputs

- Reproducible frontend validation notes or fixtures.
- Focused regression tests for any observed ABI/runtime mismatch.
- Benchmark baselines for frontend-sensitive paths.
- A go/no-go decision for beginning AI-native candidate/ranking design.
