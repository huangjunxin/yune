# M34 table lookup abstraction

Date: 2026-06-23

Implemented:

- New internal `crates/yune-core/src/dictionary/query_table.rs`.
- Internal `TableLookup` trait with:
  - `has_code`
  - `exact_candidates`
  - `prefix_candidates`
  - `all_codes`
- Heap-backed implementation for `BTreeMap<String, Vec<Candidate>>`.
- Concrete iterator structs avoid boxed iterators in the current hot path.

Focused test:

```powershell
cargo test -p yune-core heap_table_lookup_exposes_exact_prefix_and_all_code_queries
```

Result: passed.

The abstraction is intentionally internal. It does not change public
`Candidate`, public C ABI structs, or TypeDuck profile ABI. It is the safe
precondition for later storage swaps; no compiled storage swap was mixed into
this step.
