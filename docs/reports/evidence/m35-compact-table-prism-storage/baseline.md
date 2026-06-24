# M35 Baseline

Date: 2026-06-24.

Commit at baseline capture: `3a5f2a1ad79057b42a4922ba8b92bb727c6c6a35`.

Commands:

```powershell
cmd /c "cargo bench -p yune-rime-api --bench frontend_baselines > docs\reports\evidence\m35-compact-table-prism-storage\frontend-baselines-before.txt 2>&1"
powershell -ExecutionPolicy Bypass -File scripts\benchmark-yune-vs-librime.ps1 -OutputRoot docs\reports\evidence\m35-compact-table-prism-storage\baseline-yune-vs-librime -Iterations 9 -SessionIterations 9 -KeyIterations 25
```

Raw evidence:

- Native baseline: [`frontend-baselines-before.txt`](./frontend-baselines-before.txt)
- Fair baseline: [`baseline-yune-vs-librime/summary.csv`](./baseline-yune-vs-librime/summary.csv)

## Native Baseline Rows

| Row | Median | P95 | Memory note |
| --- | ---: | ---: | --- |
| `per_key_real_luna_pinyin_hao_full_abi` | `2034.769us` | `2618.233us` | full ABI/context |
| `per_key_real_luna_pinyin_hao_engine_only` | `1092.879us` | `1493.167us` | engine snapshot excluded |
| `per_key_real_luna_pinyin_ni_full_abi` | `1535.097us` | `1941.600us` | full ABI/context |
| `per_key_real_luna_pinyin_ni_engine_only` | `891.791us` | `1170.000us` | engine snapshot excluded |
| `per_key_real_luna_pinyin_zhongguo_full_abi` | `14759.755us` | `15190.288us` | full ABI/context |
| `per_key_real_luna_pinyin_zhongguo_engine_only` | `740.966us` | `887.050us` | engine snapshot excluded |
| `per_key_real_jyut6ping3_mobile_hai_full_abi` | `18900.742us` | `20472.900us` | TypeDuck guard row |
| `per_key_real_jyut6ping3_mobile_jigaajiusihaa_full_abi` | `28836.874us` | `32281.823us` | TypeDuck guard row |
| `per_key_real_jyut6ping3_mobile_jigaajiusihaa_correction_full_abi` | `24811.675us` | `30640.977us` | TypeDuck correction guard |

Key startup owner rows for upstream `luna_pinyin`:

| Row | Median | Memory delta |
| --- | ---: | ---: |
| `startup_trace_luna_pinyin_compiled_table_load` | `7102.000us` | `7704576 bytes` |
| `startup_trace_luna_pinyin_compiled_prism_load` | `95.200us` | `708608 bytes` |
| `startup_trace_luna_pinyin_compiled_reverse_load` | `4835.800us` | `3452928 bytes` |
| `startup_trace_luna_pinyin_translator_index_build` | `14500.800us` | `5869568 bytes` |
| `startup_trace_luna_pinyin_spelling_algebra_expand` | `148570.200us` | `17784832 bytes` |
| `startup_trace_luna_pinyin_translator_install` | `233169.800us` | `37556224 bytes` |
| `startup_trace_luna_pinyin_select_schema_total` | `295027.400us` | `25026560 bytes` |

## Fair Cross-Engine Baseline

| Workload | Yune median | Yune p95 | Yune ready WS | Yune peak WS | librime median | librime peak WS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `hao` | `15906.800us` | `18549.100us` | `171671552` | `182910976` | `36.300us` | `22417408` |
| `ni` | `9225.100us` | `9811.900us` | `171610112` | `182910976` | `29.900us` | `22302720` |
| `zhongguo` | `45608.600us` | `48480.400us` | `171548672` | `182910976` | `2502.200us` | `22450176` |
| session create/select/destroy | `67119.100us` | `68009.800us` | `170422272` | `182910976` | `42440.300us` | `22302720` |
| startup/runtime-ready | `66709.400us` | `1247752.600us` | `170246144` | `182910976` | `41553.100us` | `22249472` |

Baseline conclusion: M35 starts with a retained upstream `luna_pinyin` spelling-algebra/storage delta in native startup traces, while whole-process fair-harness peak remains about `182.9 MB`.
