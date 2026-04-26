# Yune Initial Analysis

## Decision

Do not start with a full librime rewrite. Start with a compatibility harness and
AI extension points, then replace modules only after behavior is measurable.

## Why Not Full Rewrite First

librime's value is not only its C++ implementation. The hard parts are:

- RIME schema semantics and patch behavior.
- Key processing and composition edge cases.
- Translators, segmentors, filters, and their order-dependent behavior.
- Dictionary compilation, lookup, user frequency, and prediction.
- Stable C API expected by frontends such as Squirrel, Weasel, ibus-rime, and
  fcitx-rime.
- Existing plugins such as Lua, octagram, predict, and proto.

Rewriting without fixtures would make it hard to know whether a difference is an
improvement or a compatibility regression.

## Compatibility Layers

Yune should separate compatibility into layers:

1. Schema compatibility: YAML structure, component names, patch/import rules.
2. Runtime behavior compatibility: key sequence to context/candidates/commit.
3. Frontend compatibility: C ABI surface shaped like `rime_api.h`.
4. Data compatibility: ability to consume RIME dictionaries or rebuild them.
5. Plugin compatibility: new Yune plugin ABI first; C++ plugin ABI later only if
   there is clear demand.

The first three layers matter most for adoption.

## First Engineering Target

Build a deterministic runner:

- Input: schema fixture plus key sequence.
- Output: preedit, candidates, highlighted index, commit text, status flags.
- Use this runner to compare Yune behavior with librime output.

Only after this runner exists should core modules be rewritten in Rust.

## AI Integration Position

AI should be an optional ranking/completion layer, not the foundation of basic
input. Classic candidates must remain available with low latency and without
network access.

Initial AI surfaces:

- Candidate reranking filter.
- Contextual phrase completion translator.
- Personalized user dictionary suggestions.
- Privacy-preserving local model bridge.

## Non-Goals For The First Milestone

- Loading existing C++ librime plugins.
- Full binary compatibility with all compiled RIME data formats.
- Cloud inference as a required dependency.
- A new end-user frontend.
