# Yune AI-Native Context Awareness — Tradeoff Notes

> **Status:** Reference / future work — **not** a committed plan, milestone, or
> decision. **Created:** 2026-06-27. Captures the design discussion behind the
> deferred AI-native layer. Informs the model/provider decision in
> [`docs/plans/active/m32-plan-ai-native-public-demo-product-layer.md`](../active/m32-plan-ai-native-public-demo-product-layer.md)
> (Task 1) and complements
> [`docs/plans/reference/m11-design-ai-native.md`](./m11-design-ai-native.md).
> No implementation is authorized by this doc.

## Why this exists

Context awareness is the proposed functional differentiator of Yune versus
librime: ranking and predicting candidates using the surrounding text, the
app/field, and meaning — not just per-syllable dictionary lookup plus an N-gram
sentence model. This note records the tradeoffs for *how* to achieve it so a
future milestone can choose with eyes open. The hard constraint is IME latency.

## Context awareness is a ladder, not one feature

The right tool and the latency answer differ sharply by rung:

1. **In-composition** (the current sentence so far). N-gram / octagram already
   does this. Microseconds. Effectively solved.
2. **Preceding committed text** (prior sentences / paragraph). Where real
   context begins; weak in plain N-grams.
3. **App / field / formality** (email vs chat vs code). Pure metadata — the
   `AiContext` features from M11 S3 feeding the ranker. No model needed.
4. **Semantic / world-knowledge** ("flying to 北京 tomorrow, the 機票…" →
   *understands* the relation). The only rung that genuinely needs an LLM.

Most of the perceived "smarter than librime" feeling lives in rungs 2–3, and
those are reachable **within IME latency without an LLM**. Only rung 4 forces an
LLM, and that is where latency, memory, cost, and privacy all bite.

## Tool spectrum

| Tool | Context reach | Per-key latency | Memory | Offline | Privacy |
| --- | --- | --- | --- | --- | --- |
| N-gram (octagram-style) | rung 1–2 (weak) | µs | MBs | yes | local |
| **Small on-device neural LM** | **rung 1–3 (strong)** | **~ms** | **tens of MB** | **yes** | **local** |
| Big local LLM (1B+ params) | rung 4 | 100 ms – 1 s | GBs | yes | local |
| Cloud LLM API | rung 4 | 200 ms – 1 s+ | 0 local | no | sends keystrokes off-device |

The decision is **not** "statistical vs LLM." The middle option — a small
on-device neural LM (~1–50M params, quantized, tens of MB, single-digit-ms
inference) — is how production keyboards (GBoard, SwiftKey, Apple QuickType)
actually do context-aware prediction. It beats N-grams on rungs 2–3 at IME
latency, offline and private. It is the most likely answer for Yune's
differentiator.

## Latency budgets — tier, don't choose one

- **Per-keystroke** must be under ~16 ms to feel instant. Only N-gram and
  small-neural fit. Context-aware *ranking* lives here.
- **On-pause** (~300–500 ms idle) gives a few hundred ms. This is the only place
  a big LLM (local or cloud) belongs — an async "complete this phrase"
  suggestion that appends when ready and never blocks typing.

The M11/M13 async second-pass architecture already supports this split, so the
design tiers by latency budget rather than picking a single tool.

## Privacy is decisive for an IME

An IME sees everything the user types. A cloud LLM streaming keystrokes
off-device is a keylogger by another name unless deeply gated, and it breaks
offline use (D-10: cloud can never be required). So cloud is a last resort:
opt-in, sensitive-context-blocked (the M11 S3/S4 policy already enforces this).
This biases the whole design toward on-device models.

## Current leaning (a lean, not a commitment)

- **Differentiator = a small on-device neural context model** feeding candidate
  ranking (rungs 2–3), using the existing `AiContext` features. Bounded, proven
  R&D; offline; private; within latency. Main cost: obtaining/quantizing a
  Cantonese/Mandarin context LM and integrating a Rust/WASM inference runtime
  (e.g. candle / ggml / ONNX) within the WASM memory budget — tens of MB is
  tolerable where the M46 finding rules out GB-scale local models in the browser.
- **Big LLM (rung 4) = optional, on-pause, opt-in.** Prefer local; treat cloud
  as an explicit, privacy-gated opt-in that may never ship.
- **Nothing in the synchronous per-key path** beyond the small model's ms-scale
  scoring.

## Relationship to adjacent work

- **octagram** is a *separate, compatibility-only* item: it matches librime byte
  output on schemas that ship `.gram`. It is not the differentiator. Deferred
  until a named `.gram`-using target exists; current targets (`luna_pinyin`
  null-grammar + `essay.txt`, dictionary-driven `jyut6ping3`) do not use it. A
  small neural LM subsumes octagram's *prediction* value but never its
  *compatibility* role.
- **M32** (AI-native public exposure) is downstream of this note. M32 exposes a
  model; this note is about *which* model. M32 should not start until value is
  validated and the model is chosen.
- **M46** memory ceiling is the binding constraint on any on-device model size.

## Open questions for the future milestone (in order)

1. **Value first.** Write 3–5 concrete Cantonese/Mandarin scenarios where
   context-aware ranking clearly beats the classic engine. If none are
   compelling, stop and invest in a better statistical LM instead.
2. Which on-device neural model, training data, and size fit the WASM budget?
3. Which Rust + WASM inference runtime, and what is its *measured* memory cost?
4. Integration: scoring first-page candidates per key within the latency budget.
5. Is the rung-4 LLM tier worth building at all — and if so, local vs opt-in
   remote?
