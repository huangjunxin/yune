# M34 compiled table query gate

Date: 2026-06-23

Compiled table storage was not enabled in M34.

Stop-gate findings:

- Current compiled table parsers produce owned `TableDictionary` data and are
  not yet a borrowed/queryable runtime reader.
- The current eager translator still owns correction, tolerance, sentence,
  TypeDuck lookup records, and ordering behavior through the heap-backed
  `entries_by_code` shape.
- A compiled table query path must preserve text bytes, comment bytes, quality,
  order, code, stems/encoder data, correction/tolerance payloads, and TypeDuck
  lookup records before it can replace the heap map.
- Unsupported table sections must be structured no-go errors, not silent
  fallback to changed output.

Decision:

M34 stops at the internal lookup abstraction. The compiled table query path is
deferred until it can be A/B tested against the heap map for exact lookup,
prefix completion, sentence, correction, and TypeDuck profile fixtures.
