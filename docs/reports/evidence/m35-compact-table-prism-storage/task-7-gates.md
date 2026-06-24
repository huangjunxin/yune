# M35 Task 7 Gates

Raw final evidence:

- Native after benchmark: [`frontend-baselines-after.txt`](./frontend-baselines-after.txt)
- Fair after rerun: [`after-yune-vs-librime/summary.csv`](./after-yune-vs-librime/summary.csv)

Gates:

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | passed |
| `cargo test -p yune-core --test upstream_luna_pinyin_parity` | passed, `12` tests |
| `cargo test -p yune-core --test cantonese_parity` | passed, `37` tests |
| `cargo test -p yune-rime-api --test typeduck_web -- --test-threads=1` | passed, `29` tests in `610.39s` |
| `cargo test --workspace` | passed |
| `cargo bench -p yune-rime-api --bench frontend_baselines` | passed, raw output captured |
| `scripts\benchmark-yune-vs-librime.ps1` fair rerun | passed, output captured |
| `git diff --check` | passed |
| Runtime/browser gates | not run; no runtime, TypeScript, TypeDuck-Web source, WASM, Cloudflare, or browser-visible file changed |
