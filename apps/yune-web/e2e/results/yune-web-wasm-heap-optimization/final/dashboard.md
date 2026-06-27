# Yune Web Startup Benchmark Dashboard

## Summary

| Scenario | Samples | Schema | Mode | Public | Median ready ms | p95 ready ms | Median first key ms | Transfer bytes | Encoded bytes | WASM heap bytes | Peak WASM heap bytes | Cache h/m/e |
| --- | ---: | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| tracked-luna-cold | 3 | luna_pinyin | real-worker-cold | no | 1208.0 | 1216.0 | 38.0 | 36594991 | 36593791 | 167772160 | 167772160 | 0/0/0 |
| tracked-luna-warm-reload | 3 | luna_pinyin | real-worker-warm-reload | no | 598.0 | 637.0 | 38.0 | 0 | 36875347 | 167772160 | 167772160 | 0/0/0 |
| tracked-luna-warm-new-page | 3 | luna_pinyin | real-worker-warm-new-page | no | 653.0 | 663.0 | 39.0 | 0 | 36593791 | 167772160 | 167772160 | 0/0/0 |
| tracked-jyut-cold | 3 | jyut6ping3_mobile | real-worker-cold | no | 5990.0 | 5995.0 | 47.0 | 33359094 | 33357894 | 936509440 | 936509440 | 0/0/0 |
| tracked-jyut-warm-reload | 3 | jyut6ping3_mobile | real-worker-warm-reload | no | 5317.0 | 5348.0 | 50.0 | 0 | 36666793 | 936509440 | 936509440 | 0/0/0 |
| tracked-jyut-warm-new-page | 3 | jyut6ping3_mobile | real-worker-warm-new-page | no | 5403.0 | 5654.0 | 46.0 | 0 | 33357906 | 936509440 | 936509440 | 0/0/0 |
| tracked-mock-cold | 3 | luna_pinyin | mock-worker-cold | no | 592.0 | 609.0 | 25.0 | 747840 | 746640 | 0 | 0 | 0/0/0 |
| tracked-mock-warm | 3 | luna_pinyin | mock-worker-warm | no | 379.0 | 386.0 | 26.0 | 747840 | 746640 | 0 | 0 | 0/0/0 |
| public-luna-cold | 3 | luna_pinyin | real-worker-cold | yes | 1238.0 | 1249.0 | 37.0 | 36594931 | 36593731 | 167772160 | 167772160 | 2/24/0 |
| public-jyut-cold | 3 | jyut6ping3_mobile | real-worker-cold | yes | 5859.0 | 5942.0 | 46.0 | 33359046 | 33357846 | 936509440 | 936509440 | 1/35/0 |

## Startup Owner Map

| Scenario | Top owner | Owner median ms | Ready median ms | Ready p95 ms |
| --- | --- | ---: | ---: | ---: |
| tracked-luna-cold | React/browser ready residual | 600.0 | 1208.0 | 1216.0 |
| tracked-luna-warm-reload | worker total to initialized | 539.0 | 598.0 | 637.0 |
| tracked-luna-warm-new-page | worker total to initialized | 550.0 | 653.0 | 663.0 |
| tracked-jyut-cold | worker total to initialized | 5384.0 | 5990.0 | 5995.0 |
| tracked-jyut-warm-reload | worker total to initialized | 5263.0 | 5317.0 | 5348.0 |
| tracked-jyut-warm-new-page | worker total to initialized | 5298.0 | 5403.0 | 5654.0 |
| tracked-mock-cold | React/browser ready residual | 592.0 | 592.0 | 609.0 |
| tracked-mock-warm | React/browser ready residual | 379.0 | 379.0 | 386.0 |
| public-luna-cold | worker total to initialized | 635.0 | 1238.0 | 1249.0 |
| public-jyut-cold | worker total to initialized | 5214.0 | 5859.0 | 5942.0 |

## Asset Transfer By Group

| Group | Transfer bytes | Encoded bytes | Duration ms |
| --- | ---: | ---: | ---: |
| schema binary | 0 | 16647200 | 0.0 |
| schema yaml | 0 | 4339161 | 0.0 |
| wasm binary | 0 | 2594503 | 0.0 |
| other | 491496 | 491196 | 319.0 |
| app js | 221186 | 220886 | 4.0 |
| wasm glue | 0 | 72378 | 0.0 |
| opencc | 0 | 66408 | 0.0 |
| worker script | 2425 | 42850 | -57.0 |
| app css | 32733 | 32433 | 2.0 |

## Browser Memory

| Scenario | WASM heap | Peak WASM heap | JS heap used | JS heap total | DOM nodes | Windows working set |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| tracked-luna-cold | 167772160 | 167772160 | 5529508 | 33566720 | 1095 | 744992768 |
| tracked-luna-warm-reload | 167772160 | 167772160 | 5721128 | 15835136 | 699 | 772444160 |
| tracked-luna-warm-new-page | 167772160 | 167772160 | 6491376 | 33828864 | 1095 | 721235968 |
| tracked-jyut-cold | 936509440 | 936509440 | 5824964 | 10330112 | 851 | 1361485824 |
| tracked-jyut-warm-reload | 936509440 | 936509440 | 5917256 | 13213696 | 873 | 1417658368 |
| tracked-jyut-warm-new-page | 936509440 | 936509440 | 5038492 | 10067968 | 835 | 1545764864 |
| tracked-mock-cold | 0 | 0 | 4777320 | 8757248 | 635 | 361373696 |
| tracked-mock-warm | 0 | 0 | 7754280 | 11640832 | 1779 | 411283456 |
| public-luna-cold | 167772160 | 167772160 | 6343428 | 33828864 | 1095 | 757305344 |
| public-jyut-cold | 936509440 | 936509440 | 4650156 | 10067968 | 1029 | 1363726336 |
