# M43 Phase 0 Verdict

Branch selected: `memory-owner-reduction`.

Phase 0 named a single non-overlapping heap-owned reducible owner above the branch threshold: `poet.entries_by_code` retained 38,208,541 B for Track A `luna_pinyin`, above the 10 MB single-owner trigger. The selected fix is bounded to packing sentence-model entry text/code storage and preserving the M40 lookup index ordering semantics.

Excluded from the branch trigger:

- `compact_table.storage`: 13,013,460 B, `mmap_file_backed`, excluded from heap-owned triggers.
- `poet.lookup_index`: 2,660,848 B, `heap_owned_guarded`, preserved for M40 full-pinyin sentence lookup.
- `schema.config`: 1,864 B, `overlap_estimate`, logical reload-signature bytes only.
- `translator.entries_by_code`: shared zero-byte compact-table reference on Track A.

Phase 0 Track A noise band: peak 127574016-127647744 B; hao 38.400-39.267 us; ni 57.050-57.950 us.

Short-key owner profile was captured but did not select Branch B: `hao`/`ni` remain dominated by translator production after raw prism/table timing, while the memory owner trigger is larger and safer to change first.

M42 abbreviation rows `cszysmsrsd` and `zybfshmsru` passed the Phase 0 native oracle-vs-Yune candidate-output guard. Their latency remains outside M43 implementation scope.
