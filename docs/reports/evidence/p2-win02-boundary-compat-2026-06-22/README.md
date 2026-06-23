# P2-WIN-02 Boundary Compatibility Evidence

Status: implementation, TypeDuck-Web regression gates, package rebuild, direct DLL probe, and non-invasive Windows IPC smoke passed. Approved TSF/Notepad reruns did not reproduce the former Yune-backed server crash/hang; the remaining interactive blocker is classified as non-Yune TSF input-delivery/frontend-shell work because TypeDuck could be activated as the session profile while Notepad still received raw ASCII.

## Scope

This evidence covers the Yune-side TypeDuck `jyut6ping3` boundary fix for the Windows `ngohaig` path:

- raw `RimeCandidate.comment` bytes now match the TypeDuck `v1.1.2` `\f\r1,` payload shape for the top `ngohaig` rows;
- default upstream `RimeApi` and the 3-pointer `RimeCandidate` layout remain unchanged;
- TypeDuck lookup-filter compiled data can preserve the rich lookup records;
- the packaged DLL loads through the TypeDuck profile API and passes stock TypeDuck-Windows IPC smoke.

## Fixture And Code Evidence

- Added fixture: `crates/yune-core/tests/fixtures/typeduck-v1.1.2/jyut6ping3-windows-boundary-ngohaig.json`.
- Fixture provenance records TypeDuck-HK/librime `v1.1.2`, TypeDuck schema commit `1bed1ae6a0ab48055f073774d7dfd152a171c548`, and TypeDuck-Windows Phase 0C evidence commits `4ac2510` / `c7ddedb`.
- Core/API tests added:
  - `typeduck_v112_windows_boundary_ngohaig_fixture_is_locked`
  - `yune_jyut6ping3_ngohaig_comments_match_windows_boundary_oracle`
  - `yune_abi_jyut6ping3_ngohaig_comments_match_v112`
  - `yune_abi_compiled_lookup_jyut6ping3_ngohaig_comments_match_v112`
  - `yune_abi_repeated_jyut6ping3_session_lifecycle_stays_responsive`
  - `yune_abi_schema_open_tolerates_uninitialized_config_slot_like_typeduck_windows`
  - `workspace_update_rebuilds_typeduck_dictionary_lookup_filter_artifacts`
- Runtime fixes:
  - Rime comment-format replacement escapes decode `\f`, `\r`, `\n`, `\t`, `\v`, and `\\`.
  - `DictionaryLookupFilter` applies to `PartialTable` candidates so composed/sentence rows receive TypeDuck lookup details.
  - Compiled table writer/reader preserves TypeDuck lookup records in a `YUNE-LOOKUP` payload.
  - Deployment now discovers `dictionary_lookup_filter` dictionaries and rebuilds their compiled side artifacts.
  - Config ownership registry tolerates TypeDuck-Windows-style uninitialized `RimeConfig` slots instead of freeing foreign pointers.

## Package Evidence

