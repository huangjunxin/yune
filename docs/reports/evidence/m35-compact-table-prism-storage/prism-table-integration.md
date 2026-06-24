# Prism/Table Integration

M35 added `RimePrismBinPayload::lookup_canonical_codes(...)`.

The prism lookup:

- uses the parsed Darts double-array to resolve the typed spelling;
- maps spelling descriptors to syllabary/canonical table codes;
- reports abbreviation/correction metadata for future ranking work;
- does not carry candidate payloads.

The table lookup supplies candidate payloads. The compact translator uses the
prism only to add canonical table-code lookup specs for spelling-algebra schemas.
Those specs distinguish:

- the canonical fetch code used to read compact table payloads;
- the typed lookup code used for materialization, comment/preedit behavior, and
  exact-vs-completion source classification.

This avoids the M34 no-go where prism-only lookup could discover spellings but
could not emit candidates, and it avoids rebuilding expanded heap aliases for
compact-active upstream `luna_pinyin`.

Proof:

```powershell
cargo test -p yune-core lookup
cargo test -p yune-core --test upstream_luna_pinyin_parity
```

Result: passed. `zhongguo` full-ABI native median improved from
`14759.755us` to `1527.055us`, showing the spelling-algebra storage path is now
hot for the watched upstream row.
