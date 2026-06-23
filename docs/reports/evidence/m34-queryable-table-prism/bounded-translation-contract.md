# M34 bounded translation contract

Date: 2026-06-23

Implemented:

- `CandidateRequest` and `TranslationResult` in `yune-core`.
- `Translator::translate_with_context_and_request(...)` as an internal bounded
  request hook with the eager path as the default compatibility wrapper.
- `StaticTableTranslator` bounded support for the safe subset: no correction,
  no dynamic correction, no sentence mode, no prefix fallback, no
  prediction-never-first, no prediction limit, no tolerance/correction specs,
  no combine-candidates, and no required syllable-count specs.
- Complete code-ordered prefix enumeration with bounded expensive
  materialization. The bounded path keeps references plus quality/emission-order
  metadata and materializes only the selected top window.
- Stable tie behavior by preserving emission order after quality comparison.

Not implemented:

- Early-stop prefix enumeration. Current code-ordered map cannot prove that a
  high-quality completion will not appear later in lexicographic order.
- Bounded correction, dynamic correction, sentence, prefix fallback, or TypeDuck
  profile paths.
- Public ABI changes. `RimeApi`, `RimeCandidate`, and TypeDuck profile layout
  were untouched.

Focused tests:

```powershell
cargo test -p yune-core bounded_
```

Result: passed.
