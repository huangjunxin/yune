# M39 Task 3 Memory Owner Summary

Profiler availability is recorded in `profiler-availability.txt`: UMDH,
GFlags, and XPerf were missing; WPR was present but no repeatable allocation
stack capture/symbolization flow is part of the native benchmark. The owner
table therefore uses the deepest repeatable evidence available in this
workspace: working-set/peak rows, product-path storage status, selected
heap-mirror bytes, and M39 owner counters.

## No-Regression Rows

| Track | Row | Phase 0 median WS | Phase 2 median WS | Phase 0 peak | Phase 2 peak |
| --- | --- | ---: | ---: | ---: | ---: |
| A | `ceshiyixiachangjushuruxingnengzenyang` | `114,974,720` | `111,534,080` | `163,598,336` | `123,891,712` |
| A | `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | `116,555,776` | `111,505,408` | `163,598,336` | `123,891,712` |
| B | `neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung` | `441,483,264` | `442,511,360` | `504,557,568` | `504,057,856` |

Track A median working set and peak are lower than Phase 0. Track B peak is
lower than Phase 0; median working set is effectively flat and within the M39
5% no-regression guard.

## M39-Owned Top Owner

The M39-owned memory owner was translator install state during upstream
sentence-model construction. Before the streaming builder, the compact storage
path created a full temporary `Vec<TableEntry>` and then cloned those entries
into the sentence model. The final Task 2 checkpoint streams table entries into
`UpstreamSentenceModel::from_table_entries`, removing that full temporary table
mirror while preserving the mmap/rsmarisa selected storage path.

Selected table/prism bytes are not the heap owner: `product_path_status.csv`
records Track A `selected_storage=rsmarisa_byte_backed`, table/prism `mmap`,
`source_fallback=false`, and zero selected table/prism heap mirror bytes. Track
B records mmap-backed `byte_backed` product storage and the same zero selected
table/prism heap mirror bytes.

Final closeout must refresh these rows from `phase-4-final-native/` before M39
is moved to completed.
