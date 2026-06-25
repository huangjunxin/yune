# Roadmap

Yune is a Rust input-method engine that uses **upstream librime as a
compatibility and performance oracle** while building a cleaner Rust engine.
The current priority is not application integration. It is engine behavior and
engine performance: prove that Yune can match librime's observable behavior and
then converge toward librime's startup, mmap-backed `rsmarisa` lookup,
lazy/page-bounded candidate iteration, memory, and allocation shape.

> **Compatibility oracle.** Upstream librime latest stable is the default
> behavior reference for user-visible schema semantics, standard ABI contracts,
> deployed data, and migration. The current pinned upstream target is
> `rime/librime 1.17.0`
> (`33e78140250125871856cdc5b42ddc6a5fcd3cd4`):
> <https://github.com/rime/librime>. This is a referenced upstream repository,
> not a local checkout path.

## Document Map

- This file - current engine roadmap dashboard, active sequence, scope
  boundaries, and readiness gates.
- [`ledgers/milestone-history.md`](./ledgers/milestone-history.md) - historical
  milestone ledger split out of this roadmap.
- [`conventions.md`](./conventions.md) - architecture, stack, structure,
  coding/testing conventions, integrations, current risks, and planning-doc
  rules.
- [`decisions.md`](./decisions.md) - standing principles plus project-wide
  decision log.
- [`requirements.md`](./requirements.md) - requirement IDs and status,
  including the closed M37 engine gates and active M38 parity gates.
- [`reports/yune-vs-librime-performance.md`](./reports/yune-vs-librime-performance.md)
  and [`reports/yune-vs-librime-root-cause-analysis.md`](./reports/yune-vs-librime-root-cause-analysis.md)
  - current performance comparison and diagnosis.
- [`plans/active/m38-plan-engine-performance-parity.md`](./plans/active/m38-plan-engine-performance-parity.md)
  - active pure engine performance parity plan.
- [`plans/completed/m37-plan-engine-hyper-optimization.md`](./plans/completed/m37-plan-engine-hyper-optimization.md)
  - completed engine hyper-optimization milestone.
- [`plans/`](./plans) - active, reference, and completed plans, findings,
  contracts, and validation artifacts; finished plans live under
  [`plans/completed/`](./plans/completed).

> The GSD planning system (`.planning/`) has been retired; durable planning now
> lives under `docs/`.

## Current Snapshot

| Lane | Current state | Next decision or gate |
| --- | --- | --- |
| Core compatibility | Phase 1 named-target upstream behavior is complete for `luna_pinyin` and common-schema basics against upstream librime `1.17.0`. | Preserve upstream-observable behavior on every engine change. |
| Engine performance | M33-M37 removed several real costs, but the fair upstream comparison still shows large lookup and memory gaps. | Close **M38** only when isolated engine startup/session and typing latency reach librime-level thresholds, with mmap-backed `rsmarisa` marisa-table lookup, lazy/page-bounded candidate iteration, context export, memory, and allocation gates all satisfied. |
| AI-native engine layer | M11/M13 proved a default-off local AI layer can sit on top of the deterministic engine. | Keep AI outside the classic deterministic performance path unless a named engine experiment explicitly enables it. |
| Future platform work | Platform-specific frontends and application shells are outside this roadmap. | Start a separate repository or separate plan before changing platform/application contracts. |

## Authoritative Sequence

1. **M38 engine performance parity** - active engine track after M37; it targets
   isolated native engine startup, session lifecycle, mmap-backed `rsmarisa`
   marisa-table lookup, lazy/page-bounded candidate iteration, context export,
   memory, and allocation against same-run librime.
2. **Future AI-native engine experiments** - later, and only after classic
   engine performance is no longer dominated by avoidable pipeline costs.

Trigger-gated, not scheduled: extracting the full processor pipeline from
`yune-rime-api` into `yune-core` lands only when a real non-ABI consumer needs
the full input path. Do not milestone that extraction speculatively.

## M37 Closeout

M37 is complete. It improved native engine storage and candidate
materialization, but it also made the key lesson clearer: end-to-end
application-shaped rows are not the right optimization target for the next milestone.
M38 therefore treats M37 as history and returns to a clean upstream
Yune-versus-librime engine comparison.

Engine lessons carried forward:

- Full-list candidate materialization can dominate short ambiguous input.
- Page-sized context export is required before frontend-shaped rows can be
  meaningful.
- Mapped or byte-backed deployed data helps memory only if it avoids rebuilding
  parallel heap mirrors.
- A storage probe is not the same thing as a runtime lookup backend.
- `rsmarisa` is an engine-performance concern when it is the route to a real
  marisa-backed table hot path.
- A faster lookup backend does not close the milestone if candidate production
  still expands and sorts a full list for a first-page read.
- Native engine wins must not be described as application, browser, or delivery
  wins without separate evidence.

## M38 Readiness

M38 is active because the clean upstream comparison still shows a large engine
gap.

M37 final upstream comparison:

- Warm startup/runtime-ready: Yune `50,415.700us`, librime `29,163.700us`
  (`1.73x`).
- Session create/select/destroy: Yune `48,233.200us`, librime `29,940.000us`
  (`1.61x`).
- `hao`: Yune `4,145.500us`, librime `11.900us` (`348.36x`).
- `ni`: Yune `3,171.050us`, librime `14.600us` (`217.20x`).
- `zhongguo`: Yune `4,801.675us`, librime `185.300us` (`25.91x`).
- Median working set: Yune `159-161 MB`, librime `11-13 MB`.

