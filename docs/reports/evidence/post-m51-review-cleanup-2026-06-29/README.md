# Post-M51 Review Cleanup Evidence

Date: 2026-06-29.

Scope: low-severity post-M50/M51 cleanup. This evidence records documentation
consistency, one behavior-preserving code cleanup, and one profile-accessor
guard. It makes no browser performance, product, package, deployment, or
iOS-device claim.

| Review issue | Resolution |
| --- | --- |
| Roadmap stale browser number | Updated the live roadmap fair `luna_pinyin` browser comparison to `64.0 MiB` versus My RIME `16.0 MiB`; updated the completed WEB-03 plan to clarify that Jyutping `160.0 MiB` is a launch guard, not a peer lane. Archived historical snapshots keep their old values. |
| Stale `docs/requirements.md` footer | Replaced the long 2026-06-28 footer with a concise 2026-06-29 note covering M50, M51, and the current fair-lane browser number. |
| Missing M50 requirement IDs | Added six M50 requirement IDs, traceability rows, coverage summary, and mapped-count update matching the measured-partial closeout. |
| Engine support contract accessor gap | Documented `rime_get_yune_windows_profile_api()` as the Windows/profile accessor for the same current profile shape without widening default `rime_get_api()`. Added focused alias and scalar-slot coverage for the Windows accessor. |
| `WordGraphEntry.code` dead field | Removed the unused field and collapsed graph construction to text plus weight. Path ordering still uses weight then text. |
| M50 diagnostic metric comparability | Added a root-cause report note that `m37_record_owned_candidate_materialization` is not directly comparable pre/post-M50 and that end-to-end benchmark rows are authoritative. |
| Commit-message hygiene | Added a conventions note asking future behavior-sensitive core commits with generic subjects to include a body or evidence pointer. |

Required verification is recorded in the final commit/push closeout.
