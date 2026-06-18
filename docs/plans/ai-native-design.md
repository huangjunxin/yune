# AI-Native Input Layer — Design

> **Status:** Active · **Milestone:** M11 (AI-native input layer) · **Updated:** 2026-06-18 · **Type:** design

> **Audience.** Yune maintainers + an executing agent. This is the north-star
> architecture for the AI-native layer; the first executable slice is in
> [`ai-native-cli-slice-plan.md`](./ai-native-cli-slice-plan.md). It is a **draft**
> meant to evolve — the contracts here are the load-bearing part.
>
> **Why now.** AI-native is a *separate layer above the compatibility foundation*
> (decisions.md standing principles). It rides on the **CLI surrogate** and the
> finished engine (M0–M8), not on the browser/Windows frontends, so it can be
> designed and built **in parallel** with the M9/M10 frontend work without touching it.

## 1. Goal & scope

Make AI/LLM assistance a first-class **source of candidates, ranking, context, and
memory** — *without* making classic RIME input slower, less predictable, or
dependent on a network. Covers requirements **AI-01…AI-07** (`requirements.md`).

In scope: the provider/ranking/context/memory/privacy contracts and a CLI
playground. Out of scope (for now): a production remote-LLM integration, a new
GUI, and exposing AI through native frontends (those stay AI-off by default until
the CLI proves the behavior — AI-07).

## 2. Non-negotiable invariants

These come straight from the standing decisions; every component below is designed
to preserve them, and the test suite must enforce them:

1. **Never block or slow classic input.** AI work is non-blocking; if a result
   isn't ready within budget, the engine uses the classic ordering. (Builds on the
   existing `RerankResult::Pending` contract.)
2. **Classic candidates always available** when AI is disabled, pending, or failed.
3. **AI candidates are source-labeled** and **never auto-commit by default**.
4. **Local-first.** Baseline behavior works with mock/local providers; remote LLM
   calls are an opt-in enhancement, never required.
5. **Deterministic fallback.** Timeout/fallback behavior is deterministic so tests
   are reproducible; tests use mock providers.
6. **Privacy is opt-in, inspectable, clearable.** Sensitive contexts disable
   learning *and* remote calls. The user can inspect, clear, and disable memory.
7. **AI memory is separate from librime userdb.** Personalization must not corrupt
   classic dictionary/userdb compatibility (which is measured against librime).

## 3. What already exists (build on, don't replace)

The engine pipeline (`crates/yune-core/src/engine.rs`) is: translate → sort by
`quality` → `CandidateFilter`s → `CandidateRanker`s → set `context.candidates`
(engine.rs:817–826). Relevant seams:

- `trait Translator` — produces `Candidate`s from input.
- `trait CandidateRanker { fn try_rerank(&self, &Context, &[Candidate]) -> RerankResult }` (`lib.rs:75`). `RerankResult::Pending` keeps classic order; `Ready(..)` replaces it (`lib.rs:105`). `MockAiRanker` is the existing example (`lib.rs:110`). Installed via `Engine::add_ranker` (`engine.rs:95`).
- `struct Candidate { text, comment, source: CandidateSource, quality }`; `enum CandidateSource` (Table/Completion/Sentence/…).

The AI layer **extends** these, it does not fork them.

## 4. Architecture

```
                 bounded, privacy-classified context
                              │
        ┌─────────────────────┼───────────────────────────┐
        ▼                     ▼                            ▼
  AiCandidateProvider    AiRanker (budgeted)        ContextProvider + PrivacyClassifier
  (source-labeled,        (extends CandidateRanker;        (what may be shared;
   non-blocking)           Pending on timeout)              sensitive ⇒ no learn/remote)
        │                     │                            │
        └────────► Engine pipeline (classic first) ◄───────┘
                              │
                       MemoryStore (separate from librime userdb)
                              │
                Provider backends: mock | local-model | (opt-in) remote
```

### 4.1 Candidate provision — `AiCandidateProvider` (AI-01, AI-03)
A provider that returns **source-labeled** AI candidates for the current bounded
input context. Modeled as a `Translator`-shaped contributor *or* a dedicated
`AiCandidateProvider` trait installed alongside translators. Key rules:
- Output candidates carry `CandidateSource::Ai { provider, confidence }` (extend the
  enum) so the UI and merge policy can distinguish them.
