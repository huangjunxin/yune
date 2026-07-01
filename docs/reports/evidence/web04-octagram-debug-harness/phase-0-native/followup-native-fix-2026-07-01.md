# WEB-04 Phase 0 Native Fix Follow-Up

Date: 2026-07-01.

Verdict: **green for Task 0**. The native/product-path blocker recorded in
`fe1c6cd5` is resolved for the WEB-04 top-candidate gate. Browser work was not
started in this slice.

## Native Fix

The fix is intentionally narrow:

- Existing exact `schema/schema_id == "luna_pinyin"` behavior remains allowed.
- Exact `schema/schema_id == "luna_pinyin_octagram"` is additionally allowed
  through the upstream Luna quality path only when the translator is
  `script_translator`, the dictionary is `luna_pinyin`, and `grammar/language`
  is present and passes logical resource-id validation.
- Random `luna_pinyin_*` names are not swept into this path.

The regression tests cover:

- Named WEB-04 profile `luna_pinyin_octagram` receives the Luna quality path and
  octagram grammar.
- Random `luna_pinyin_experimental` does not automatically receive that path.
- Plain `luna_pinyin` without `grammar/language` keeps null-grammar behavior.

## Product-Path Rerun

Temporary Yune shared data:
`target/web04-octagram-debug-harness/phase-0-native-followup/shared`.

The temp profile was `luna_pinyin_octagram.schema.yaml` with inline
`grammar/language: zh-hant-t-essay-bgw`. No shared `grammar.yaml` was added.

Pinned model:

- Source: `lotem/rime-octagram-data`
- License: `LGPL-3.0`
- Branch/commit: `hant`
  `bb8e1313552f0f27f2f968031dfaf4563e55d982`
- Model: `zh-hant-t-essay-bgw.gram`
- URL:
  `https://raw.githubusercontent.com/lotem/rime-octagram-data/bb8e1313552f0f27f2f968031dfaf4563e55d982/zh-hant-t-essay-bgw.gram`
- Size: `10513408` bytes
- SHA256:
  `574c99d100f422766c433c601ed6efd642e881d69a30df9fffb6f1695be550e3`

Command shape:

```powershell
target\debug\yune-cli.exe frontend `
  --shared-data-dir target\web04-octagram-debug-harness\phase-0-native-followup\shared `
  --user-data-dir target\web04-octagram-debug-harness\phase-0-native-followup\user\direct-luna_pinyin_octagram `
  --schema luna_pinyin_octagram `
  --sequence <input>
```

The fresh librime oracle rows are the WEB-04 Task 0 same-run rows recorded in
`target/web04-octagram-debug-harness/phase-0-native/oracle-output/`.

## Result

| Input | Fresh librime octagram top | Yune `luna_pinyin_octagram` top | Match |
| --- | --- | --- | --- |
| `youhuiyong` | `優惠用` | `優惠用` | yes |
| `jintianhuiyi` | `今天會議` | `今天會議` | yes |
| `jintianwanshangyouhui` | `今天晚上又會` | `今天晚上又會` | yes |
| `gegeguojiayougegeguojiadeguoge` | `各個國家有各個國家的國歌` | `各個國家有各個國家的國歌` | yes |

Yune still has known first-page ordering differences outside the M54/WEB-04
accepted top-candidate gate, so this evidence compares the accepted top
candidate.

Plain `luna_pinyin` was rerun in the mixed shared-data root and a plain-only
control root. The top-5 lists matched exactly for all six WEB-04 inputs, proving
the dedicated profile/model did not turn on grammar for plain Luna.

Machine-readable follow-up evidence is in
`followup-native-fix-2026-07-01.json`.

## Boundaries

- No browser implementation was started.
- No `apps/yune-web` asset plumbing was started.
- No `packages/yune-web-runtime` change was made.
- No public C ABI was changed.
- No third-party `.gram` bytes were committed.
