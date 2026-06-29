# M47 RED-06 Native Memory Evidence

Date: 2026-06-29

Harness: `crates/yune-rime-api/tests/native_memory_probe.rs`, Windows
`PROCESS_MEMORY_COUNTERS_EX.WorkingSetSize` / `PrivateUsage` plus the test-local
counting allocator. These are Windows proxy measurements, not iOS
`phys_footprint`.

## Runs

| Folder | Probe config | Steady WS | Steady private | Allocator live | Allocator high-water | Peak WS |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| `after-reverse-drop-keyboard-proxy/` | Lean keyboard profile with RED-01/RED-03/RED-04 opt-outs | 58.0 MB | 23.3 MB | 16.0 MB | 22.7 MB | 62.9 MB |
| `after-reverse-drop-full-mobile/` | Full mobile profile, no memory opt-outs | 195.2 MB | 159.7 MB | 116.7 MB | 126.4 MB | 202.7 MB |
| `after-reverse-drop-committed-default/` | Committed default workspace | 268.6 MB | 241.7 MB | 185.1 MB | 415.1 MB | 487.7 MB |

## Verdict

RED-06 drops the primary reverse `.bin` byte buffer and parsed reverse dictionary
after advanced data is merged and before table advanced payload / compact-table
parsing. On the lean keyboard profile, fresh pre-change evidence in
`../m47-ios-keyboard-profile-pin-2026-06-29/red05-lean-keyboard-proxy/` measured
58.1 MB steady / 79.6 MB peak / 35.4 MB allocator high-water. This run measures
58.0 MB steady / 62.9 MB peak / 22.7 MB allocator high-water.

The remaining M47 blocker is steady retained memory, not the primary
create-session parse peak. The next branch should target the `normal_codes`
HashSet and unnamed compact-table descriptor heap.
