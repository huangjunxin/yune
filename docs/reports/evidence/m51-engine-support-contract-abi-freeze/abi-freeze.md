# M51 ABI Freeze

Scope: contract/ABI guard work only. No default ABI widening, browser
performance, browser memory, product, package, deployment, platform frontend, or
iOS-device claim is made.

## Guard Coverage

| Expectation | Coverage |
| --- | --- |
| `RimeApi.data_size` stays upstream-sized for default `rime_get_api()` | Covered by `crates/yune-rime-api/src/tests/abi.rs` and `crates/yune-rime-api/tests/typeduck_profile_abi_surface.rs`. |
| Default upstream `RimeApi` slot offsets stay locked | Covered by `rime_api_function_table_layout_matches_librime_header` and `assert_api_slot!` checks in `crates/yune-rime-api/src/tests/abi.rs`. |
| `RimeLeversApi.data_size` and slot offsets stay locked | Covered by `crates/yune-rime-api/src/tests/abi.rs`. |
| `RimeCandidate` remains upstream-shaped | Covered by `rime_frontend_struct_layout_matches_librime_header` in `crates/yune-rime-api/src/tests/abi.rs`; size remains three pointers: `text`, `comment`, `reserved`. |
| TypeDuck profile accessor remains opt-in and larger than default | Covered by `crates/yune-rime-api/tests/typeduck_profile_abi_surface.rs`. |
| TypeDuck fork-only list-append slots remain profile-scoped | Covered by `typeduck_profile_abi_surface.rs` and `config_api` tests, including `default_rime_api_exposes_upstream_config_list_contract`. |
| TypeDuck Windows boundary behavior remains profile-compatible | Covered by `cargo test -p yune-rime-api --test typeduck_windows_boundary`. |
| `yune_web_*` exported-symbol family stays synchronized | New test `yune_web_export_allowlist_matches_rust_anchor_and_ts_runtime` compares `scripts/yune-web-exports.txt` with the Rust `#[no_mangle]` functions, `yune_web_module.rs` linker anchor, and TypeScript runtime bindings. |

## Added Guard

`crates/yune-rime-api/tests/typeduck_profile_abi_surface.rs` now includes
`typeduck_profile_append_scalar_slots_round_trip_through_profile_table`, which
calls all four TypeDuck profile append slots through
`rime_get_typeduck_profile_api()` and verifies bool/int/double/string round-trip
through upstream config getters.

`crates/yune-rime-api/tests/yune_web.rs` now includes
`yune_web_export_allowlist_matches_rust_anchor_and_ts_runtime`.

The test enforces:

- `scripts/yune-web-exports.txt` contains exactly 14 non-empty `yune_web_*`
  symbols;
- `crates/yune-rime-api/src/web_runtime.rs` exports exactly those symbols with
  `#[no_mangle]`;
- `crates/yune-rime-api/src/bin/yune_web_module.rs` references exactly those
  symbols so Emscripten keeps them linked;
- `packages/yune-web-runtime/src/module.ts` binds exactly those symbols.

## Focused Gate Results

| Command | Result | Notes |
| --- | --- | --- |
| `cargo test -p yune-rime-api --test yune_web yune_web_export_allowlist_matches_rust_anchor_and_ts_runtime` | pass | 1 selected test passed. |
| `cargo test -p yune-rime-api --test typeduck_profile_abi_surface typeduck_profile_append_scalar_slots_round_trip_through_profile_table` | pass | 1 selected test passed. |
| `cargo test -p yune-rime-api abi` | pass | 2 unit ABI tests, 2 frontend-host ABI tests, and 4 TypeDuck boundary ABI tests matched the selector; all passed. |
| `cargo test -p yune-rime-api config_api` | pass | 22 selected tests passed. |
| `cargo test -p yune-rime-api --test yune_web yune_web_adapter_processes_keys_and_returns_json_state` | pass | 1 selected test passed. |
| `cargo test -p yune-rime-api --test typeduck_profile_abi_surface` | pass | 3 selected tests passed. |
| `cargo test -p yune-rime-api --test typeduck_windows_boundary` | pass | 4 selected tests passed. |

Verdict: ABI/export guard coverage is complete for M51 Task 2. No ABI
implementation change was needed.
