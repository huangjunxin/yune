# Yune vs upstream librime root-cause dashboard

Date: 2026-06-25

This report explains M39 native-engine behavior only. It does not claim browser,
frontend, product-delivery, packaging, or public-demo speed wins.

## Current Verdict

The M39 long-input failure was not a lookup-storage failure. Track A already
used the deployed `luna_pinyin` `rsmarisa` string table through mmap-backed
selected bytes. The dominant owner was upstream sentence-model composition:
each long uninterrupted pinyin row drove expensive code-prefix and vocabulary
scans before the first page was needed.

M39 fixed that owner by indexing `UpstreamSentenceModel` by code, reusing a
bounded dynamic-programming pass across end positions, and routing first-page
native requests through a limited sentence-model path. It also streamed table
entries into the sentence model at build time so the M39-owned transient memory
peak no longer holds a full temporary table-entry list beside the model.

Track B was different. The required
`neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung` row did not use
the upstream sentence model. It stayed on a TypeDuck profile path with
no-marisa exact/prefix lookup and profile fallback/full-list merge behavior.
M39 preserved and gated that path instead of applying the Track A rewrite to it.

## Bottleneck Map

| Area | M39 finding | Final status |
| --- | --- | --- |
| Track A 37-character row | Dominated by `upstream_sentence_model_ns` at `436,917.530 us/op` before implementation. | Final Yune `514.903 us`, librime `291.786 us`, ratio `1.765x`. |
| Track A 59-character row | Dominated by `upstream_sentence_model_ns` at `1,228,565.656 us/op` before implementation. | Final Yune `917.961 us`, librime `695.653 us`, ratio `1.320x`. |
| Track B 50+ Cantonese row | Separate profile owner: no upstream sentence model; no-marisa exact/prefix lookup plus profile fallback. | Final median `188.857 us/op`, p95 `194.910 us/op`, below Phase 0. |
| Storage | Track A hot path already used `rsmarisa_byte_backed` selected storage. | Preserved: table/prism `mmap`, selected heap mirrors `0`, positive `rsmarisa` counters. |
| Bounded output | Long Track A rows needed first-page bounded sentence output, not full-list materialization. | Preserved: Track A target rows show bounded requests and no full-list fallback. |
| Memory | M39-owned transient sentence-model build held duplicate table/model data. | Reduced: Track A max peak `163,598,336 B` -> `123,985,920 B`. |

## Owner Movement

| Row | Before | After |
| --- | --- | --- |
| `ceshiyixiachangjushuruxingnengzenyang` | Process key `452,200.116 us`; owner `upstream_sentence_model_ns` `436,917.530 us/op`. | Process key `514.903 us`; indexed bounded upstream sentence model `444.995 us/op`. |
| `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | Process key `1,240,080.937 us`; owner `upstream_sentence_model_ns` `1,228,565.656 us/op`. | Process key `917.961 us`; indexed bounded upstream sentence model `823.125 us/op`. |
| `neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung` | Median `189.207 us/op`; no upstream model calls. | Median `188.857 us/op`; no upstream model calls; profile fallback counted and preserved. |

## What Changed

- `UpstreamSentenceModel` stores model entries in code order and performs range
  lookup instead of scanning all entries for each prefix.
- Vocabulary lookup now keeps a first-code index so candidate expansion does not
  probe unrelated words for every substring.
- Sentence construction reuses dynamic-programming state across end positions
  and exposes a limited first-page path for bounded requests.
- `TableStorage::table_entry_iter` streams selected table entries into
  `UpstreamSentenceModel::from_table_entries`, removing the duplicated temporary
  build vector.
- M39 owner counters were added for upstream sentence-model work,
  `StaticTableTranslator::sentence_candidate`, prefix fallback, dynamic
  correction, path cloning/replacement/pruning, and bounded/full-list request
  paths.
- Sentence-candidate counters are batched per call so attribution does not
  distort the Track B profile benchmark.

## Guardrails Preserved

- Startup/session remain faster than same-run librime: startup `0.917x`,
  session `0.938x`.
- `hao`, `ni`, and `zhongguo` remain inside their gates: `3.281x`, `3.863x`,
  and `0.329x`.
- Track A selected storage remains `rsmarisa_byte_backed`, with table/prism
  `mmap`, selected heap mirrors `0`, `source_fallback=false`, and positive
  runtime `rsmarisa` exact/prefix counters.
- Track B selected storage remains byte-backed and mmap-backed, with selected
  heap mirrors `0`.
- Upstream-observable behavior and TypeDuck rich-comment boundary behavior are
  covered by focused tests and the full workspace test suite.

## Remaining Caveats

Yune still has a larger whole-process memory footprint than librime in absolute
terms. M39 did not claim memory parity; it added heap-owner attribution, reduced
the M39-owned transient Track A peak, and proved no regression against the
post-M38 baseline.

The Track B profile still reports `rsmarisa_status=missing_string_table` because
the selected product path uses Yune-readable byte-backed compact storage for
the current TypeDuck profile artifacts. That is a profile storage fact, not a
Track A regression.

Future browser or product-delivery claims require separate rebuilt runtime and
real-browser evidence. M39 evidence is native-engine evidence only.
