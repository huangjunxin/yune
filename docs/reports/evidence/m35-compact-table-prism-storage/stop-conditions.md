# M35 Stop Conditions

These stop conditions were applied during implementation:

- Stop if candidate-view lookup cannot reproduce heap eager exact/prefix/all-code behavior.
- Stop if compact table payload lookup cannot preserve text, raw code/comment, weight/order, corrections, tolerance rules, and TypeDuck lookup-record payloads.
- Stop if prism lookup is used as candidate payload storage. M35 allows prism only to discover canonical codes; table storage supplies payloads.
- Stop if compact-active schemas build or retain heap `entries_by_code` at ready state.
- Stop if spelling-algebra parity is recovered by materializing expanded heap aliases for compact-active schemas.
- Stop if TypeDuck rich comments, lookup records, composition, partial selection, default-confirm recomposition, or userdb learning diverge.
- Stop if mmap/borrowed storage would require unsafe/lint exceptions before compact owned storage is active and measured.

Outcomes:

- Candidate-view and compact table tests passed.
- Upstream `luna_pinyin` compact storage is active with prism canonical-code lookup and without retained heap `entries_by_code`.
- TypeDuck compact storage is a no-go in M35; heap fallback remains active and byte-identical.
- Mmap is deferred by measurement gate. Compact owned storage removes the upstream spelling-algebra heap delta, but whole-process fair-harness peak remains a separate demand-paging/borrowed-storage owner.
