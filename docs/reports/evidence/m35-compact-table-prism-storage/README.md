# M35 Compact Table Storage Evidence

This folder contains the checked-in evidence for M35 compact table+prism
runtime storage.

Primary report:

- [`../../yune-vs-librime-performance.md`](../../yune-vs-librime-performance.md)

Benchmark inputs:

- [`frontend-baselines-before.txt`](./frontend-baselines-before.txt)
- [`frontend-baselines-after.txt`](./frontend-baselines-after.txt)
- [`baseline-yune-vs-librime/`](./baseline-yune-vs-librime/)
- [`after-yune-vs-librime/`](./after-yune-vs-librime/)

Visualizations generated from the final M35 evidence:

- [`m35-native-improvement.svg`](./m35-native-improvement.svg) - native
  upstream `luna_pinyin` watched-row before/after movement.
- [`m35-cross-engine-gap.svg`](./m35-cross-engine-gap.svg) - final fair
  Yune/librime median latency gap.
- [`m35-memory-story.svg`](./m35-memory-story.svg) - dictionary-local memory
  win versus unresolved whole-process peak.

Task evidence:

- [`baseline.md`](./baseline.md)
- [`memory-attribution.md`](./memory-attribution.md)
- [`reader-audit.md`](./reader-audit.md)
- [`stop-conditions.md`](./stop-conditions.md)
- [`candidate-view-contract.md`](./candidate-view-contract.md)
- [`compact-table-reader.md`](./compact-table-reader.md)
- [`storage-switch.md`](./storage-switch.md)
- [`prism-table-integration.md`](./prism-table-integration.md)
- [`typeduck-guard.md`](./typeduck-guard.md)
- [`mmap-gate.md`](./mmap-gate.md)
- [`harness-attribution.md`](./harness-attribution.md)
- [`task-7-gates.md`](./task-7-gates.md)
