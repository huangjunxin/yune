# AI-Native Slice S1 — Provider Interface + Mock in the CLI

> **Status:** Active · **Milestone:** M11 (AI-native input layer) · **Created:** 2026-06-18 · **Type:** execution plan

> **Audience.** An executing agent. This is slice **S1** of the AI-native layer
> ([design](./ai-native-design.md)): the minimal, source-labeled, non-blocking,
> off-by-default AI candidate path, demonstrable in the CLI. Delivers **AI-01,
> AI-03, AI-07**.
>
> **Parallel-safe.** S1 touches **only `crates/yune-core` and `crates/yune-cli`** —
> not `yune-rime-api/src/typeduck_web.rs` or any browser/Windows frontend path — so
> it can run **alongside the M9 TypeDuck-Web work** without file conflicts. (Stage
> by path when committing on the shared checkout.)

## Goal
A CLI run can enable a **mock AI provider** that contributes a **source-labeled** AI
candidate which is **non-blocking** and **never auto-commits**, while a default run
stays **byte-identical** to today. This proves the layer's contracts on the CLI
surrogate before any native frontend depends on it.

## Build on what exists (do NOT fork)
Engine pipeline: translate → sort by `quality` → `CandidateFilter`s → `CandidateRanker`s → set candidates (`crates/yune-core/src/engine.rs:817-826`). `CandidateRanker::try_rerank → RerankResult::{Pending,Ready}` (`lib.rs:75/105`) already encodes the non-blocking contract; `MockAiRanker` (`lib.rs:110`) is the pattern; `Engine::add_ranker` (`engine.rs:95`) is the install seam. `Candidate { text, comment, source: CandidateSource, quality }`.

## Work items (one commit each)

### WI-1 — Source labeling
- Extend `CandidateSource` (`crates/yune-core/src/state.rs`) with an AI variant, e.g. `Ai { provider: String, confidence: f32 }`. Update all `match` sites (grep `CandidateSource::`) so the build stays warning-clean.
- Surface the label in CLI render + transcript (`crates/yune-cli/src/render.rs`, `transcript.rs`) so an AI candidate is visibly distinguishable.
- **Acceptance:** existing tests pass; an AI-sourced candidate renders with a clear source marker.

### WI-2 — Provider trait + mock (non-blocking)
- Add an owned module `crates/yune-core/src/ai/mod.rs` with `trait AiCandidateProvider { fn name(&self) -> &'static str; fn provide(&self, context: &Context, budget: Duration) -> AiResult; }` where `AiResult` is `Pending | Ready(Vec<Candidate>)` (mirror `RerankResult`'s non-blocking shape). Add `MockAiProvider` returning one deterministic source-labeled suggestion (`Ready`) or `Pending`.
- Install via `Engine::add_ai_provider` (analogous to `add_ranker`); run providers in the pipeline so AI candidates are **merged after** classic ones (non-preempting) and clearly labeled. Providers are **off unless installed**.
- **Acceptance:** with no provider installed, candidate output is **byte-identical** to baseline; with `MockAiProvider`, a labeled AI candidate appears after classic candidates; a `Pending` provider changes nothing.

### WI-3 — CLI flag + observability (AI-07)
- Add `--ai-provider mock|none` to `crates/yune-cli` (default `none`). `mock` installs `MockAiProvider`.
- Transcript records, per key event: the AI source label(s) and the `Ready`/`Pending` (fallback) decision and merge result.
- **Acceptance:** `cargo run -p yune-cli -- --ai-provider mock …` shows the labeled AI candidate + the fallback decision in the transcript; default run output unchanged.

### WI-4 — Safety tests (enforce the invariants)
Owned tests in `yune-core` (provider/merge) + `yune-cli` (transcript/flag):
- AI-off **and** `Pending`-provider runs produce **byte-identical** output to baseline.
- AI candidate is present and source-labeled when the mock is active.
- No AI candidate auto-commits (selection/commit behavior unchanged).
- Deterministic (mock is pure; no wall-clock dependence in assertions).

## Out of scope for S1 (later slices)
Budgeted ranking + merge policy (S2/AI-02), context + privacy classifier (S3/AI-04/06), memory store (S4/AI-05), local-model/remote backends (S5). Keep S1's mock pure and synchronous; the real async/budget model arrives in S2.

## Quality gate
`cargo fmt` · `cargo clippy --workspace --all-targets -- -D warnings` · focused `yune-core` + `yune-cli` tests · `cargo test --workspace`. Per CONVENTIONS: own each slice (new behavior in `crates/yune-core/src/ai/` + matching tests; keep `lib.rs`/`main.rs` facades). Update this checklist as WIs land; do not touch the M9 TypeDuck-Web files.

## Checklist
- [ ] WI-1 — `CandidateSource::Ai` + CLI render/transcript labeling
- [ ] WI-2 — `AiCandidateProvider` trait + `MockAiProvider`, non-blocking, off-by-default, non-preempting merge
- [ ] WI-3 — `--ai-provider` CLI flag + transcript observability
- [ ] WI-4 — safety tests (AI-off byte-identical, labeled, no auto-commit, deterministic)

---

*Draft created 2026-06-18. First slice of the AI-native layer ([design](./ai-native-design.md)); delivers AI-01/03/07 and stays parallel-safe with M9.*
