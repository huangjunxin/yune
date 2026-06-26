# M43 Final Gates

Status: passed with measured memory blocker

M43 closes as a native-engine partial structural memory reduction. It does not
claim a whole-process memory win, short-key speed win, abbreviation speed win,
browser/product speed win, or TypeDuck-profile speed win.

## Selected Branch

Phase 0 selected `memory-owner-reduction`.

- Selected owner: `poet.entries_by_code`
- Phase 0 owner estimate: `38,208,541 B`
- Final owner estimate: `18,694,662 B`
- Owner drop: `19,513,879 B` (`51.072%`)

Whole-process peak did not move enough for a memory-win claim:

- Phase 0 Track A repeated peak band: `127,574,016-127,647,744 B`
- Final Track A repeated peak band: `127,492,096-127,627,264 B`
- M43 whole-process memory-win target: `<=107,797,708 B`
- Result: partial structural reduction; whole-process memory remains a measured
  blocker.

## Final Native Rows

| Row | Yune median | librime median | Ratio / result |
| --- | ---: | ---: | --- |
| startup/runtime-ready | `23,640.300 us` | `30,007.900 us` | `0.788x` |
| session create/select/destroy | `24,163.200 us` | `27,837.100 us` | `0.868x` |
| `hao` | `38.533 us` | `11.367 us` | `3.390x`; guard only |
| `ni` | `57.350 us` | `14.000 us` | `4.096x`; guard only |
| `zhongguo` | `61.075 us` | `166.162 us` | `0.368x` |
| `ceshiyixiachangjushuruxingnengzenyang` | `286.549 us` | `291.022 us` | `0.985x` |
| `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | `489.329 us` | `667.159 us` | `0.733x` |
| `cszysmsrsd` | `4,193.150 us` | `1,205.380 us` | `3.479x`; behavior guard |
| `zybfshmsru` | `4,467.740 us` | `843.120 us` | `5.299x`; behavior guard |

Track B guard:

- `neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung`
- Median: `182.780 us/op`
- p95: `191.182 us/op`
- Result: guard only; no TypeDuck-profile speed claim.

## Behavior And Storage Gates

- Final candidate-output comparison for `cszysmsrsd` and `zybfshmsru`: pass.
- Track A storage: `rsmarisa_byte_backed`.
- Table/prism mapping mode: `mmap`.
- Selected table/prism heap mirror bytes: `0`.
- `source_fallback=false`.
- First-page output and `RimeGetContext` remain page-bounded.
- M40 full-pinyin sentence lookup path remains separate from M42 abbreviation
  expansion.

## Evidence Artifacts

- Phase 0 benchmark and verdict:
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/phase-0-baseline/`
- Final benchmark:
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/`
- Final comparison CSV:
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/summary-comparison.csv`
- Final owner profile:
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/memory-owner-profile.csv`
- Final candidate output:
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/oracle-vs-yune-candidate-output.md`
- Final noise summary:
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/noise-band-summary.md`
- M43 visualizations:
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/visuals/`

## Required Commands

| Command | Result |
| --- | --- |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `git diff --check` | Pass |
| SVG XML parse and report-link check | Pass |

Focused checks also run:

- `cargo test -p yune-core upstream_sentence_model_memory_profile_accounts_packed_entries`
- `cargo test -p yune-rime-api m37_metrics_exports_snapshot_json_for_loaded_benchmarks`
- earlier focused sentence and abbreviation guards for the packed model path
  and M42 behavior path
