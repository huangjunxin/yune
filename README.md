# Yune / 新韵

Yune is a RIME-compatible input engine experiment for modern, contextual
typing.

新韵是一个面向现代上下文输入体验的输入法引擎实验项目。第一阶段不追求
完整重写 librime，而是先建立兼容边界、测试基准和可演进的 Rust 核心。

## Goals

- Keep RIME schema and frontend compatibility as explicit design constraints.
- Validate AI-assisted candidate ranking and contextual completion before
  replacing mature engine behavior.
- Build small Rust components that can be tested independently.
- Avoid depending on C++ plugin ABI compatibility for the first milestone.

## Workspace

- `crates/yune-core`: session state, composition, candidates, and engine traits.
- `crates/yune-schema`: RIME schema compatibility layer.
- `crates/yune-rime-api`: future C ABI shim for RIME frontends.
- `crates/yune-cli`: local test runner for input sequences and diagnostics.

## First Milestone

The first milestone is a compatibility harness:

1. Load a small RIME-style schema subset.
2. Feed deterministic key sequences through a Yune session.
3. Compare composition, candidate, and commit output against recorded fixtures.
4. Add an AI ranking hook that can reorder candidates without blocking classic
   input behavior.
