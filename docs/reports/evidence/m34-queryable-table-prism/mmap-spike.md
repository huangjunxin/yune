# M34 mmap/borrowed storage gate

Date: 2026-06-23

Mmap/borrowed storage was not attempted.

Decision:

- Mmap is storage work, not a standalone typing-latency fix.
- The runtime hot path must first walk a queryable table/prism representation.
  Mapping bytes and then rebuilding heap maps would not solve the measured
  candidate-pipeline owner.
- No browser demand-paging claim is made. Browser/WASM delivery and cache
  mechanics remain M31 work unless browser evidence proves otherwise.
- No `unsafe` exception was needed or added.

Follow-up:

Reopen mmap only after table/prism query parity proves that the runtime can
borrow or index compact storage directly and after Windows file lifetime/rebuild
behavior is covered.
