# M34 storage hot-path swap decision

Date: 2026-06-23

Storage hot-path replacement was deferred.

Reasons:

- Lever A produced a narrow, lower-risk first-page full-ABI win without touching
  storage representation.
- Dynamic correction and TypeDuck profile behavior still rely on full or
  global table state.
- Sentence, prefix fallback, prediction-never-first, and other global behaviors
  still require eager fallback until their bounded semantics are proven.
- Current compiled table/prism readers are not yet the queryable borrowed
  representation needed to remove heap `entries_by_code`.

Retained state:

- `StaticTableTranslator.entries_by_code` remains live.
- No retained table state was deleted.
- `TableLookup` is the only storage-abstraction step landed.
