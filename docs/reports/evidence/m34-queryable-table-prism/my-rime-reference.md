# M34 my_rime reference split

Date: 2026-06-23

The local `C:\Users\laubonghaudoi\Documents\GitHub\my_rime` checkout was treated
as reference-only.

Findings:

- `my_rime` uses stock librime compiled to WASM. It is not a Yune behavior
  oracle and no source was copied into Yune.
- Its smooth typing primarily reflects librime's existing lazy/page-bounded
  candidate data path.
- Its bridge returns the current librime menu page rather than forcing the web
  app to consume every candidate.
- Its selected-schema prebuilt asset fetch, lazy schema dependencies, content
  cache, PWA/CDN behavior, and worker isolation are delivery/cache lessons.
- Those delivery/cache lessons belong to M31 public-demo work, not M34.

M34 therefore records `my_rime` as an attribution split: engine candidate
pipeline and table/prism data-path ideas stay in M34; public web delivery and
browser warm-cache work stays in M31.