The M38 closeout gates are deliberately engine-only:

- Phase 0 must rerun same-machine Yune and librime baseline rows before
  implementation.
- Phase 0 must attribute startup/session, raw prism lookup, raw table lookup,
  selected table backend, `rsmarisa` lookup calls, translator candidate
  production, context export, memory, and allocation owners.
- Final engine status must prove a real marisa-backed deployed table is selected
  through `rsmarisa` for the benchmarked hot path. Probe-only evidence, mmap
  success, or extracted-payload inspection does not close M38.
- Final selected table/prism bytes must be mmap-backed or otherwise
  file-backed/borrowed on the native hot path. Owned-buffer selected table data
  or a full heap mirror does not close M38.
- Final candidate production must use lazy/page-bounded iterator behavior or an
  equivalent bounded view for ordinary first-page reads. Full-list fallback must
  be explicit, counted, and semantically justified.
- Final startup and session medians must be within `1.25x` of same-run librime.
- Final `hao`, `ni`, and `zhongguo` rows must each be within `5x` of same-run
  librime. Rows that merely improve from M37 but remain outside this bound do
  not close M38.
- Final evidence must explain any remaining gap above librime. M38 cannot close
  with unexplained lookup or memory outliers.
- Application, frontend, browser, packaging, and public delivery performance are
  explicitly out of scope.

## Track Map

| Track | Scope | Current source of truth |
| --- | --- | --- |
| Engine performance | Native engine startup, schema/session lifecycle, mmap-backed `rsmarisa` marisa-table lookup, lazy/page-bounded translation, context export, memory, and allocation | Active: [`plans/active/m38-plan-engine-performance-parity.md`](./plans/active/m38-plan-engine-performance-parity.md). Prior closeout: [`plans/completed/m37-plan-engine-hyper-optimization.md`](./plans/completed/m37-plan-engine-hyper-optimization.md), performance reports, and `docs/reports/evidence/`. |
| Core compatibility | Upstream behavior fixtures and standard ABI-observable behavior | [`requirements.md`](./requirements.md), [`decisions.md`](./decisions.md), and per-milestone plans. |
| AI-native engine research | Default-off AI behavior layered above the deterministic engine | Future explicit engine experiments only. |
| Historical record | Completed milestone outcomes and reference/provenance pointers | [`ledgers/milestone-history.md`](./ledgers/milestone-history.md). |

## Milestone Ledger

| Milestone or track | Status | Current roadmap meaning |
| --- | --- | --- |
| M0-M24 | Complete | Phase 1 named-target engine/basic oracle parity is complete; history lives in [`ledgers/milestone-history.md`](./ledgers/milestone-history.md). |
| M25-M30 | Complete | Early performance and runtime-hardening work is historical context only. |
| M31 | Complete | Public demo delivery is historical context and not a current engine-performance target. |
| M33-M37 | Complete | Recent engine-performance work closed fairness, shared caches, compact storage, compiled-active paths, page-bounded materialization, and mapped storage experiments. |
| M38 | Draft / active | Pure engine performance parity against same-run upstream librime evidence, including mandatory mmap-backed `rsmarisa` marisa-table lookup and lazy/page-bounded candidate iteration. |

## Scope Ledger

A living map so "parity" always names a target. Deferred rows move into scope
only when an engine target needs them; nothing here commits to a timeline.

| In scope - target-driven, measured | Deferred - implement when an engine target needs it | Non-goal |
| --- | --- | --- |
| `luna_pinyin` core versus upstream `1.17.0`, including completed M17 null-grammar sentence/lattice and M18 punctuation processor slices | Learned `.gram`/octagram grammar, contextual translation, and broader plugin-backed gears until a named engine target needs them | Bit-for-bit parity with librime internals |
| Common RIME schemas added through explicit breadth milestones | Further schema breadth only with fresh oracle fixtures and owning tests | Unbounded schema checklist work |
| Native engine performance parity for startup, session lifecycle, mmap-backed `rsmarisa` marisa-table lookup, raw lookup, lazy/page-bounded translation, context export, memory, and allocation | Frontend/application delivery evidence and platform packaging | Claiming application-visible wins from native engine evidence |
| AI-native layer on the compatible deterministic base | Richer AI experiments after the classic engine path is competitive | Replacing or altering classic input paths by default |

## Deferred / Future

- **librime C++ plugin ABI** (Lua, octagram, predict, proto): deferred until a
  concrete engine target requires it; prefer Yune-native extension points first.
- **AI-native input layer beyond M13:** future work owns richer local-first AI
  behavior, privacy/memory controls, and any explicit remote-provider decision.
  Until then, proven AI remains default-off and outside the classic performance
  path.
- **Frontend/application delivery:** platform UI, browser delivery, packaging,
  cache behavior, and public deployment are outside the engine roadmap. They
  need separate plans and separate evidence.
- **iOS keyboard developer support:** future keyboard SDK or host work needs
  its own Yune-native package/host contract.

## Principles

The standing principles that govern all current and future work - librime as
oracle not template, name-the-protected-behavior, own-each-slice, AI-native as
a separate local-first layer, fixtures before module replacement, deferred
plugin ABI, and upstream-first oracle sequencing - have one canonical home:
[`decisions.md` -> Standing principles](./decisions.md#standing-principles).
