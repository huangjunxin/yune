# M34 bounded engine refresh

Date: 2026-06-23

Implemented:

- `Engine::refresh_candidates` now requests a bounded candidate window for
  ordinary short `luna_pinyin` typing when the active state is safe:
  - schema id is `luna_pinyin`
  - input length is at most two characters
  - no rankers
  - no userdb entries
  - filters are absent or only `charset_filter`
- The initial bounded window is `DEFAULT_PAGE_SIZE + 15`.
- Out-of-window selection, highlighting, deletion, page movement, candidate-list
  iteration, and explicit full-list readers force complete eager refresh.
- Deleting a candidate from an incomplete bounded window now completes the list
  first, so later paging cannot resurrect the deleted candidate from a freshly
  materialized full list.
- `Snapshot::candidate_list_complete` lets `RimeGetContext` report
  `is_last_page = false` when the retained bounded window ends but more
  candidates may exist.
- Candidate-list iterator APIs force full list completion, preserving their
  full-list ABI contract.

Focused tests:

```powershell
cargo test -p yune-core bounded_
```

Result: passed.

Behavior guard:

- First-page `luna_pinyin` `ni` full-ABI native median improved
  `1,760.250 us` -> `1,132.950 us`.
- Full-ABI TypeDuck watch rows stayed inside the accepted guard:
  `hai` `+5.7%`, `jigaajiusihaa` `-6.0%`,
  `jigaajiusihaa_correction` `-5.5%`.

No browser/runtime claim is made. No runtime/browser files changed.
