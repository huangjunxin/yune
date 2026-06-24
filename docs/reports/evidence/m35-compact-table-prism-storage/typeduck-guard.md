# TypeDuck Guard

M35 does not enable compact storage for TypeDuck `jyut6ping3`.

Reason: the TypeDuck profile is a product compatibility surface with invariants
that are broader than upstream `luna_pinyin`:

- rich comments and panel comment bytes;
- lookup records and dictionary-lookup filter behavior;
- long composition and prediction ranking;
- partial selection and consumed-span recomposition;
- default-confirm recomposition;
- userdb learning of full primary dictionary codes;
- dynamic correction scans.

The implementation keeps TypeDuck on heap fallback rather than weakening any of
those behaviors for memory or latency.

Required guards:

```powershell
cargo test -p yune-core --test cantonese_parity
cargo test -p yune-rime-api --test typeduck_web -- --test-threads=1
```

Results:

- `cantonese_parity`: passed, `37` tests.
- `typeduck_web`: passed, `29` tests in `610.39s`.

Native TypeDuck full-ABI watch rows stayed within the 10% guard:

| Row | Before median | After median | Change |
| --- | ---: | ---: | ---: |
| `hai_full_abi` | `18900.742us` | `18450.767us` | `-2.4%` |
| `ngohaig_full_abi` | `14879.035us` | `13384.063us` | `-10.0%` |
| `jigaajiusihaa_full_abi` | `28836.874us` | `26953.441us` | `-6.5%` |
| `loengjathau_full_abi` | `16993.191us` | `16051.646us` | `-5.5%` |
| `jigaajiusihaa_correction_full_abi` | `24811.675us` | `26707.480us` | `+7.6%` |

Conclusion: TypeDuck compact storage is closed by no-go for M35 and remains an
explicit future slice if lookup-record/rich-comment parity can be proven.
