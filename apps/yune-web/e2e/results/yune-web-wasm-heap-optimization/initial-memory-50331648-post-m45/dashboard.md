# Yune Web Startup Benchmark Dashboard

## Summary

| Scenario | Samples | Schema | Mode | Public | Median ready ms | p95 ready ms | Median first key ms | Transfer bytes | Encoded bytes | WASM heap bytes | Peak WASM heap bytes | Cache h/m/e |
| --- | ---: | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| tracked-luna-cold | 3 | luna_pinyin | real-worker-cold | no | 1239.0 | 1332.0 | 34.0 | 36594991 | 36593791 | 184549376 | 184549376 | 0/0/0 |
| tracked-luna-warm-reload | 3 | luna_pinyin | real-worker-warm-reload | no | 621.0 | 636.0 | 37.0 | 0 | 36875347 | 184549376 | 184549376 | 0/0/0 |
| tracked-luna-warm-new-page | 3 | luna_pinyin | real-worker-warm-new-page | no | 663.0 | 668.0 | 33.0 | 0 | 36593791 | 184549376 | 184549376 | 0/0/0 |
| tracked-jyut-cold | 3 | jyut6ping3_mobile | real-worker-cold | no | 6005.0 | 6027.0 | 48.0 | 33359106 | 33357906 | 936509440 | 936509440 | 0/0/0 |
| tracked-jyut-warm-reload | 3 | jyut6ping3_mobile | real-worker-warm-reload | no | 5468.0 | 5482.0 | 46.0 | 0 | 36666793 | 936509440 | 936509440 | 0/0/0 |
| tracked-jyut-warm-new-page | 3 | jyut6ping3_mobile | real-worker-warm-new-page | no | 5466.0 | 5514.0 | 47.0 | 0 | 33357906 | 936509440 | 936509440 | 0/0/0 |
| tracked-mock-cold | 3 | luna_pinyin | mock-worker-cold | no | 601.0 | 605.0 | 10.0 | 747840 | 746640 | 0 | 0 | 0/0/0 |
| tracked-mock-warm | 3 | luna_pinyin | mock-worker-warm | no | 381.0 | 409.0 | 26.0 | 747840 | 746640 | 0 | 0 | 0/0/0 |
| public-luna-cold | 3 | luna_pinyin | real-worker-cold | yes | 1247.0 | 1253.0 | 38.0 | 36594931 | 36593731 | 184549376 | 184549376 | 2/24/0 |
| public-jyut-cold | 3 | jyut6ping3_mobile | real-worker-cold | yes | 5926.0 | 6383.0 | 49.0 | 33359046 | 33357846 | 936509440 | 936509440 | 1/35/0 |

## Startup Owner Map

| Scenario | Top owner | Owner median ms | Ready median ms | Ready p95 ms |
| --- | --- | ---: | ---: | ---: |
| tracked-luna-cold | worker total to initialized | 625.0 | 1239.0 | 1332.0 |
| tracked-luna-warm-reload | worker total to initialized | 568.0 | 621.0 | 636.0 |
| tracked-luna-warm-new-page | worker total to initialized | 563.0 | 663.0 | 668.0 |
| tracked-jyut-cold | worker total to initialized | 5407.0 | 6005.0 | 6027.0 |
| tracked-jyut-warm-reload | worker total to initialized | 5411.0 | 5468.0 | 5482.0 |
| tracked-jyut-warm-new-page | worker total to initialized | 5356.0 | 5466.0 | 5514.0 |
| tracked-mock-cold | React/browser ready residual | 601.0 | 601.0 | 605.0 |
| tracked-mock-warm | React/browser ready residual | 381.0 | 381.0 | 409.0 |
| public-luna-cold | worker total to initialized | 655.0 | 1247.0 | 1253.0 |
| public-jyut-cold | worker total to initialized | 5325.0 | 5926.0 | 6383.0 |

## Asset Transfer By Group

| Group | Transfer bytes | Encoded bytes | Duration ms |
| --- | ---: | ---: | ---: |
| schema binary | 0 | 16647200 | 0.0 |
| schema yaml | 0 | 4339161 | 0.0 |
| wasm binary | 0 | 2594503 | 0.0 |
| other | 491496 | 491196 | 320.0 |
| app js | 221186 | 220886 | 4.0 |
| wasm glue | 0 | 72378 | 0.0 |
| opencc | 0 | 66408 | 0.0 |
| worker script | 2425 | 42850 | -57.0 |
| app css | 32733 | 32433 | 2.0 |

## Browser Memory

| Scenario | WASM heap | Peak WASM heap | JS heap used | JS heap total | DOM nodes | Windows working set |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| tracked-luna-cold | 184549376 | 184549376 | 6381216 | 33566720 | 1095 | 740573184 |
| tracked-luna-warm-reload | 184549376 | 184549376 | 9290528 | 40960000 | 2227 | 746627072 |
| tracked-luna-warm-new-page | 184549376 | 184549376 | 6299460 | 33828864 | 1095 | 733151232 |
| tracked-jyut-cold | 936509440 | 936509440 | 5558132 | 8757248 | 983 | 1355247616 |
| tracked-jyut-warm-reload | 936509440 | 936509440 | 6340192 | 12427264 | 997 | 1414918144 |
| tracked-jyut-warm-new-page | 936509440 | 936509440 | 5627696 | 9805824 | 997 | 1368784896 |
| tracked-mock-cold | 0 | 0 | 4853844 | 8757248 | 635 | 352710656 |
| tracked-mock-warm | 0 | 0 | 8287092 | 11902976 | 1779 | 408150016 |
| public-luna-cold | 184549376 | 184549376 | 5729908 | 33828864 | 1095 | 742039552 |
| public-jyut-cold | 936509440 | 936509440 | 5920852 | 10067968 | 1108 | 1364209664 |
