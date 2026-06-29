# M48 luna_pinyin Sentence Over-Segmentation Fix Plan

> **Status:** Complete · **Milestone:** M48 (luna_pinyin sentence over-segmentation fix) · **Closed:** 2026-06-29 · **Type:** execution plan
>
> **For agentic workers:** this is oracle-driven. Capture expected bytes from the **external oracle** (upstream librime `1.17.0`, the pinned target), run Yune's real production path, assert it matches. **Never derive expected values from Yune.** The read-only diagnosis and closeout are recorded in [`../../roadmap.md`](../../roadmap.md) under "Closed Correctness Defect: luna_pinyin sentence over-segmentation"; do not re-litigate it. Steps use checkbox (`- [ ]`) syntax. Phase 0 landed a *failing* fixture before the fix.

## Goal

Restore upstream-parity sentence segmentation for multi-syllable `luna_pinyin`
input. After M48, `jianli` produces 建立/簡歷/監理 (`jian|li`) and `biancheng`
produces 變成/編程/便成 (`bian|cheng`), matching upstream librime `1.17.0`,
instead of the current over-segmented 及按裏 (`ji+an+li`) / 比按成
(`bi+an+cheng`). The fix must not regress the existing passing parity rows or the
Cantonese / `yune_web` gates.

## Background (root cause, already diagnosed)

The active `luna_pinyin` route scores sentence/lattice paths through the M17
null-grammar poet path. That path sums **raw** essay/dict integer frequencies:
`collect_sentence_states` accumulates `next.weight += null_grammar_score(entry.weight)`
(`crates/yune-core/src/poet/mod.rs:200`, also `:226`, `:367`), where
`null_grammar_score(w) = w + ln(1e-6)` (`poet/mod.rs:83`;
`UPSTREAM_NO_GRAMMAR_PENALTY = -13.815510557964274`, `poet/mod.rs:17`), and
`entry.weight` is the raw frequency carried verbatim (`poet/mod.rs:862` span
edge, `:887` vocab edge, `:1316`/`:1324` `ModelEntry`; single-char dict weights
backfilled from raw essay counts at
`crates/yune-core/src/dictionary/source.rs:866-876`). `compare_path_state`
(`poet/mod.rs:321-328`) ranks by summed weight descending first, fewer-words
only a tiebreak. Because the `-13.8` per-word penalty is negligible against
five-digit frequencies, adding more high-frequency single characters *increases*
the path weight, so over-segmentation wins.

This is **not** a data, segmentation, or deferred-grammar problem. The dict +
`essay.txt` are canonical and complete; `jian`/`bian` form correctly as single
syllables; both the `jian|li` path and the essay phrase edge 建立 are emitted
into the word graph — they lose on score. Reproduced path scores for `jianli`:
`及按裏 60149996` > `建裏 60055116` > `建立 60028890`. The deferred learned
`.gram`/octagram gear is **not** implicated: default upstream `luna_pinyin` uses
no `.gram` model and still returns 建立.

The codebase already applies the correct conversion in the sibling non-poet
sentence path: `sentence_piece_quality = raw_quality.max(1.0).ln() - word_penalty`
(`crates/yune-core/src/translator/mod.rs:142`). The poet path never calls it.

## Reproduction (native, shared core)

```
cargo run -p yune-cli -- frontend \
  --shared-data-dir apps/yune-web/public/schema \
  --user-data-dir <tmp> --schema luna_pinyin \
  --sequence "jianli" --output json
```

Read the **last** event's candidates (the frontend prints one event per
keypress; the first deploy is slow because it compiles the 442k-entry essay).
Native reproduces the exact wrong split, and the wasm and native paths share the
poet scorer, so M48 is a core-shared fix, not web-only.

## Boundary

- **In scope:** converting the poet sentence-model weights to librime-scaled
  log values before path accumulation, the matching unit-test fix, the compact
  `luna_pinyin` preset-vocabulary hydration needed by the shipped runtime path,
  and the new oracle parity fixture. Keep the behavior change scoped to the
  upstream `luna_pinyin` sentence/poet path.
- **Out of scope:** the deferred learned `.gram`/octagram grammar, contextual
  translation, any short-key latency work, Track B / TypeDuck profiles, ABI
  changes, and web/packaging changes. No default-ABI widening.
- **Reference only:** upstream librime `Poet`/`Language`/`Grammar` and
  `dict_compiler.cc` weight convention, and the live oracle
  `https://my-rime.vercel.app/` as a cross-check (authoritative capture is
  librime `1.17.0`).

## Phase 0 — Prove the upstream target (mandatory, lands a FAILING fixture)

- [x] Capture upstream librime `1.17.0` first-page candidates, comments, order,
      context preedit, and commit preview for `jianli` and `biancheng` on
      `luna_pinyin`, in the same shape as the existing
      `crates/yune-core/tests/fixtures/upstream-1.17.0/luna-pinyin-*.json`
      fixtures. Cross-check against `https://my-rime.vercel.app/`.
