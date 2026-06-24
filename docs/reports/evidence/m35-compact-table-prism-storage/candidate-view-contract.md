# Candidate-View Contract

M35 changed `dictionary::query_table::TableLookup` from heap-only candidate
slices into lookup views.

The core view is `LookupCandidate<'a>`:

- `text() -> &str`
- `raw_comment() -> &str`
- `raw_quality() -> f32`
- `source_hint() -> CandidateSource`
- `to_candidate() -> Candidate` for explicit materialization

Prefix lookup returns `LookupCandidateEntry<'a>` with an entry code and a
candidate view. Heap storage implements the trait by borrowing existing
`Candidate` values. Compact storage implements the same trait by borrowing
compact text/code records and synthesizing the raw comment from the canonical
code.

This keeps the public `Candidate`, `RimeCandidate`, `RimeApi`, and TypeDuck
profile ABI unchanged. Materialization now happens at selected or compatibility
boundaries:

- bounded first-page rows materialize only selected candidates;
- eager/full-list callers still receive owned `Vec<Candidate>` values;
- filters, rankers, userdb, ABI context, and candidate-list iterators preserve
  the existing owned-candidate contract.

Focused proof:

```powershell
cargo test -p yune-core lookup
```

Result: passed. The test set includes heap exact/prefix/all-code lookup,
compact-vs-heap exact/prefix/all-code lookup, correction/tolerance preservation,
and prism canonical-code lookup feeding table payload rows.
