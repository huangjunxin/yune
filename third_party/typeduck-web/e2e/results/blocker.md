# Browser E2E Blocker / Supersession Notes

**Date**: 2026-06-18

The earlier WI-4 evidence in this directory used Yune's echo placeholder path and is superseded for the real-assets candidate gate by HR-1b. The refreshed artifacts are:

- `browser-run.log` — focused HR-1b real-assets browser smoke.
- `browser-console.json` — console evidence for reload/init and `processKey({n/e/i})`.
- `dom-snapshot-candidates.txt` — candidate panel DOM excerpt showing `nei -> 你 / 呢 / 尼`.
- `screenshot-real-assets-nei.png` — browser screenshot of the real candidate panel.

**Current status**: HR-1b PASS for real-assets candidate rendering.

**Still open**:

- HR-2: `setOption` still throws in the adapter/runtime path and causes option error toasts.
- HR-3: `deploy()` must be made to return true with real assets.
- HR-4/HR-5: live persistence, reload survival, paging/deletion, and dictionary-panel oracle comment bytes still need the full real-assets E2E matrix.
