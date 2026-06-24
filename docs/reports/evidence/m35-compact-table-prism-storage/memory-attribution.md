# M35 Memory Attribution

M35 separates dictionary-specific deltas from whole-process high-water marks.
The main upstream owner before implementation was the heap-expanded
`luna_pinyin` spelling-algebra/index path, not the compiled file size itself.

## Compiled Asset Sizes

Fair upstream `luna_pinyin` benchmark assets from `target/yune-vs-librime-benchmark/.../yune/user/build`:

| Asset | Bytes |
| --- | ---: |
| `luna_pinyin.table.bin` | `13013460` |
| `luna_pinyin.prism.bin` | `31728` |
| `luna_pinyin.reverse.bin` | `252756` |
| `stroke.table.bin` | `4548476` |
| `stroke.prism.bin` | `3501372` |
| `stroke.reverse.bin` | `1498516` |

TypeDuck-Web product/browser schema assets:

| Asset | Bytes |
| --- | ---: |
| `jyut6ping3.table.bin` | `4306860` |
| `jyut6ping3.reverse.bin` | `70012` |
| `jyut6ping3_mobile.prism.bin` | `242728` |
| `jyut6ping3.dict.yaml` | `3594190` |
| `jyut6ping3_scolar.table.bin` | `6115656` |
| `jyut6ping3_scolar.prism.bin` | `2343228` |
| `jyut6ping3_scolar.reverse.bin` | `3568716` |
| `jyut6ping3_scolar.dict.yaml` | `7254777` |
| `cangjie5.table.bin` | `1509778` |
| `cangjie5.prism.bin` | `1430557` |
| `cangjie5.reverse.bin` | `663901` |
| `luna_pinyin.table.bin` | `613855` |
| `luna_pinyin.prism.bin` | `23465` |
| `luna_pinyin.reverse.bin` | `704357` |

Source row counts:

| Dictionary | Rows |
| --- | ---: |
| fair-harness upstream `luna_pinyin.dict.yaml` | `70805` |
| TypeDuck-Web `jyut6ping3.dict.yaml` | `127144` |
| TypeDuck-Web `luna_pinyin.dict.yaml` | `48970` |
| TypeDuck-Web `cangjie5.dict.yaml` | `42955` |

## Native Startup Deltas

Upstream `luna_pinyin` before and after compact storage:

| Startup span | Before median | Before delta | After median | After delta | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| `compiled_table_load` | `7102.000us` | `7704576` | `4983.800us` | `7741440` | parse scratch remains |
| `compiled_prism_load` | `95.200us` | `708608` | `81.600us` | `708608` | unchanged |
| `compiled_reverse_load` | `4835.800us` | `3452928` | `3934.200us` | `3452928` | unchanged size class |
| `translator_index_build` | `14500.800us` | `5869568` | `17084.000us` | `-77824` | heap map no longer retained for compact path |
| `spelling_algebra_expand` | `148570.200us` | `17784832` | `122.200us` | `0` | expanded alias heap removed |
| `translator_install` | `233169.800us` | `37556224` | `55155.800us` | `9822208` | retained dictionary delta cut by `73.8%` |
| `select_schema_total` | `295027.400us` | `25026560` | `104363.600us` | `-2613248` | scratch dropped before ready-state sample |

TypeDuck `jyut6ping3_mobile` remains on heap fallback because lookup records,
rich comments, partial selection, default-confirm recomposition, and userdb
learning are product-profile invariants. The TypeDuck startup trace still shows
large heap expansion:

| Startup span | After median | After delta |
| --- | ---: | ---: |
| `startup_trace_jyut6ping3_mobile_spelling_algebra_expand` | `4762494.600us` | `679698432` |
| `startup_trace_jyut6ping3_mobile_translator_install` | `5112314.200us` | `751448064` |

## Whole-Process Peak

The fair cross-engine whole-process Yune peak changed from `182910976` bytes to
`182444032` bytes (`-466944`, about `-0.3%`). That is not the M35 win surface.
The measured M35 win is dictionary-specific retained delta in the native
upstream `luna_pinyin` schema path. The remaining fair-harness peak owner is
closed as a borrowed/mmap/demand-paging follow-up, not as a browser or public
delivery claim.
