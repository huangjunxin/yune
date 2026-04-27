# Roadmap

## Completed

### M0: Skeleton

- Rust workspace.
- Core session and candidate types.
- CLI smoke test.
- Initial analysis and architecture notes.

### M1: Compatibility Harness

- Recorded deterministic fixtures, including checked-in composing-state output.
- Run Yune fixtures from CLI.
- Defined JSON output for context, candidates, commit, and status.
- Added workspace tests for deterministic behavior.

### M2: Schema Subset

- Parse a RIME-style schema subset.
- Model processors, segmentors, translators, and filters as named components.
- Support minimal punctuation and echo translation.
- Support selected RIME config patch/include behavior used by compatibility tests.

### M3: Dictionary Prototype

- Implement a simple table dictionary format.
- Support deterministic lookup and candidate ranking.
- Add fixture coverage for pinyin and shape-based schemas.

### M4: AI Hook

- Add a non-blocking candidate reranking trait.
- Provide a mock ranker for tests.
- Keep classic candidate ordering as fallback.

### M5: RIME Frontend Shim

- Implemented a focused RIME-style C ABI subset for sessions, context, status,
  commit, config, levers, schema lists, deployment helpers, modules, runtime
  options/properties, and key processing.
- Added librime-compatible key-table lookup coverage for broad X11 keysym name
  groups, including function keys, keypad keys, modifiers, ISO/XKB/dead keys,
  input-method keys, Latin, kana, Arabic, Cyrillic, Greek, technical,
  publishing/APL, and Hebrew names.
- Aligned core and ABI key handling for navigation, editing, selection, keypad
  keys, Return variants, modifier fallbacks, and
  `menu/alternative_select_keys`.
- Extended simulated key-sequence parsing so known librime key names either
  process as printable characters or parse as ignored no-op events instead of
  failing.
- Validated the compatibility surface with focused Rust tests.

## Next

- Compare current ABI behavior against real frontend clients or a dedicated
  compatibility test client.
- Broaden schema and dictionary coverage beyond the current fixtures.
- Expand data compatibility toward more RIME dictionary and userdb formats.
- Keep AI ranking and completion optional, local-first, and behind classic input
  behavior.