Command:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\package-typeduck-windows.ps1
```

Result:

- Passed.
- Dynamic loader smoke `dynamic_loader_harness_loads_packaged_typeduck_profile_dll` passed.
- Fresh packaged DLL: `target\typeduck-windows-native\x86_64-pc-windows-msvc\dist\lib\rime.dll`
- Fresh packaged DLL SHA256: `A49E2CED354D837D18B22FDE72A50CFCA984BF8BAA0B18A8D2C3F3064C5D236D`
- Copied for local smoke to `C:\Users\laubonghaudoi\Documents\GitHub\TypeDuck-Windows\output\rime.dll`.

## Direct DLL Probe

The direct probe used:

- DLL: `C:\Users\laubonghaudoi\Documents\GitHub\TypeDuck-Windows\output\rime.dll`
- DLL SHA256: `A49E2CED354D837D18B22FDE72A50CFCA984BF8BAA0B18A8D2C3F3064C5D236D`
- `shared_data_dir`: missing `output\data`, to avoid source fallback;
- `user_data_dir`: `%APPDATA%\TypeDuck`, with the lookup side table rebuilt from the TypeDuck source dictionary using the patched Yune writer.

Probe result:

```text
session=1
select=1
getctx=1
preedit=ngohaig
cand_count=5
cand[0].text=我係個
cand[0].comment.hex4=0c0d312c
cand[1].text=我係
cand[1].comment.hex4=0c0d312c
cand[2].text=我喺
cand[2].comment.hex4=0c0d312c
cand[3].text=我
cand[3].comment.hex4=0c0d312c
```

The first candidate comment includes the composition row and component rows, with actual form-feed and carriage-return separators.

## TypeDuck-Windows IPC Smoke

The final smoke ran:

```powershell
"ngohaig`n" | C:\Users\laubonghaudoi\Documents\GitHub\TypeDuck-Windows\x64\Release\TestTypeDuckIPC.exe /console
```

Using:

- `TypeDuckServer.exe` from `C:\Users\laubonghaudoi\Documents\GitHub\TypeDuck-Windows\output`
- packaged Yune `output\rime.dll` SHA256 `A49E2CED354D837D18B22FDE72A50CFCA984BF8BAA0B18A8D2C3F3064C5D236D`
- temporary x64 WinSparkle SHA256 `CB13A9A6A88054A26E940A945D8E82F1CEBA5858F73A57636226A09B7CE5B4C4`

Result:

- `status.schema_id=jyut6ping3`
- `status.disabled=0`
- preedit advanced to `ngohaig`
- final `ctx.cand` serialized the rich rows starting with actual `\f\r1,` payloads
- server stopped through `/stop`
- no `TypeDuckServer`, `TestTypeDuckIPC`, or `TypeDuckDeployer` process remained afterward

Cleanup:

- Restored tracked `output\WinSparkle.dll`.
- Restored WinSparkle SHA256: `14D9ADA9FD17AD089D7DEA3A4B6E7117F132B23CD150323C60DF5FFDA5C72B6F`
- Verified TypeDuck-Windows git status only showed pre-existing untracked planning/workflow files.

## Interactive TSF/Notepad Smoke

The first approved Notepad smoke used the normal TypeDuck setup/deployer registration path and the same packaged `output\rime.dll` SHA256 `A49E2CED354D837D18B22FDE72A50CFCA984BF8BAA0B18A8D2C3F3064C5D236D`.

Captured files:

- `notepad-tsf-smoke.txt`
- `notepad-tsf-smoke.png`
- `notepad-tsf-output.txt`
- `notepad-tsf-smoke-windows-events.txt`
- `notepad-tsf-activate-profile.txt`
- `notepad-tsf-language-before.json`
- `notepad-tsf-language-after-set.json`
- `notepad-tsf-language-after-cleanup.json`
- `notepad-tsf-smoke-cleanup-retry.txt`
- `notepad-tsf-smoke-manual-cleanup.txt`

Setup and activation:

- `TypeDuckSetup.exe /t` exited `0`.
- `TypeDuckDeployer.exe /install` exited `0`.
- The TSF activation helper reported `CoCreateInstance(ITfInputProcessorProfileMgr)=0x00000000`, `ActivateProfile(flags=0x00000004)=0x00000000`, and active profile `langid=0x0c04`.
- `TypeDuckServer.exe` started and remained alive through the attempt.

Result:

- Notepad stayed responsive.
- `TypeDuckServer.exe` stayed alive.
- The captured clipboard and saved file content were raw `ngohaig `, UTF-8 hex `6e676f6861696720`.
- No TypeDuck candidate commit was captured, so this run is classified as inconclusive: the automation reached Notepad, but it did not prove the TSF key path was active in the editor.

Cleanup:

- The first uninstaller attempt was canceled at the elevation prompt.
- The retry returned `setup_uninstall_exit=0`, but left TypeDuck machine registration keys and system IME files present.
- After explicit approval, targeted TypeDuck-only manual cleanup removed the active TypeDuck Rime key, TSF TIP key, keyboard layout keys, and `TypeDuck.dll` / `TypeDuck.ime` from `System32` and `SysWOW64`.
- Post-cleanup verification shows the Windows user input list restored to `en-US` / `0409:00000409`, no TypeDuck processes, no active TypeDuck TIP/layout keys, and no active TypeDuck `.dll` / `.ime` files.
- `C:\Windows\System32\TypeDuck.dll.old.0` remains access-denied but is recorded twice in `PendingFileRenameOperations` for deletion on reboot. This matches the earlier TypeDuck setup/uninstall backup-file behavior and is not an active TIP, layout, IME file, or running process.

## Session-Activation TSF/Notepad Rerun

The second approved Notepad smoke used the same package and setup/deployer path, but replaced the earlier plain `ActivateProfile(flags=0x00000004)` helper with a throwaway session-scoped helper built under `target\p2-win02-tsf-harness\`. The helper calls `ITfInputProcessorProfileMgr::ActivateProfile` with `TF_IPPMF_FORSESSION | TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE` (`0x20000004`), matching the Phase 0C verifier reproduction.

Captured files:

- `notepad-tsf-smoke-session-activation.txt`
- `notepad-tsf-smoke-session-activation.png`
- `notepad-tsf-output-session-activation.txt`
- `notepad-tsf-smoke-session-activation-windows-events.txt`
- `notepad-tsf-session-activate-before-notepad.txt`
- `notepad-tsf-session-activate-after-focus.txt`
- `notepad-tsf-session-language-before.json`
- `notepad-tsf-session-language-after-set.json`
- `notepad-tsf-session-language-after-cleanup.json`
- `notepad-tsf-session-cleanup.txt`

Setup and activation:

- `TypeDuckSetup.exe /t` exited `0`.
- `TypeDuckDeployer.exe /install` exited `0`.
- Session activation before Notepad launch reported `ActivateProfile=0x00000000`, `active.type=1`, `active.langid=0x0c04`, and `active.flags=0x00000003`.
- Session activation after focusing the target Notepad window reported the same active TypeDuck profile state.
- `TypeDuckServer.exe` started and remained alive through the attempt.

Result:

- Notepad stayed responsive.
- `TypeDuckServer.exe` stayed alive.
- Virtual-key input for `ngohaig` plus Space still produced raw ASCII in Notepad: clipboard text `ngohaig `, UTF-8 hex `6e676f6861696720`.
- Windows Application log capture found no matching TypeDuck, Notepad, `rime.dll`, `DWrite`, or `ntdll` application errors.
- This is a stronger non-Yune classification than the first rerun: Yune's raw comment/session boundary is no longer the observed blocker; the remaining interactive issue is TSF/frontend input delivery or shell activation, because the TypeDuck profile is active at session scope but Notepad does not route the key stream into TypeDuck candidate processing.

Cleanup:

- The normal `TypeDuckSetup.exe /u` path returned `0`.
- After explicit approval, targeted TypeDuck-only manual cleanup removed active TypeDuck Rime, TSF TIP, keyboard layout, and system `.dll` / `.ime` files.
- Post-cleanup verification shows `en-US` / `0409:00000409`, no TypeDuck processes, no active TypeDuck TIP/layout keys, and no active TypeDuck `.dll` / `.ime` files.
- `C:\Windows\System32\TypeDuck.dll.old.0` and `C:\Windows\System32\TypeDuck.dll.old.1` remain access-denied backup files queued in `PendingFileRenameOperations` for deletion on reboot.

## Verification

Passed:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p yune-core table_writer_preserves_typeduck_lookup_records_for_compiled_path -- --nocapture
cargo test -p yune-core --test cantonese_parity -- --nocapture
cargo test -p yune-core --test cantonese_parity yune_jyut6ping3_ngohaig_comments_match_windows_boundary_oracle -- --nocapture
cargo test -p yune-rime-api workspace_update_rebuilds_typeduck_dictionary_lookup_filter_artifacts -- --nocapture
cargo test -p yune-rime-api --test typeduck_windows_boundary -- --nocapture
cargo test -p yune-rime-api --test typeduck_profile_abi_surface -- --nocapture
cargo test -p yune-rime-api --test dynamic_loader -- --nocapture
cargo test -p yune-rime-api --test typeduck_web -- --nocapture
cargo test --workspace
npm.cmd --prefix packages/yune-typeduck-runtime test
npm.cmd --prefix packages/yune-typeduck-runtime run build
powershell -ExecutionPolicy Bypass -File scripts\package-typeduck-windows.ps1
git diff --check
```

Notes:

- `typeduck_web` passed 28 tests. The real-assets oracle dictionary panel test printed its documented local TypeDuck `v1.1.2` oracle-build-assets skip message when those local assets were absent; the file still passed, and the P2-WIN-02 core/ABI boundary tests are active.
- A focused Playwright rerun was not used as proof from this worktree because `third_party/typeduck-web/source` is a junction to the original checkout; the worktree-valid TypeDuck-Web regression gate is the native adapter suite above.
- Approved Notepad TSF reruns were performed and cleaned up. The second rerun used session-scoped TypeDuck activation and classifies the remaining interactive failure as non-Yune TSF input-delivery/frontend-shell work, not as an unresolved Yune raw comment/session boundary bug.
