# Roadmap

## M0: Skeleton

- Rust workspace.
- Core session and candidate types.
- CLI smoke test.
- Initial analysis and architecture notes.

## M1: Compatibility Harness

- Record librime fixtures for common schemas.
- Run Yune fixtures from CLI.
- Define JSON output for context, candidates, commit, and status.
- Add CI checks for deterministic behavior.

## M2: Schema Subset

- Parse a RIME-style schema subset.
- Model processors, segmentors, translators, and filters as named components.
- Support minimal punctuation and echo translation.

## M3: Dictionary Prototype

- Implement a simple table dictionary format.
- Support deterministic lookup and candidate ranking.
- Add fixture coverage for pinyin and shape-based schemas.

## M4: AI Hook

- Add a non-blocking candidate reranking trait.
- Provide a mock ranker for tests.
- Keep classic candidate ordering as fallback.

## M5: RIME Frontend Shim

- Implement a small C ABI subset.
- Validate against a local frontend or compatibility test client.
