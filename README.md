# Yune

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/rust-1.76%2B-orange.svg)](https://www.rust-lang.org)

**Languages:** English | [简体中文](README.zh-CN.md) | [粵語](README.yue.md)

> The engine that turns your typing into Chinese characters.
> Type `nihao`, get 你好. Type `nei5 hou2`, get 你好 in Cantonese.
> Built in Rust — runs on desktop, in the browser, and anywhere else.

## Contents

- [What Yune Does](#what-yune-does)
- [Why It Exists](#why-it-exists)
- [How It Works](#how-it-works)
- [Current Status](#current-status)
- [Compatibility](#compatibility)
- [Performance](#performance)
- [Quick Start](#quick-start)
- [Quality Checks](#quality-checks)
- [Repository Layout](#repository-layout)
- [Documentation](#documentation)
- [Non-Goals](#non-goals)
- [Contributing](#contributing)
- [License](#license)

## What Yune Does

You type romanized Chinese (Pinyin for Mandarin, Jyutping for Cantonese) on a
standard keyboard. Yune converts it to the right Chinese characters in real time.

Under the hood, Yune reads the same dictionary and configuration files as
[RIME](https://rime.im) — the most widely used open-source Chinese input-method
engine. This means it works with the thousands of existing RIME schemas and
dictionaries that the community has built over the years.

**[yune-web.pages.dev](https://yune-web.pages.dev)** — try it in your browser.

### Capabilities

- RIME schema and config handling: `__include`, `__patch`, custom patches, deploy
  freshness, schema installation, and schema switching.
- Full input pipeline: speller, selector, navigator, key binder, editor, ASCII
  composer, chord composer, punctuation, recognizer, translators, and filters.
- Dictionary support: source `.dict.yaml`, imports, Yune-native compiled
  table/prism/reverse artifacts, rebuild execution, and fixture-backed ranking
  verified against the reference engine.
- C ABI compatibility: upstream-shaped default `RimeApi` and `RimeLeversApi`,
  config/context/candidate/session/deploy APIs, dynamic-loader tests, and
  frontend lifecycle tests.
- TypeDuck profile behavior: fork-only ABI slots exposed through
  `rime_get_typeduck_profile_api()`, rich Cantonese dictionary comments, and
  TypeDuck-Web/Windows compatibility evidence.
- Browser runtime: `@yune-ime/yune-web-runtime`, the `yune-web` Vite app,
  multi-schema browser harness (jyut6ping3, cangjie5, luna_pinyin, and more),
  UI language switching, output standard selection, public demo, and Playwright
  evidence.
- AI foundation: provider trait, local/mock providers, staged AI rows, privacy
  policy, separate AI memory, and default-off browser exposure.

## Why It Exists

RIME has been the backbone of open-source Chinese input for over a decade. It
works well. But it's a large C++ codebase that's difficult to change, test, or
embed in modern environments like browsers and mobile apps.

Yune rebuilds the engine from scratch in Rust with three goals:

**Run everywhere.** The same core engine compiles to a native shared library
(for desktop IMEs like Squirrel, Weasel, or ibus-rime), to WebAssembly (for
browser-based input), or to a CLI tool (for testing and benchmarking).

**Be testable.** Every behavior is verified byte-for-byte against the real RIME
engine. Instead of porting C++ code (and inheriting its bugs and assumptions),
Yune runs RIME as a "behavior oracle": capture what it outputs for a given
input, then assert Yune produces the exact same result. This preserves
compatibility without cargo-culting a 15-year-old C++ architecture.

**Prepare for AI-native input.** The engine has a built-in, default-off AI layer.
In the future, an on-device language model could suggest completions or
corrections alongside traditional dictionary candidates — without slowing down
the classic path and without sending your typing to a cloud service.

## How It Works

```
keystrokes  ──►  spelling algebra  ──►  dictionary lookup  ──►  ranking & filtering  ──►  commit text
                    (normalize)          (find candidates)        (sort, deduplicate)        (output)
```

The pipeline is built from swappable Rust traits — translators, filters, and
rankers — rather than a monolithic class hierarchy. Want to plug in a custom
ranking model? Implement a trait. Want a different dictionary format? Swap the
translator.

Everything runs in safe Rust. The workspace enforces `unsafe_code = "forbid"`.

## Current Status

Yune is an active engine project.

- **Compatibility baseline:** Phase 1 is complete. Yune produces identical output
  to RIME 1.17.0 for Mandarin (`luna_pinyin`) and Cantonese (`jyut6ping3` via
  TypeDuck profile). It has been validated as a drop-in replacement in real-world
  frontends (TypeDuck-Web, TypeDuck-Windows).
- **Current work:** milestones M38-M46 are complete. Native startup/session,
  `zhongguo`, both long full-pinyin rows, and both abbreviation rows are now
  faster than same-run librime while matching its candidate output. The
  remaining gaps are explicit and honestly measured: short-key fixed overhead
  (`n`/`ni`/`hao` are slower than librime, but only tens of microseconds, so
  imperceptible while typing), and whole-process memory, where M43-M46
  attributed the footprint but found no cheap structural fix — Yune is several
  times heavier than librime on the fair `luna_pinyin` comparison, and the
  Jyutping product path is heavier still (though it carries a much larger
  multilingual dictionary, so it has no like-for-like librime baseline).
- **Public demo:** `yune-web` is deployed at <https://yune-web.pages.dev>. It's
  a Yune engine demo, not a claim that browser-level performance is solved.
- **AI posture:** the AI layer exists but is default-off, local-only in the web
  harness, and outside the classic deterministic input path.

See [docs/roadmap.md](docs/roadmap.md) for the detailed milestone plan.

## Compatibility

Yune's compatibility is target-driven, not checklist-driven.

**Reference engines** (the "oracles" that define correct behavior):

- Default core oracle: upstream `rime/librime 1.17.0`
  (`33e78140250125871856cdc5b42ddc6a5fcd3cd4`).
- TypeDuck profile oracle: TypeDuck-HK/librime `v1.1.2`
  (`74cb52b78fb2411137a7643f6c8bc6517acfde69`).

**Rules:**

- Preserve upstream-observable behavior for named targets.
- Isolate TypeDuck fork behavior behind the TypeDuck profile surface.
- Add librime features only when a named target needs them.
- Keep expected bytes non-circular: always capture them from the relevant oracle,
  never derive them from Yune itself.

Default `rime_get_api()` remains upstream-shaped. TypeDuck fork-only ABI slots
are exposed exclusively through `rime_get_typeduck_profile_api()`.

## Performance

The current native comparison is mixed, honest, and intentionally measured
same-run against upstream `rime/librime 1.17.0`. Yune **matches librime
candidate output on every row** and is **faster on seven of ten rows** —
including the two abbreviation rows it also matches byte-for-byte. It is slower
only on the three single-character short-key rows, a higher constant factor on
inputs of tens of microseconds that is imperceptible while typing. The one real
gap is memory, where Yune is several times heavier than librime.

![Yune vs librime native latency ratios](docs/reports/evidence/yune-vs-librime-native-ratios.svg)

Current native Track A same-run ratios (M45 final; lower is better):

- **Faster than librime:** `zhongguo` `0.373x`, `cszysmsrsd` `0.440x`,
  `zybfshmsru` `0.640x`, 59-character pinyin `0.720x`, startup `0.875x`,
  session `0.921x`, and 37-character pinyin `0.939x`. The two abbreviation rows
  (`cszysmsrsd`, `zybfshmsru`) match librime candidate output exactly *and* beat
  its latency.
- **Slower than librime:** `hao` ~`2.1-2.2x`, `n` ~`3.3-3.5x`, `ni`
  ~`3.5-3.7x` (short-key ratios vary a few percent run-to-run). These are
  single-character inputs of `24-69 us` — a real constant-factor gap, but
  imperceptible in use; candidate output matches librime on every run.
- **Memory is the honest weak spot.** On the fair `luna_pinyin` comparison —
  same schema, no dictionary confound — Yune is about `10x` heavier than a
  librime-family engine both natively (`127 MB` vs librime `13-17 MB`) and in the
  browser (`160 MiB` vs My RIME `16 MiB`). That ~10x is real, not a dictionary
  artifact. The Jyutping product path is heavier still (`504 MB` native, `893 MB`
  browser), but it is **not** a like-for-like comparison — Yune runs TypeDuck's
  multilingual `jyut6ping3` (Cantonese plus English/Hindi/Urdu/Nepali), so its
  number is the ~10x base inefficiency plus a larger dictionary, recorded as a
  guard rather than a comparison. M43-M46 attributed the visible structural
  owners and found no cheap structural fix, so memory stays a measured, open
  gap.
- Track B TypeDuck-profile rows and browser startup are separate evidence lanes,
  not upstream-librime native comparisons.

Current reports:

- [docs/reports/yune-vs-librime-performance.md](docs/reports/yune-vs-librime-performance.md)
- [docs/reports/yune-vs-librime-root-cause-analysis.md](docs/reports/yune-vs-librime-root-cause-analysis.md)

## Quick Start

Prerequisites:

- Rust 1.76 or newer
- Node.js and npm (for the browser harness and TypeScript runtime)
- Emscripten (only if building the WASM artifact locally)

Build and test:

```bash
cargo build
cargo test --workspace
```

Feed keystrokes directly to the core engine:

```bash
cargo run -p yune-cli -- run "nihao "
```

Run against real RIME data through the full ABI path:

```bash
cargo run -p yune-cli -- frontend \
  --shared-data-dir ./path/to/rime-data \
  --user-data-dir ./tmp/yune-user \
  --schema luna_pinyin \
  --sequence "nihao "
```

Run the browser demo locally:

```bash
npm --prefix apps/yune-web install
npm --prefix apps/yune-web run build
npm --prefix apps/yune-web run start
```

For browser validation work, start with
[apps/yune-web/e2e/yune-browser-smoke.md](apps/yune-web/e2e/yune-browser-smoke.md).

## Quality Checks

Run these before merging significant changes:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm --prefix packages/yune-web-runtime test
npm --prefix packages/yune-web-runtime run build
```

Browser-visible claims need Playwright or equivalent real-browser evidence.

## Repository Layout

| Path | What's In It |
| --- | --- |
| `crates/yune-core` | The engine: dictionary lookup, spelling algebra, candidate ranking, filters, user dictionary, AI staging. |
| `crates/yune-rime-api` | C ABI adapter: exposes the engine as a drop-in replacement for RIME's shared library. |
| `crates/yune-cli` | Developer CLI: feed it keystrokes, get JSON output for testing and debugging. |
| `packages/yune-web-runtime` | TypeScript wrapper for the WASM build. |
| `apps/yune-web` | Browser demo app — the public face of the project. |
| `docs` | Roadmap, architecture decisions, conventions, reports. |
| `fixtures` | Deterministic test fixtures (expected engine output for given inputs). |
| `scripts` | Build helpers, benchmarks, oracle-capture tooling. |

## Documentation

- [docs/conventions.md](docs/conventions.md) — architecture, stack, coding rules,
  testing conventions, ABI rules, integrations, and current risks.
- [docs/roadmap.md](docs/roadmap.md) — active roadmap and milestone gates.
- [docs/decisions.md](docs/decisions.md) — decision log and standing principles.
- [docs/requirements.md](docs/requirements.md) — requirement IDs and status.
- [docs/ledgers/fork-parity-ledger.md](docs/ledgers/fork-parity-ledger.md) —
  Cantoboard and TypeDuck fork deltas versus upstream.
- [docs/plans/](docs/plans/) — active, reference, and completed execution
  records.

## Non-Goals

Equally important as the goals — these are things Yune intentionally does not do:

- Bit-for-bit librime internals or full C++ plugin ABI parity.
- A broad librime feature checklist without a named target.
- Widening the default upstream `RimeApi` for TypeDuck-only behavior.
- Cloud inference as a hard dependency.
- Remote AI providers without explicit privacy and product gates.
- Claiming application/browser performance wins from native engine evidence.

## Contributing

Bug reports, feature proposals, and pull requests are welcome. For anything that
affects behavioral compatibility, include oracle-captured evidence (real RIME
output against the same input — expected values must not be derived from Yune
itself). Start with [docs/conventions.md](docs/conventions.md) for architecture
and coding rules.

## License

Original code is [MIT](LICENSE). Third-party schemas, dictionaries, fixtures,
generated data, and provenance materials keep their upstream licenses — see
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
