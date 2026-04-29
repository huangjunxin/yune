---
phase: 01-cli-frontend-surrogate
reviewed: 2026-04-29
status: secured
threats_reviewed: 15
threats_open: 0
---

# Phase 1: Security Review

**Verdict:** secured

All planned Phase 01 threats were reviewed against the implemented CLI frontend surrogate, resolved code review report, and verification artifact. Mitigations are present for every planned `mitigate` disposition, and the single planned acceptance remains bounded to local CLI input size.

## Threat Disposition Table

| Threat | Category | Target | Disposition | Evidence | Notes |
| --- | --- | --- | --- | --- | --- |
| T-01-01-01 | Tampering | `args.rs` runtime path flags | mitigated | `crates/yune-cli/src/args.rs:63`, `crates/yune-cli/src/args.rs:73`, `crates/yune-cli/src/args.rs:95`, `crates/yune-cli/src/args.rs:200`, `crates/yune-cli/tests/frontend_surrogate.rs:84` | Frontend commands require explicit `--shared-data-dir` and `--user-data-dir`; missing paths fail during CLI parse before ABI setup. |
| T-01-01-02 | Elevation of Privilege | `rime_frontend.rs` FFI boundary | mitigated | `crates/yune-cli/src/rime_frontend.rs:17`, `crates/yune-cli/src/rime_frontend.rs:153`, `crates/yune-cli/src/rime_frontend.rs:593`, `crates/yune-cli/src/rime_frontend.rs:610`, `crates/yune-cli/src/rime_frontend.rs:617`, `crates/yune-cli/src/rime_frontend.rs:641` | Unsafe ABI calls and pointer conversions are centralized in `rime_frontend.rs`; ABI structs initialize `data_size` before calls. |
| T-01-01-03 | Denial of Service | `rime_frontend.rs` process-wide RIME state | mitigated | `crates/yune-cli/src/rime_frontend.rs:185`, `crates/yune-cli/src/rime_frontend.rs:202`, `crates/yune-cli/src/rime_frontend.rs:656`, `crates/yune-cli/src/rime_frontend.rs:681`, `crates/yune-cli/src/rime_frontend.rs:795` | Cleanup guard destroys active sessions, calls `cleanup_all_sessions` when available, and finalizes initialized RIME state on success or failure. |
| T-01-01-04 | Tampering | `rime_frontend.rs` allocation ownership | mitigated | `crates/yune-cli/src/rime_frontend.rs:256`, `crates/yune-cli/src/rime_frontend.rs:271`, `crates/yune-cli/src/rime_frontend.rs:285`, `crates/yune-cli/src/rime_frontend.rs:305`, `crates/yune-cli/src/rime_frontend.rs:316`, `crates/yune-cli/src/rime_frontend.rs:342` | Populated commit, context, and status ABI structs are copied into owned Rust values before their matching `free_*` calls. |
| T-01-01-05 | Information Disclosure | CLI error output | mitigated | `crates/yune-cli/src/rime_frontend.rs:109`, `crates/yune-cli/src/rime_frontend.rs:124`, `crates/yune-cli/src/rime_frontend.rs:186`, `crates/yune-cli/src/args.rs:183`, `crates/yune-cli/tests/frontend_surrogate.rs:84` | Error messages use problem/next-action wording and tests cover missing path errors without stdout leakage. Schema validation prevents path-like schema IDs from appearing in runtime path handling. |
| T-01-02-01 | Information Disclosure | `transcript.rs` frontend JSON | mitigated | `crates/yune-cli/src/transcript.rs:6`, `crates/yune-cli/src/transcript.rs:26`, `crates/yune-cli/src/transcript.rs:345`, `crates/yune-cli/src/transcript.rs:480`, `crates/yune-cli/src/transcript.rs:677`, `crates/yune-cli/tests/frontend_surrogate.rs:202` | Frontend JSON serializes owned transcript fields only and tests assert absence of runtime paths, PIDs, timestamps, durations, pointer-like values, and environment names. |
| T-01-02-02 | Tampering | `fixture.rs` replay fixture parsing | mitigated | `crates/yune-cli/src/fixture.rs:23`, `crates/yune-cli/src/fixture.rs:59`, `crates/yune-cli/src/fixture.rs:73`, `crates/yune-cli/src/fixture.rs:107`, `crates/yune-cli/src/fixture.rs:290`, `crates/yune-cli/src/fixture.rs:311` | Fixture replay extracts only top-level `schema_id` and `sequence` string fields, rejects malformed strings, and reuses schema ID validation before replay. |
| T-01-02-03 | Repudiation | `fixture.rs` mismatch output | mitigated | `crates/yune-cli/src/fixture.rs:41`, `crates/yune-cli/src/fixture.rs:47`, `crates/yune-cli/src/fixture.rs:208`, `crates/yune-cli/src/fixture.rs:355` | Mismatches include deterministic expected and actual bodies; JSON normalization preserves string whitespace and omits environment-specific context from generated actual output. |
| T-01-02-04 | Denial of Service | `render.rs` human output | accepted | `crates/yune-cli/src/render.rs:9`, `crates/yune-cli/src/render.rs:11`, `crates/yune-cli/src/rime_frontend.rs:212` | Accepted as planned: this is a local CLI surrogate and transcript size is bounded by user-provided sequence length, not remote service input. |
| T-01-02-05 | Spoofing | `render.rs` terminal text | mitigated | `crates/yune-cli/src/render.rs:9`, `crates/yune-cli/src/render.rs:82`, `crates/yune-cli/src/render.rs:161`, `crates/yune-cli/tests/frontend_surrogate.rs:160` | Human output is plain label/value text; tests assert no ANSI escape and no path leakage. |
| T-01-03-01 | Denial of Service | process-wide frontend tests | mitigated | `crates/yune-cli/src/rime_frontend.rs:662`, `crates/yune-cli/tests/frontend_surrogate.rs:11`, `crates/yune-cli/tests/frontend_surrogate.rs:86`, `crates/yune-cli/tests/frontend_surrogate.rs:122`, `crates/yune-cli/tests/frontend_surrogate.rs:162`, `crates/yune-cli/tests/frontend_surrogate.rs:204`, `crates/yune-cli/tests/frontend_surrogate.rs:240` | Tests that touch process-wide RIME state use mutex guards; runtime code cleanup/finalize behavior is separately covered. |
| T-01-03-02 | Information Disclosure | transcript determinism tests | mitigated | `crates/yune-cli/src/transcript.rs:677`, `crates/yune-cli/tests/frontend_surrogate.rs:202` | Unit and integration tests assert frontend JSON omits temp paths, process IDs, timestamps, durations, pointer-like values, and environment-derived names. |
| T-01-03-03 | Tampering | module ownership markers | mitigated | `crates/yune-cli/src/args.rs:5`, `crates/yune-cli/src/rime_frontend.rs:17`, `crates/yune-cli/src/transcript.rs:6`, `crates/yune-cli/src/render.rs:3`, `crates/yune-cli/src/fixture.rs:9` | Ownership comments are narrow and explicitly scope Phase 1 to the CLI surrogate/librime comparison target rather than claiming native frontend validation. |
| T-01-03-04 | Repudiation | fixture replay tests | mitigated | `crates/yune-cli/src/fixture.rs:41`, `crates/yune-cli/src/fixture.rs:333`, `crates/yune-cli/src/fixture.rs:355`, `crates/yune-cli/tests/frontend_surrogate.rs:238` | Fixture replay tests compare deterministic CLI output against ABI replay and verify mismatch diagnostics contain expected/actual bodies. |
| T-01-03-05 | Elevation of Privilege | ABI test setup | mitigated | `crates/yune-cli/src/rime_frontend.rs:711`, `crates/yune-cli/src/rime_frontend.rs:722`, `crates/yune-cli/tests/frontend_surrogate.rs:43`, `crates/yune-cli/tests/frontend_surrogate.rs:65`, `crates/yune-cli/tests/frontend_surrogate.rs:126` | ABI tests create explicit temporary shared/user runtime roots and select logical schema IDs; no arbitrary resource IDs are used beyond those runtime roots. |

## Gate Result

`threats_open: 0`

Phase 01 satisfies the planned security mitigations. No remediation is required before proceeding to the next phase.
