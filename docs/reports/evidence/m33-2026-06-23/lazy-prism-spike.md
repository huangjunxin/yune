# M33 Lazy Prism Spike

Date: 2026-06-23

Decision: no-go for M33.

## Question

Can Yune replace eager `with_spelling_algebra` materialization with a lazy
prism/double-array walk while preserving current `luna_pinyin` and TypeDuck
`jyut6ping3` candidate output byte-for-byte?

## Finding

Not in this milestone. The current upstream prism parser exposes spelling
membership and descriptors, but, as in librime, it is not the candidate payload
store. Candidate payloads live on the table side, and the existing Yune
translator contract still expects those payloads to be available as owned
`Candidate` rows:

- candidate text
- candidate comment
- candidate order
- candidate quality
- sentence/prefix fallback payloads
- TypeDuck correction and profile behavior tied to table-derived candidates

The active `StaticTableTranslator` readers still depend on `entries_by_code` for
exact lookup, completion, prefix fallback, sentence segmentation, and correction
scans. Replacing that with a prism-only walk would be the wrong abstraction:
librime's design is a prism-built syllable graph plus table queries. Yune needs a
broader queryable table+prism data path, not a small lazy-load optimization.
That storage lesson is separate from the candidate-pipeline lesson: short-prefix
typing also needs bounded/lazy result production so the engine does not clone,
sort, and filter candidates the current page will not display.

## Executable evidence

Focused test:

```powershell
cargo test -p yune-core upstream_luna_pinyin_prism_fixture_does_not_contain_candidate_payloads -- --nocapture
```

Result: passed.

The test uses the checked-in upstream `luna_pinyin.prism.bin` fixture and proves
both sides of the spike boundary: `ni` is indexed as a prism spelling, but common
candidate text bytes are absent from the prism. The candidate list still has to
come from table-backed state. This does not prove lazy lookup is impossible; it
proves the next milestone must model the table and prism together.

## Consequence

M33 closes at the low-risk cache plus lazy reverse-lookup slice. Lazy lookup and
mmap should be reopened only as a future milestone with explicit byte-parity
tests for both upstream `luna_pinyin` and TypeDuck `jyut6ping3`. That milestone
must frame the work as two levers: bounded/lazy candidate production for typing
latency, and queryable table+prism storage for memory/cold-start, rather than a
prism-only rewrite.
