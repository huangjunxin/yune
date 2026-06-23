# M34 prism/table integration gate

Date: 2026-06-23

Prism-backed candidate lookup was not enabled in M34.

Stop-gate findings:

- Existing prism parsing can expose spelling/syllable graph data.
- Prism payloads do not carry candidate text/comment/order bytes.
- A byte-identical lookup therefore needs prism data to discover spellings and
  table data to supply candidate payloads.
- Current `luna_pinyin` per-key rows do not have a proven query-time
  spelling-algebra owner; spelling expansion remains mainly construction-time.

Decision:

No prism-only performance claim is safe. Prism/table integration remains future
storage work behind table payload query parity.
