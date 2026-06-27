# WEB-01 Post-M45 Reconciliation

Date: 2026-06-27

This run is labelled post-M45. M45 changed only native benchmark reporting in
the committed code path, so WEB-01 treats inherited engine/runtime state as the
baseline and does not claim native memory movement.

M41 closed tracked `jyut6ping3_mobile` startup around `1,254 ms` and public-demo
startup around `1,291 ms`, with `128.0 MiB` WASM linear memory. The WEB-01
post-M45 comparator and attribution runs now see Jyutping around `5.8-6.0 s`
ready-to-input and `893.1 MiB` WASM linear memory.

The difference is a current-runtime/refreshed-artifact state and benchmark-shape
change, not a WEB-01 optimization regression:

- WEB-01 rebuilt the current local WASM/runtime artifacts before measuring.
- The comparator measures the current browser production outputs plus My RIME
  live rows, not only the M41 startup harness.
- Attribution proves the current Jyutping high-water is reached even when the
  requested shared asset family is reduced to `extras` (`0 B` requested family
  bytes, `6.6 MiB` unique encoded resources).

Therefore WEB-01 uses `893.1 MiB` as the current post-M45 browser target while
keeping M41 as historical pre-refresh startup evidence.
