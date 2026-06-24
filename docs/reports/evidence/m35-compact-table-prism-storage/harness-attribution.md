# Harness Attribution

M35 keeps native engine-only, native full-ABI, fair cross-engine, memory, and
browser-delivery claims separate.

## Native Before/After

| Row | Before median | After median | Change |
| --- | ---: | ---: | ---: |
| `luna_pinyin_hao_full_abi` | `2034.769us` | `1411.302us` | `-30.6%` |
| `luna_pinyin_hao_engine_only` | `1092.879us` | `750.517us` | `-31.3%` |
| `luna_pinyin_ni_full_abi` | `1535.097us` | `1252.294us` | `-18.4%` |
| `luna_pinyin_ni_engine_only` | `891.791us` | `697.044us` | `-21.8%` |
| `luna_pinyin_zhongguo_full_abi` | `14759.755us` | `1527.055us` | `-89.7%` |
| `luna_pinyin_zhongguo_engine_only` | `740.966us` | `485.482us` | `-34.5%` |

These are the safest M35 engine movement claims. The `hao`/`ni` engine-only
stretch target of low hundreds of microseconds was not met; the remaining owner
is per-key translation/context overhead after compact lookup, plus the existing
benchmark's full candidate/result behavior.

## Fair Cross-Engine Before/After

| Workload | Yune baseline | Yune after | Change | librime after | After ratio |
| --- | ---: | ---: | ---: | ---: | ---: |
| `hao` | `15906.800us` | `12547.200us` | `-21.1%` | `35.400us` | `354.4x` |
| `ni` | `9225.100us` | `5678.500us` | `-38.4%` | `28.700us` | `197.9x` |
| `zhongguo` | `45608.600us` | `35848.500us` | `-21.4%` | `1452.800us` | `24.7x` |
| session create/select/destroy | `67119.100us` | `47806.600us` | `-28.8%` | `30977.000us` | `1.5x` |
| startup/runtime-ready | `66709.400us` | `46516.200us` | `-30.3%` | `31052.200us` | `1.5x` |

The fair cross-engine harness still includes C#/P/Invoke/context/memory
sampling costs and remains unsuitable as the main raw typing-latency headline.
It is useful as a compatibility-harness trend: Yune improved across watched
rows, but it is still much slower than librime on per-key workloads.

## Browser/Delivery

No runtime, TypeScript, TypeDuck-Web source, WASM, Cloudflare, or browser UI
file changed in M35. No browser startup, browser typing, public delivery, or
Cloudflare claim is made.
