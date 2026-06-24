# Mmap / Borrowed Storage Gate

M35 did not attempt mmap.

Gate result:

- Compact owned storage is now active for upstream `luna_pinyin`.
- Native dictionary-specific deltas improved materially:
  - `spelling_algebra_expand` median `148570.200us` -> `122.200us`;
  - `spelling_algebra_expand` memory delta `17784832` -> `0`;
  - `translator_install` memory delta `37556224` -> `9822208`.
- Whole-process fair-harness peak did not meet the stretch target:
  - Yune peak `182910976` -> `182444032` bytes;
  - librime peak after-run remains about `22437888` bytes.

No unsafe code, lint exception, Windows file-lifetime policy, or mmap crate was
introduced. The remaining peak-memory owner is not safe to hide behind an mmap
claim because TypeDuck product schemas still use heap fallback and the fair
harness whole-process high-water includes runtime/process overhead beyond the
upstream dictionary-specific delta.

Decision: mmap/borrowed storage is closed by no-go for M35 and deferred to a
separate measured design that can cover byte borrowing, rebuild invalidation,
Windows file lifetime, and demand-paged table payloads.
