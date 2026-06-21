# M10 T3 Stock TypeDuck-Windows Server IPC Smoke Evidence

> **Status:** Archived evidence - **Milestone:** M10 (TypeDuck-Windows native backend) - **Closed:** 2026-06-21 - **Type:** evidence record

This directory preserves the durable text evidence for the successful M10 T3
stock TypeDuck-Windows server/client IPC smoke. The original local run wrote
its full evidence under ignored `target/typeduck-windows-e2e/evidence/`; this
tracked copy keeps the reviewable proof without committing build outputs,
executables, DLLs, or machine-local absolute paths.

## Smoke Summary

- Yune landed fix: `9449f7e2 Complete M10 TypeDuck-Windows T3 smoke`
- TypeDuck-Windows checkout: `f3ffcfe3b6a3018b1c3c9d256a6f0d587a2d2e27`
- Original local evidence scope: `target/typeduck-windows-e2e/evidence/m10-t3-20260621-100337-stock-real-server`
- Server under test: stock `TypeDuckServer.exe` from the pinned TypeDuck-Windows checkout output
- Client under test: stock `TestTypeDuckIPC.exe /console`
- Yune package under test: packaged `output/rime.dll`
- Packaged DLL SHA256: `6F6BABFD8C09EC1706D471457D7758D1D1F246D23D078992F3DD4ED1A6E2A6F2`
- Input sequence: `ngohaig`
- Client exit code: `0`
- Client stderr: empty
- Server remained running after `/stop`: `false`

## Evidence Files

- `client-out-excerpt.txt` - sanitized text excerpt from the stock client
  transcript. It records the progressive IPC replies, `status.schema_id=jyut6ping3`,
  `ctx.preedit` advancing from `n` through `ngohaig`, and candidate/context
  payload evidence. The original raw transcript includes serialized candidate
  bytes with embedded control characters and remains local under the ignored
  `target/` evidence directory.

## Scope Caveat

This is a real stock TypeDuck-Windows server/client IPC smoke, not an
interactive TSF typing smoke. It proves the existing TypeDuck-Windows backend
path can load packaged Yune and exchange key/context/candidate data through the
stock IPC client. It does not prove visible candidate-window rendering,
interactive typing into Notepad, or full TSF activation/deactivation behavior;
those are Phase 2 Windows product/frontend acceptance gates.