- [x] Add `jianli` and `biancheng` cases to `upstream_luna_pinyin_parity`
      (`crates/yune-core/tests/upstream_luna_pinyin_parity.rs`) wired to the new
      fixtures. Confirm they **fail** today (Yune returns 及按裏 / 比按成), proving
      the defect is captured by an oracle, not by Yune's own output.
- [x] Confirm the upstream weight convention from librime `1.17.0` source:
      `dict_compiler.cc` stores `log(raw_weight)` with an epsilon floor, and
      `DictEntryIterator::Peek()` subtracts librime's fixed `log(1e8)` scale
      before `Grammar::Evaluate(...)` returns `entry_weight + null_penalty`.
      There is no corpus-total `/total` normalization. Zero/negative weights
      use librime's `DBL_EPSILON` floor.

## Phase 1 — Fix (log-scale the poet weights)

- [x] Feed log-scaled entry weights into the poet accumulation instead of raw
      counts: `ln(raw_or_epsilon) - ln(1e8)`, epsilon-floored, **no** `/total`
      normalization. Prefer converting **at ingestion** so every
      `WordGraphEntry`/`ModelEntry` weight is already log-scaled before
      `compare_path_state` sees it (`crates/yune-core/src/poet/mod.rs:862`,
      `:887`, `:1316`, `:1324`); alternatively convert in the accumulation
      (`:200`, `:226`, `:367`). The existing `-13.815510557964274` per-word
      penalty (`poet/mod.rs:17`) then dominates correctly.
- [x] Confirm the post-fix ranking on the worked examples: `jianli` now returns
      `建立`, `簡歷`, `監理`, `監利`, `剪力`; `biancheng` now returns
      `變成`, `編程`, `便成`, `編成`, `邊城`, so the phrase parse wins.
- [x] Replace the masking poet unit test
      `make_sentence_prefers_single_phrase_when_penalty_outweighs_shorter_path`
      (`crates/yune-core/src/tests/poet.rs:14`): its synthetic `AB=100` vs
      `A+B=19` weights do not model real log-scaled frequencies. Re-express it (or
      add a sibling) so it asserts the over-segmentation invariant with
      realistic log-scaled weights.
- [x] Keep the change behavior-preserving for non-sentence/exact lookups
      (ordering there only needs monotonicity; log is monotonic, so ranking is
      unchanged — verify no fixture moves).

## Gate (definition of done)

- [x] New `jianli` / `biancheng` `upstream_luna_pinyin_parity` fixtures pass.
- [x] Existing parity rows unchanged: `nihao`, `renmin`, `tiantian`, `woshi`,
      `zhongguo`, `ni`, `hao`, `guo`, `zhong` still match upstream.
- [x] `cargo test -p yune-core --test upstream_luna_pinyin_parity` green.
- [x] `cargo test -p yune-core --test cantonese_parity` green (TypeDuck v1.1.2
      unaffected).
- [x] `cargo test -p yune-rime-api --test yune_web` green (ABI contract intact).
- [x] `cargo fmt --check` green.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` currently blocks
      on an unrelated existing lint in
      `crates/yune-core/src/dictionary/compiled_prism.rs:430`
      (`clippy::ref_option`).
- [ ] Browser cross-check on the public demo was not run; M48 closeout makes no
      browser-visible claim beyond the native/shared-core CLI and `yune_web`
      integration evidence.

## Closeout Evidence (2026-06-29)

- Upstream oracle fixture regenerated with `scripts/capture-upstream-m17-poet.ps1`
  against pinned librime `1.17.0`. The fixture locks `jianli` as
  `建立`, `簡歷`, `監理`, `監利`, `剪力`, and `biancheng` as
  `變成`, `編程`, `便成`, `編成`, `邊城`.
- Production CLI over `apps/yune-web/public/schema` after the fix:
  `jianli` -> `建立`, `簡歷`, `監理`, `監利`, `剪力`;
  `biancheng` -> `變成`, `編程`, `便成`, `編成`, `邊城`.
- Targeted verification passed:
  `cargo test -p yune-core --test upstream_luna_pinyin_parity`;
  `cargo test -p yune-core poet`;
  `cargo test -p yune-core --test cantonese_parity`;
  `cargo test -p yune-rime-api --test yune_web`;
  `cargo fmt --check`.
- Broad clippy was attempted and failed only on the unrelated pre-existing
  `compiled_prism.rs:430` `clippy::ref_option` lint; M48 did not change that
  file.

## Risks / watch-items

- **Weight source double-check:** if any poet weights are *already* log-scaled
  somewhere, double-converting would invert the fix. Phase 1 must verify the raw
  scale at the ingestion sites before converting.
- **Epsilon/floor mismatch:** use librime's exact floor (Phase 0) so edge cases
  (zero/one-count entries) rank identically to the oracle.
- **Tiebreak interactions:** `compare_path_state` still falls back to word count
  and text; confirm the log scale does not surface a new tie that reorders an
  existing passing row.
