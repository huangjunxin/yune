# M49 Track A Short-Key Latency Follow-Up

> Status: Completed partial - Milestone: M49 - Closed: 2026-06-29 - Type: native
> engine performance follow-up

## Scope

Native Track A only:

- `luna_pinyin` native peer comparison against same-run upstream librime 1.17.0.
- Preserve M40/M42/M43/M44/M45 behavior and storage guards.
- No web harness, frontend, packaging, deployment, public-demo, iOS-device, or
  broad product-speed claim.

Success required the strict `<=3.0x` ratio gate for the tracked latency rows.
This pass is therefore closed as partial.

## Changes Retained

1. **MARISA prefix traversal constant-factor reduction**
   - Compact MARISA prefix traversal now stores the current code on traversal
     frames.
   - Matching node entry rows are yielded lazily through an entry cursor instead
     of materializing every row for the node into a pending vector before the
     bounded caller can stop.

2. **Preset-vocabulary transient prefilter**
   - The normal sentence path keeps the existing first-code vocabulary index.
   - It adds a transient character-code prefilter before expensive phrase-code
     derivation, avoiding the rejected retained prefix index.

## Rejected Direction

A retained two/three-code preset-vocabulary prefix index fixed the long rows but
added about `35 MB` retained heap (`poet.vocabulary_prefix_index`). It was not
kept because M47 made memory honesty a hard constraint and Track A should not
trade a latency partial for a large retained heap owner.

## Final Evidence

Evidence root:
[`docs/reports/evidence/m49-track-a-short-key-latency/`](../../reports/evidence/m49-track-a-short-key-latency/)

Final benchmark:
[`docs/reports/evidence/m49-track-a-short-key-latency/final-native-benchmark/`](../../reports/evidence/m49-track-a-short-key-latency/final-native-benchmark/)

| Row | Baseline | Final | Verdict |
| --- | ---: | ---: | --- |
| `n` | `71.300 us` / `3.478x` | `62.400 us` / `3.074x` | improved, still blocker |
| `ni` | `51.000 us` / `3.617x` | `46.250 us` / `3.269x` | improved, still blocker |
| `hao` | `25.233 us` / `2.200x` | `26.300 us` / `2.248x` | pass |
| 37-char pinyin | `2,789.897 us` / `9.670x` | `894.400 us` / `3.094x` | improved, still blocker |
| 59-char pinyin | `5,064.307 us` / `7.512x` | `1,543.742 us` / `2.280x` | pass |

Current full `luna_pinyin` Track A memory high-water is `188.3 MB` versus
librime `17.6 MB` in the same final run. This is the post-M48 full
preset-vocabulary peer-comparison shape, not the M47 TypeDuck keyboard-profile
iOS proxy.

## Verification

Passed:

- `cargo fmt --check`
- `cargo test -p yune-core dictionary:: -- --nocapture`
- `cargo test -p yune-core short_key`
- `cargo test -p yune-core poet`
- `cargo test -p yune-core --test upstream_luna_pinyin_parity`
- final native benchmark above

## Remaining Work

1. Attribute and reduce the remaining short-prefix translator/prefix constant
   factor for `n` and `ni`.
2. Reduce the 37-character preset-vocabulary graph rebuild cost without adding
   retained heap.
3. Attribute the post-M48 full `luna_pinyin` Track A memory peak/private shape.
