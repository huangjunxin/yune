# M34 final gates

Date: 2026-06-23

## Commands run

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p yune-core bounded_
cargo test -p yune-core heap_table_lookup_exposes_exact_prefix_and_all_code_queries
cargo test -p yune-core --test upstream_luna_pinyin_parity
cargo test -p yune-core --test cantonese_parity
cargo test -p yune-rime-api --test typeduck_web
cargo test --workspace
cmd.exe /C "cargo bench -p yune-rime-api --bench frontend_baselines > docs\reports\evidence\m34-queryable-table-prism\frontend-baselines-after-final.txt 2>&1"
powershell -ExecutionPolicy Bypass -File scripts\benchmark-yune-vs-librime.ps1 -OutputRoot docs\reports\evidence\m34-queryable-table-prism\after-yune-vs-librime -Iterations 9 -SessionIterations 9 -KeyIterations 25
git diff --check
```

All commands above passed before report/archive closeout. The first
`typeduck_web` invocation used too short a tool timeout and produced no passing
or failing result; it was rerun with a longer timeout and passed. The benchmark
command emitted the existing Cargo output filename collision warning for
`yune-rime-api`, then completed successfully through `cmd.exe` redirection. The
cross-engine rerun completed and regenerated `after-yune-vs-librime`; librime
printed its known LevelDB cleanup warning after writing summaries.
`git diff --check` exited successfully and printed only Git's LF-normalization
warning for `crates/yune-rime-api/src/candidate_api.rs`.

Runtime/browser gates were not run because no runtime, WASM, TypeScript,
TypeDuck-Web source, or browser-visible files changed.
