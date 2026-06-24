# Storage Switch

M35 added private translator storage:

```rust
enum TableStorage {
    Heap(BTreeMap<String, Vec<Candidate>>),
    Compact(Box<CompactTableStore>),
}
```

All translator lookup readers now go through storage methods for exact,
prefix, and all-code queries. Heap storage remains the fallback for source
dictionaries, unsupported compiled paths, TypeDuck profile behavior, dynamic
correction scan paths, and full compatibility behavior.

## Enabled Compact Path

`crates/yune-rime-api/src/schema_install.rs` now preserves parsed prism payloads
inside compiled dictionary load outcomes. It enables compact storage only when:

- the component is the upstream `script_translator`;
- the dictionary is `luna_pinyin`;
- the schema id is exactly `luna_pinyin`;
- a prism payload is available;
- the TypeDuck `jyut6ping3` profile guard is false.

This means compact-active upstream `luna_pinyin` consumes the compiled
dictionary as short-lived construction scratch and stores `CompactTableStore`
plus the prism payload. It does not retain heap `entries_by_code`.

## Guarded Heap Paths

TypeDuck `jyut6ping3` remains heap-backed in M35 because product-profile
invariants include rich comments, lookup records, correction behavior, long
composition, partial selection, default-confirm recomposition, and userdb
learning. Those invariants passed with heap fallback.

## Bench Harness Alignment

`crates/yune-rime-api/benches/frontend_baselines.rs` now mirrors production for
engine-only real-schema `luna_pinyin`: it loads the prism payload and uses
compact storage for the upstream profile while leaving TypeDuck and other
schemas heap-backed.
