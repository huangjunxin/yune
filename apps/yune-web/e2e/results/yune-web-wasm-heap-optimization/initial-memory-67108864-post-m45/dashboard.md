# Yune Web Startup Benchmark Dashboard

## Summary

| Scenario | Samples | Schema | Mode | Public | Median ready ms | p95 ready ms | Median first key ms | Transfer bytes | Encoded bytes | WASM heap bytes | Peak WASM heap bytes | Cache h/m/e |
| --- | ---: | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| tracked-luna-cold | 3 | luna_pinyin | real-worker-cold | no | 1200.0 | 1210.0 | 38.0 | 36594991 | 36593791 | 167772160 | 167772160 | 0/0/0 |
| tracked-luna-warm-reload | 3 | luna_pinyin | real-worker-warm-reload | no | 615.0 | 616.0 | 38.0 | 0 | 36875347 | 167772160 | 167772160 | 0/0/0 |
| tracked-luna-warm-new-page | 3 | luna_pinyin | real-worker-warm-new-page | no | 654.0 | 654.0 | 38.0 | 0 | 36593791 | 167772160 | 167772160 | 0/0/0 |
| tracked-jyut-cold | 3 | jyut6ping3_mobile | real-worker-cold | no | 5900.0 | 5942.0 | 47.0 | 33359106 | 33357906 | 936509440 | 936509440 | 0/0/0 |
| tracked-jyut-warm-reload | 3 | jyut6ping3_mobile | real-worker-warm-reload | no | 5302.0 | 5315.0 | 45.0 | 0 | 36666793 | 936509440 | 936509440 | 0/0/0 |
| tracked-jyut-warm-new-page | 3 | jyut6ping3_mobile | real-worker-warm-new-page | no | 5351.0 | 5419.0 | 41.0 | 0 | 33357906 | 936509440 | 936509440 | 0/0/0 |
| tracked-mock-cold | 3 | luna_pinyin | mock-worker-cold | no | 608.0 | 619.0 | 25.0 | 747840 | 746640 | 0 | 0 | 0/0/0 |
| tracked-mock-warm | 3 | luna_pinyin | mock-worker-warm | no | 383.0 | 387.0 | 28.0 | 747840 | 746640 | 0 | 0 | 0/0/0 |
| public-luna-cold | 3 | luna_pinyin | real-worker-cold | yes | 1249.0 | 1316.0 | 35.0 | 36594931 | 36593731 | 167772160 | 167772160 | 2/24/0 |
| public-jyut-cold | 3 | jyut6ping3_mobile | real-worker-cold | yes | 5902.0 | 5980.0 | 41.0 | 33359046 | 33357846 | 936509440 | 936509440 | 1/35/0 |

## Startup Owner Map

| Scenario | Top owner | Owner median ms | Ready median ms | Ready p95 ms |
| --- | --- | ---: | ---: | ---: |
| tracked-luna-cold | worker total to initialized | 603.0 | 1200.0 | 1210.0 |
| tracked-luna-warm-reload | worker total to initialized | 548.0 | 615.0 | 616.0 |
| tracked-luna-warm-new-page | worker total to initialized | 554.0 | 654.0 | 654.0 |
| tracked-jyut-cold | worker total to initialized | 5294.0 | 5900.0 | 5942.0 |
| tracked-jyut-warm-reload | worker total to initialized | 5246.0 | 5302.0 | 5315.0 |
| tracked-jyut-warm-new-page | worker total to initialized | 5220.0 | 5351.0 | 5419.0 |
| tracked-mock-cold | React/browser ready residual | 608.0 | 608.0 | 619.0 |
| tracked-mock-warm | React/browser ready residual | 383.0 | 383.0 | 387.0 |
| public-luna-cold | worker total to initialized | 637.0 | 1249.0 | 1316.0 |
| public-jyut-cold | worker total to initialized | 5296.0 | 5902.0 | 5980.0 |

## Asset Transfer By Group

| Group | Transfer bytes | Encoded bytes | Duration ms |
| --- | ---: | ---: | ---: |
| schema binary | 0 | 16647200 | 0.0 |
| schema yaml | 0 | 4339161 | 0.0 |
| wasm binary | 0 | 2594503 | 0.0 |
| other | 491496 | 491196 | 329.0 |
| app js | 221186 | 220886 | 4.0 |
| wasm glue | 0 | 72378 | 0.0 |
| opencc | 0 | 66408 | 0.0 |
| worker script | 2425 | 42850 | -55.0 |
| app css | 32733 | 32433 | 2.0 |

## Browser Memory

| Scenario | WASM heap | Peak WASM heap | JS heap used | JS heap total | DOM nodes | Windows working set |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| tracked-luna-cold | 167772160 | 167772160 | 5332752 | 33566720 | 1095 | 742072320 |
| tracked-luna-warm-reload | 167772160 | 167772160 | 6568468 | 15048704 | 699 | 766119936 |
| tracked-luna-warm-new-page | 167772160 | 167772160 | 6201704 | 33566720 | 1095 | 720912384 |
| tracked-jyut-cold | 936509440 | 936509440 | 4723684 | 9543680 | 1031 | 1355952128 |
| tracked-jyut-warm-reload | 936509440 | 936509440 | 4459700 | 11902976 | 981 | 1411584000 |
| tracked-jyut-warm-new-page | 936509440 | 936509440 | 5491388 | 38264832 | 1038 | 1373212672 |
| tracked-mock-cold | 0 | 0 | 4861780 | 8757248 | 635 | 354635776 |
| tracked-mock-warm | 0 | 0 | 7771628 | 11640832 | 1779 | 411369472 |
| public-luna-cold | 167772160 | 167772160 | 5566872 | 33828864 | 1095 | 746835968 |
| public-jyut-cold | 936509440 | 936509440 | 4590204 | 8757248 | 981 | 1375854592 |