- **Non-blocking**: the provider returns immediately with whatever is ready for the
  *current* input; results computed for a stale input are discarded.
- AI candidates are **never auto-committed** and never displace classic candidates
  from being selectable.

### 4.2 Non-blocking ranking & merge (AI-02)
- An `AiRanker` extends the existing `CandidateRanker`: it runs background
  computation and, on each key event, returns `Ready(reordered)` only if a result
  for the current input is available within a **strict time budget**; otherwise
  `Pending` (classic order preserved). Late results are applied only at stable
  boundaries (next key event) or discarded — never mid-render.
- A **merge policy** defines deterministic ordering across `table`, `completion`,
  `sentence`, `userdb`, and `ai` sources. AI never preempts the top classic
  candidate unless explicitly configured, and never auto-commits.

### 4.3 Context & privacy (AI-04, AI-06)
- A `ContextProvider` assembles a **bounded** context (preceding text, cursor,
  field/app hints, schema id, current candidate list) and a `PrivacyClassifier`
  tags it by sensitivity.
- **Sensitive ⇒ disable learning and remote calls** for that context; classic input
  stays fully functional. Nothing leaves the device unless the user opted into a
  remote provider *and* the context is non-sensitive.

### 4.4 Memory & personalization (AI-05)
- A `MemoryStore` records user vocabulary, phrase/domain preferences, and style —
  **separate from the librime-compatible userdb** so classic compatibility is never
  corrupted. It influences ranking/completion *through* the provider/ranker, not by
  writing librime userdb.
- Inspectable, clearable, and disable-able by the user; updates respect the privacy
  classifier.

### 4.5 Provider backends
`mock` (deterministic, for tests + CLI demos) → `local-model` (on-device) →
optional `remote`. The trait surface is backend-agnostic; baseline ships mock+local.

## 5. Observability & CLI playground (AI-07)
- CLI flags enable a mock/local provider per run (`--ai-provider mock|local|none`,
  off by default).
- The transcript records, per key event: AI source labels, the timeout/fallback
  decision (Ready vs Pending), and the merge result — so AI behavior is *observable
  and diffable* in the CLI before any native frontend depends on it.

## 6. Phasing (slices → requirements)
- **S1 — Provider interface + mock in CLI** (AI-01/03/07): `CandidateSource::Ai`, the
  provider/ranker trait, a mock provider, engine + CLI wiring (off by default),
  non-blocking + source-labeled + no auto-commit, deterministic tests, transcript
  fields. *(First slice — see the slice plan.)*
- **S2 — Budgeted ranking + merge policy** (AI-02).
- **S3 — Context provider + privacy classifier** (AI-04/06).
- **S4 — Memory store** (AI-05), kept separate from librime userdb.
- **S5 — Local-model backend**; remote stays optional/later.

## 7. Risks / open questions
- **Async without blocking the synchronous pipeline.** Leading approach: background
  worker keyed by current input; the engine only ever reads the latest *ready*
  result (else `Pending`). Needs care so stale results never apply.
- **Merge determinism vs. usefulness.** A fixed merge order is testable but may feel
  static; revisit once S2 has real signals.
- **Memory ↔ userdb boundary.** Must prove AI memory never leaks into librime userdb
  state that compatibility tests measure.
- **Privacy classification correctness.** Misclassifying a sensitive field is the
  highest-severity failure mode; default to "sensitive" when unsure.

## 8. Safety invariants the test suite must enforce
- Disabling AI (or a `Pending`/failed provider) yields **byte-identical** classic
  output to AI-off.
- No AI candidate auto-commits.
- A provider exceeding its time budget never delays a key event.
- Sensitive context ⇒ no remote call and no memory write (assert with a recording
  mock).
- AI memory writes never touch librime userdb files.

---

*Draft created 2026-06-18. The architecture/contracts are the durable part; specifics will evolve as slices land. Builds on the M4 `CandidateRanker` hook; gated by the standing AI-native principles in [`../decisions.md`](../decisions.md) and requirements AI-01…AI-07 in [`../requirements.md`](../requirements.md).*
