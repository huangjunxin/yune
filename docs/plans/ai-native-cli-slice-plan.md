# AI-Native Slice S1 — Provider Interface + Mock in the CLI

> **Status:** Active · **Milestone:** M11 (AI-native input layer) · **Updated:** 2026-06-18 · **Type:** execution plan

> **Audience.** An executing agent. Slice **S1** of the AI-native layer
> ([design](./ai-native-design.md)): the minimal, source-labeled, non-blocking,
> off-by-default AI candidate path in the CLI. Delivers **AI-01, AI-03, AI-07** —
> **plus three cheap, safety-critical enforcement fixes the design review promoted
> into S1** (userdb-leak gate, commit-boundary no-auto-commit, single merge
> function). Revised after the judge-panel review.
>
> **Parallel-safe.** S1 touches **only `crates/yune-core` and `crates/yune-cli`** —
> not `yune-rime-api/src/typeduck_web.rs` or any browser/Windows path — so it runs
> **alongside M9** without file conflicts. Stage by path on the shared checkout.

## Goal
A CLI run can enable a **mock AI provider** contributing a **source-labeled**,
**non-blocking** AI candidate that **never auto-commits** and — critically —
**never writes the librime userdb**, while a default run stays **byte-identical**
to today. Proves the layer's safety contracts on the CLI surrogate first.

## Build on what exists (do NOT fork)
Pipeline `crates/yune-core/src/engine.rs`: translate → userdb extend → sort by `quality` → filters → rankers → set candidates (engine.rs:805–826). `CandidateRanker`/`RerankResult::{Pending,Ready}` (lib.rs:75/105) is the non-blocking primitive; `Engine::add_ranker` (engine.rs:95) the install seam. `Candidate { text, comment, source: CandidateSource, quality }`; `CandidateSource::Ai` already exists as a **unit** variant (state.rs). **Commit/userdb path:** `commit_candidate` (engine.rs:740–762) stages `pending_userdb_learning` for *every* committed candidate; the host drains it to a `*.userdb` file (`session.rs`). `UserDbCommitMetadata.candidate_source` (userdb.rs:11) already exists; `BackdatedScanPolicy { scans_ai_ranker_memory: false }` already anticipates this gate.

## Work items (one commit each)

### WI-1 — Source labeling (keep `Ai` a UNIT variant)
- Keep `CandidateSource::Ai` as the existing **unit** variant — do **not** add `{ provider, confidence }`: an `f32` field breaks `#[derive(Eq)]` on `CandidateSource` and ripples into `UserDbCommitMetadata`/`assert_eq!` sites. Carry `provider`/`confidence` in `Candidate.comment` (e.g. `"ai:mock 0.62"`).
- Surface the label in CLI render + transcript (`crates/yune-cli/src/render.rs`, `transcript.rs`) so AI rows are visibly distinct.
- **Acceptance:** build stays warning-clean; an AI candidate renders with a clear source marker; existing tests pass.

### WI-2 — `AiCandidateProvider` + mock + the single merge function
- Owned module `crates/yune-core/src/ai/mod.rs`: `trait AiCandidateProvider { fn name(&self)->&'static str; fn provide(&self, ctx:&Context, budget:Duration)->AiResult }`, `enum AiResult { Pending, Ready(Vec<Candidate>) }`. `MockAiProvider` returns one deterministic source-labeled suggestion (`Ready`) or `Pending` from a fixed input→suggestion map — **pure/synchronous** (no thread, no clock).
- Install via `Engine::add_ai_provider` (mirror `add_ranker`); providers **off unless installed**.
- Add **one deterministic merge function** that is the **sole writer** of `context.candidates` ordering: classic candidates first (preserving their order/indices), AI candidates appended after, and the **top classic candidate pinned at index 0** (no AI preemption in S1). Do *not* let a ranker full-replace ordering.
- **Acceptance:** no provider installed ⇒ candidate output **byte-identical** to baseline; with `MockAiProvider`, a labeled AI row appears **after** classic candidates and index 0 stays classic; a `Pending` provider changes nothing.

### WI-3 — Commit-boundary safety (no auto-commit + userdb-leak gate)  ← highest priority
- **No auto-commit:** in `commit_candidate`/`commit_highlighted` (engine.rs:740–762, 167), branch on `candidate.source` so an `Ai` candidate is never the default/space-committed selection (committing one requires explicit user navigation+selection).
- **userdb-leak gate (the §2.7 invariant):** when the committed `candidate.source == CandidateSource::Ai`, **do not** stage `pending_userdb_learning` (route to the future `MemoryStore` instead). Guard it so **classic** learning on the shared hot path is unaffected.
- **Acceptance:** committing an `Ai` candidate leaves `userdb().entries()` unchanged **and** `take_pending_userdb_learning()` returns `None`; committing a **classic** candidate still stages learning exactly as before.

### WI-4 — CLI flag + observability (AI-07)
- `--ai-provider mock|none` (default `none`); `mock` installs `MockAiProvider`.
- Transcript records, per key event, the AI source label(s) and a **discrete, input-derived** `ai_decision` enum (`ready | pending | off`) — computed from whether a result for the *current* input was staged, **never** from wall-clock time (keeps the transcript deterministic, consistent with the no-timestamp contract).
- **Acceptance:** `cargo run -p yune-cli -- --ai-provider mock …` shows the labeled AI row + the `ai_decision`; default run output unchanged.

### WI-5 — Safety tests (enforce the invariants)
Owned tests in `yune-core` + `yune-cli`:
- AI-off **and** `Pending`-provider runs are **byte-identical** to baseline.
- AI candidate present + source-labeled when mock active; **index 0 stays classic**.
- **No `Ai` auto-commit**; default/space commit is always classic.
- **userdb isolation:** after an `Ai` commit, `userdb().entries()` unchanged + `take_pending_userdb_learning()` is `None`; after a classic commit, learning is staged as before.
- Deterministic (mock is pure; assertions are value-based, not timing-based).

## Out of scope for S1 (later slices)
Async background worker + input-keyed results + time budget and the `CandidateSource::Ai` struct variant (**S2**); `ContextProvider` + privacy classifier (**S3**); persisted `MemoryStore` (**S4**); local-model/remote backends (**S5**).

## Quality gate
`cargo fmt` · `cargo clippy --workspace --all-targets -- -D warnings` · focused `yune-core` + `yune-cli` tests · `cargo test --workspace`. Per CONVENTIONS: own each slice (new behavior in `crates/yune-core/src/ai/` + matching tests; keep `lib.rs`/`main.rs` facades). Do not touch the M9 TypeDuck-Web files; stage by path.

## Checklist
- [ ] WI-1 — `CandidateSource::Ai` kept unit; provider/confidence in `comment`; CLI labeling
- [ ] WI-2 — `AiCandidateProvider` + `MockAiProvider` (pure, off-by-default) + single merge function pinning index 0
- [ ] WI-3 — commit-boundary no-auto-commit **and** userdb-leak gate (no `pending_userdb_learning` for `Ai`)
- [ ] WI-4 — `--ai-provider` flag + discrete `ai_decision` transcript field
- [ ] WI-5 — safety tests (byte-identical, labeled, index-0-classic, no-auto-commit, **userdb isolation**, deterministic)

---

*Draft 2026-06-18, revised after the judge-panel design review. First slice of the AI-native layer ([design](./ai-native-design.md)); delivers AI-01/03/07 + the promoted safety gates; parallel-safe with M9.*
