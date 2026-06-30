# M48 luna_pinyin Sentence Over-Segmentation Fix Plan

> **Status:** Active — scoped, not started. - **Track:** Core compatibility (engine correctness; null-grammar sentence path) - **Created:** 2026-06-29 - **Type:** oracle-driven correctness fix (Phase 0 proves the upstream target before any code change)
>
> **For agentic workers:** this is oracle-driven. Capture expected bytes from the **external oracle** (upstream librime `1.17.0`, the pinned target), run Yune's real production path, assert it matches. **Never derive expected values from Yune.** The read-only diagnosis is already done and recorded in [`../../roadmap.md`](../../roadmap.md) under "Open Correctness Defect: luna_pinyin sentence over-segmentation"; do not re-litigate it. Steps use checkbox (`- [ ]`) syntax. Land Phase 0 (a *failing* fixture) before the fix.

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

- **In scope:** converting the poet sentence-model weights to log-scaled values
  before path accumulation, the matching unit-test fix, and the new oracle parity
  fixture. Keep the change inside `yune-core` (the poet / sentence-model build),
  behavior-preserving for non-sentence lookups.
- **Out of scope:** the deferred learned `.gram`/octagram grammar, contextual
  translation, any short-key latency work, Track B / TypeDuck profiles, ABI
  changes, and web/packaging changes. No default-ABI widening.
- **Reference only:** upstream librime `Poet`/`Language`/`Grammar` and
  `dict_compiler.cc` weight convention, and the live oracle
  `https://my-rime.vercel.app/` as a cross-check (authoritative capture is
  librime `1.17.0`).

## Phase 0 — Prove the upstream target (mandatory, lands a FAILING fixture)

- [ ] Capture upstream librime `1.17.0` first-page candidates, comments, order,
      context preedit, and commit preview for `jianli` and `biancheng` on
      `luna_pinyin`, in the same shape as the existing
      `crates/yune-core/tests/fixtures/upstream-1.17.0/luna-pinyin-*.json`
      fixtures. Cross-check against `https://my-rime.vercel.app/`.
- [ ] Add `jianli` and `biancheng` cases to `upstream_luna_pinyin_parity`
      (`crates/yune-core/tests/upstream_luna_pinyin_parity.rs`) wired to the new
      fixtures. Confirm they **fail** today (Yune returns 及按裏 / 比按成), proving
      the defect is captured by an oracle, not by Yune's own output.
- [ ] Confirm the upstream weight convention from librime `1.17.0` source:
      `dict_compiler.cc` stores `log(raw_weight)` with an epsilon floor, and
      `Grammar::Evaluate(...)` returns `entry_weight + null_penalty` (no
      `/total` normalization). Record the exact epsilon/floor used so Yune
      matches rather than approximates.

## Phase 1 — Fix (log-scale the poet weights)

- [ ] Feed log-scaled entry weights into the poet accumulation instead of raw
      counts: `entry.weight.max(1.0).ln()`, epsilon-floored, **no** `/total`
      normalization. Prefer converting **at ingestion** so every
      `WordGraphEntry`/`ModelEntry` weight is already log-scaled before
      `compare_path_state` sees it (`crates/yune-core/src/poet/mod.rs:862`,
      `:887`, `:1316`, `:1324`); alternatively convert in the accumulation
      (`:200`, `:226`, `:367`). The existing `-13.815510557964274` per-word
      penalty (`poet/mod.rs:17`) then dominates correctly.
- [ ] Confirm the post-fix ranking on the worked example: `建立 ≈ -3.54` >
      `jian|li (建+裏) ≈ -7.71` > `ji+an+li (及+按+裏) ≈ -9.27`, so the phrase
      parse wins.
- [ ] Replace the masking poet unit test
      `make_sentence_prefers_single_phrase_when_penalty_outweighs_shorter_path`
      (`crates/yune-core/src/tests/poet.rs:14`): its synthetic `AB=100` vs
      `A+B=19` weights do not model real log-scaled frequencies. Re-express it (or
      add a sibling) so it asserts the over-segmentation invariant with
      realistic log-scaled weights.
- [ ] Keep the change behavior-preserving for non-sentence/exact lookups
      (ordering there only needs monotonicity; log is monotonic, so ranking is
      unchanged — verify no fixture moves).

## Gate (definition of done)

- [ ] New `jianli` / `biancheng` `upstream_luna_pinyin_parity` fixtures pass.
- [ ] Existing parity rows unchanged: `nihao`, `renmin`, `tiantian`, `woshi`,
      `zhongguo`, `ni`, `hao`, `guo`, `zhong` still match upstream.
- [ ] `cargo test -p yune-core --test upstream_luna_pinyin_parity` green.
- [ ] `cargo test -p yune-core --test cantonese_parity` green (TypeDuck v1.1.2
      unaffected).
- [ ] `cargo test -p yune-rime-api --test yune_web` green (ABI contract intact).
- [ ] `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Browser cross-check on the public demo: `jianli` → 建立 and `biancheng` →
      變成 on `yune-web` (Playwright evidence per the web-claim rule).

## Risks / watch-items

- **Weight source double-check:** if any poet weights are *already* log-scaled
  somewhere, double-converting would invert the fix. Phase 1 must verify the raw
  scale at the ingestion sites before converting.
- **Epsilon/floor mismatch:** use librime's exact floor (Phase 0) so edge cases
  (zero/one-count entries) rank identically to the oracle.
- **Tiebreak interactions:** `compare_path_state` still falls back to word count
  and text; confirm the log scale does not surface a new tie that reorders an
  existing passing row.
