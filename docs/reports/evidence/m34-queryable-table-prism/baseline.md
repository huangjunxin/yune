# M34 baseline and after measurements

Date: 2026-06-23

## Commands

Before cross-engine rerun:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-yune-vs-librime.ps1 -OutputRoot docs\reports\evidence\m34-queryable-table-prism\baseline-yune-vs-librime -Iterations 9 -SessionIterations 9 -KeyIterations 25
```

Native before log:

```powershell
cmd /c "cargo bench -p yune-rime-api --bench frontend_baselines > target\m34-frontend-baselines-before.txt 2>&1"
```

Final native after log:

```powershell
cmd /c "cargo bench -p yune-rime-api --bench frontend_baselines > docs\reports\evidence\m34-queryable-table-prism\frontend-baselines-after-final.txt 2>&1"
```

Final cross-engine after rerun:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-yune-vs-librime.ps1 -OutputRoot docs\reports\evidence\m34-queryable-table-prism\after-yune-vs-librime -Iterations 9 -SessionIterations 9 -KeyIterations 25
```

The native bench logs include the existing Cargo output filename collision
warning for the `yune-rime-api` lib/cdylib outputs. The command completed
successfully.

## Native before/after

| Row | Before median | After median | Change |
| --- | ---: | ---: | ---: |
| `per_key_real_luna_pinyin_ni_full_abi` | `1,760.250 us` | `1,132.950 us` | `-35.6%` |
| `per_key_real_luna_pinyin_ni_engine_only` | `569.700 us` | `575.250 us` | `+1.0%` |
| `per_key_real_luna_pinyin_zhongguo_full_abi` | `12,697.600 us` | `12,119.013 us` | `-4.6%` |
| `per_key_real_luna_pinyin_zhongguo_engine_only` | `532.575 us` | `515.713 us` | `-3.2%` |
| `per_key_real_jyut6ping3_mobile_hai_full_abi` | `18,389.567 us` | `19,446.467 us` | `+5.7%` |
| `per_key_real_jyut6ping3_mobile_jigaajiusihaa_full_abi` | `29,937.777 us` | `28,155.585 us` | `-6.0%` |
| `per_key_real_jyut6ping3_mobile_jigaajiusihaa_correction_full_abi` | `29,649.146 us` | `28,032.915 us` | `-5.5%` |
| `startup_trace_luna_pinyin_select_schema_total` | `240,094.000 us` | `227,901.000 us` | `-5.1%` |
| `startup_trace_luna_pinyin_translator_install` | `188,404.000 us` | `176,787.000 us` | `-6.2%` |
| `startup_trace_luna_pinyin_spelling_algebra_expand` | `121,609.000 us` | `112,475.000 us` | `-7.5%` |
| `startup_trace_luna_pinyin_translator_index_build` | `11,657.000 us` | `11,289.000 us` | `-3.2%` |

`per_key_real_luna_pinyin_hao_*` is a new M34 native benchmark row. Final after:
`1,378.800 us` full ABI, `761.667 us` engine-only.

## Cross-engine before/after

| Workload | Engine | Baseline median | After median | Change |
| --- | --- | ---: | ---: | ---: |
| `hao` key sequence | Yune | `13,336.800 us` | `12,216.900 us` | `-8.4%` |
| `ni` key sequence | Yune | `5,858.800 us` | `5,693.900 us` | `-2.8%` |
| `zhongguo` key sequence | Yune | `36,451.100 us` | `35,909.100 us` | `-1.5%` |
| session create/select/destroy | Yune | `48,329.000 us` | `46,743.400 us` | `-3.3%` |
| warm startup/runtime-ready | Yune | `50,065.200 us` | `47,126.800 us` | `-5.9%` |

Final after ratios versus librime:

| Workload | Yune after | librime after | Ratio |
| --- | ---: | ---: | ---: |
| `hao` key sequence | `12,216.900 us` | `35.100 us` | `348.1x` |
| `ni` key sequence | `5,693.900 us` | `28.700 us` | `198.4x` |
| `zhongguo` key sequence | `35,909.100 us` | `1,379.400 us` | `26.0x` |
| session create/select/destroy | `46,743.400 us` | `28,121.800 us` | `1.7x` |
| warm startup/runtime-ready | `47,126.800 us` | `30,315.200 us` | `1.6x` |

The cross-engine public surface remains mixed. M34 is not a broad per-key win
against librime.
