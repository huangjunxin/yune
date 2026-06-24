# Compact Table Reader

M35 added `CompactTableStore` in `crates/yune-core/src/dictionary/compiled_table.rs`.

The reader consumes a parsed `TableDictionary` into compact owned arrays:

- first-occurrence `syllabary_codes`;
- sorted code groups with entry ranges;
- compact entries containing text and raw weight;
- advanced payload data for corrections, tolerance, stems, encoder, lookup
  records, and preset vocabulary.

Queries implemented through `TableLookup`:

- `has_code`
- `exact_candidates`
- `prefix_candidates`
- `all_codes`

The compact reader does not retain `Vec<Candidate>` for every row. It emits
`LookupCandidate` views and materializes owned `Candidate` only at translator
compatibility boundaries.

Proof:

```powershell
cargo test -p yune-core lookup
cargo test -p yune-core --test upstream_luna_pinyin_parity
```

Results:

- compact exact/prefix/all-code output matches heap lookup in focused tests;
- correction and tolerance payloads survive compact parsing;
- upstream `luna_pinyin` parity passed after compact schema enablement;
- unsupported TypeDuck compact enablement was not accepted in M35.
