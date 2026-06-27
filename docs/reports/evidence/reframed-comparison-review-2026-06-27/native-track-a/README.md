# Native In-Process Benchmark

This run uses the Rust `native_inprocess_benchmark` bench and loads each engine DLL directly in the measured process. It does not use the historical managed .NET/PInvoke benchmark host.

- Track A: luna_pinyin, Yune versus librime 1.17.0.
- Track B: jyut6ping3_mobile, Yune Cantonese profile/product path.
- Track A inputs: $TrackAInputs.
- Track B inputs: $TrackBInputs.

Track B caveat: this run did not deploy product compiled artifacts before the
benchmark. `product_path_status.csv` records `source_fallback=true` and
`compiled_ready=false` for the Track B dictionaries, so Track B rows in this
directory are source-YAML fallback artifacts and must not be cited as product
memory evidence. Use the M46 `source_fallback=false` byte-backed Track B run for
valid product memory numbers.
