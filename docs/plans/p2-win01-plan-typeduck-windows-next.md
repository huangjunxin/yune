# P2-WIN-01 TypeDuck-Windows Next Product Plan

> **Status:** Draft / review requested - **Track:** Phase 2 Windows frontend product - **Created:** 2026-06-21 - **Type:** strategy and execution-gate plan
>
> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a modern Windows IME product for TypeDuck that uses Yune as the only runtime engine, while making an explicit repo/architecture decision before large implementation work starts.

**Architecture:** Treat Yune Phase 1 as the completed engine base. The Windows project is a Phase 2 product/frontend track: TSF integration, process/lifecycle, candidate UI, settings, installer, diagnostics, and product polish live in the Windows frontend repo, while Yune exposes the engine through the upstream-shaped default ABI plus the named TypeDuck profile ABI. The existing `TypeDuck-HK/TypeDuck-Windows` checkout is reference material, build evidence, and a possible source for platform-shell extraction, not a constraint on the new product design.

**Tech Stack:** Yune Windows package (`rime.dll` plus `rime_typeduck_profile_api.h`), Windows Text Services Framework (TSF), C++20 / C++/WinRT for low-level IME shell work unless a spike proves another language safer, Windows App SDK / WinUI 3 for modern settings and product UI, native HWND/DirectComposition-style rendering for the first low-latency candidate window, MSBuild or a consciously chosen replacement build system, installer/update tooling selected after audit.

---

## Boundary

This plan is deliberately **not** a Yune core-engine milestone.

- **In scope:** Windows IME frontend, TSF registration, session lifecycle, Yune package loading, candidate window, dictionary-panel display, settings UI, schema/profile controls, installer/update path, diagnostics, crash reporting, accessibility, high-DPI behavior, and release packaging.
- **In scope with Yune coordination:** any new frontend requirement that cannot be expressed through current `rime_get_api()` or `rime_get_typeduck_profile_api()` must become a named Yune API/profile proposal with tests before implementation.
- **Out of scope:** keeping librime as a runtime fallback, widening the default Yune `RimeApi`, preserving old Weasel UI architecture for its own sake, blindly porting historical commits, or treating upstream Weasel implementation details as product requirements.
- **Reference only:** old TypeDuck/librime and old TypeDuck-Windows behavior remain useful for product comparison, compatibility probes, and regression fixtures.

## Current Inputs

- Yune M10 proved that stock TypeDuck-Windows can load packaged Yune, create a session, process `ngohaig`, and return `status.schema_id=jyut6ping3` with candidate/context data through IPC.
- The local TypeDuck-Windows checkout is at `C:\Users\laubonghaudoi\Documents\GitHub\TypeDuck-Windows` on `dev`, with untracked planning/workflow artifacts already present. Do not sweep those into unrelated commits.
- Existing TypeDuck-Windows modules worth auditing:
  - `WeaselTSF/` - TSF text service boundary.
  - `WeaselServer/` and `WeaselIPCServer/` - server process and IPC lifecycle.
  - `WeaselIPC/` - client/server wire format helpers.
  - `RimeWithWeasel/` - current RIME engine adapter.
  - `WeaselUI/` - candidate window and multi-hint panel behavior.
  - `WeaselDeployer/` and `WeaselSetup/` - settings, deployer, installer.
  - `INTEGRATION_PLAN.md`, `LIBRIME_INTEGRATION_PLAN.md`, and `docs/commit-inventory.md` - historical modernization analysis.

## Strategic Recommendation

The product direction should be **Yune-only**, not dual-engine. A librime fallback would preserve technical debt and make product bugs harder to classify.

The repo direction should be **decision-gated**, with this default bias:

1. **Prefer a fresh Windows product repo if the audit can isolate a small platform shell.** This gives us a clean architecture, modern UX, and a product identity not constrained by old Weasel design.
2. **Do not start from a blank TSF implementation unless a spike proves it is cheap.** TSF registration, edit sessions, key sinks, composition, language bar behavior, IPC, installer registration, and cleanup are high-risk platform glue. The existing TypeDuck-Windows repo has working examples and should be mined aggressively.
3. **Use a hybrid extraction path if needed.** Start a fresh repo, but transplant or adapt the smallest proven slices of `WeaselTSF`, IPC, installer registration, and smoke harnesses until a native Yune-first shell is stable. Keep old UI/settings design out unless it earns its place.
4. **Continue in the existing repo only if audit proves extraction is riskier than in-place modernization.** In-place work is acceptable if it is the fastest route to a shippable Yune-only Windows IME, but it should still delete librime fallback assumptions and old architecture debt as explicit milestones.

## Platform Stack Notes

- Microsoft TSF is the official Windows text-service framework for IMEs and language input services; the actual IME boundary should treat TSF as the platform contract.
- Windows App SDK / WinUI 3 is the modern native Windows desktop UI path and is a strong default for settings, diagnostics, setup helpers, and product windows.
- WebView2 can embed web UI in native Windows apps, but it adds runtime/distribution and process-model considerations. Use it only if a spike proves web UI reuse is worth the dependency; do not use it for the first candidate window unless latency, focus, IME positioning, and accessibility are proven.
- Candidate UI should start as a small native renderer with deterministic layout, high-DPI support, and clear test hooks. It can become prettier after it is stable.

## Phase Plan

### Phase 0 - Audit And Repo Decision

**Goal:** Decide fresh repo, existing repo, or hybrid extraction with evidence.

**Files to read in TypeDuck-Windows:**
- `INTEGRATION_PLAN.md`
- `LIBRIME_INTEGRATION_PLAN.md`
- `docs/commit-inventory.md`
- `README.md`
- `INSTALL.md`
- `WeaselTSF/**`
- `WeaselServer/**`
- `WeaselIPC*/**`
- `RimeWithWeasel/**`
- `WeaselUI/**`
- `WeaselDeployer/**`
- `WeaselSetup/**`

**Required outputs:**
- `docs/windows-next-audit.md` in the Windows working repo.
- `docs/windows-next-repo-decision.md` in the Windows working repo.

**Checklist:**
- [ ] Record the current branch, remote, dirty files, and untracked planning artifacts.
- [ ] Build or reproduce the current TypeDuck-Windows baseline from a clean command sequence.
- [ ] Re-run the Yune-backed M10 stock IPC smoke and save the evidence path.
- [ ] Classify each existing module as `reuse`, `rewrite`, `reference-only`, or `delete`.
- [ ] Decide repo path:
  - `fresh-repo` if TSF/IPC/installer slices can be extracted cleanly.
  - `existing-repo` if the platform shell is too intertwined to extract safely.
  - `hybrid-extraction` if a fresh repo is still desired but the first milestone imports minimal shell code.
- [ ] Name the first executable smoke target before product UI work starts.

**Acceptance gate:** A reviewer can read the audit and understand exactly why the repo strategy was chosen, which old modules are allowed to survive, and how Yune is loaded.

### Phase 1 - Yune Host Contract Spike

**Goal:** Build the smallest Windows executable that loads packaged Yune and drives a real session without old frontend UI.

**Expected components:**
- `YuneHost` wrapper around `rime.dll`, `rime_get_api()`, and `rime_get_typeduck_profile_api()`.
- deterministic lifecycle: setup, initialize, deploy, create session, select `jyut6ping3`, process keys, read status/context/candidates, destroy session, cleanup.
- log file with package path, schema id, session id, key events, candidate count, and errors.

**Checklist:**
- [ ] Load the Yune package produced by `scripts/package-typeduck-windows.ps1`.
- [ ] Reject startup if the TypeDuck profile accessor is missing.
- [ ] Create a session and confirm `status.schema_id=jyut6ping3`.
- [ ] Process `ngohaig` and record candidate/context data.
- [ ] Process a reverse-lookup probe such as `` `zhe `` if the packaged schema assets support it.
- [ ] Save stdout/log evidence under a deterministic evidence directory.

**Acceptance gate:** A console smoke can prove Yune-only engine lifecycle without TSF, tray UI, settings UI, or installer.

### Phase 2 - Minimal TSF Shell

**Goal:** Register a Windows IME text service that forwards key events to the Yune host and commits text into real applications.

**Working assumption:** C++20 / C++/WinRT is the default because TSF is COM-heavy and existing Weasel/TypeDuck examples are C++.

**Checklist:**
- [ ] Implement or extract TSF registration and unregistration.
- [ ] Implement key event sink, composition lifecycle, edit session commit, and cleanup.
- [ ] Route key events to the Phase 1 Yune host.
- [ ] Keep all UI optional; first smoke can use logs and simple candidate data.
- [ ] Test in Notepad and one Chromium-based app.
- [ ] Verify install, activate IME, type `ngohaig`, commit candidate, deactivate, uninstall, and reboot-safe cleanup.

**Acceptance gate:** A minimal installed IME can type through Yune in real Windows text fields and can be removed cleanly.

### Phase 3 - Candidate Window V1

**Goal:** Replace log-only candidate display with a fast, stable, readable candidate window.

**Checklist:**
- [ ] Render horizontal and vertical candidate list modes.
- [ ] Support page size, page navigation, highlighted candidate, numbering, and selection.
- [ ] Render candidate comments and dictionary-panel details without inline control markers.
- [ ] Support high-DPI scaling, multi-monitor positioning, dark/light themes, and IME focus transitions.
- [ ] Keep the first candidate window native and small; do not introduce a broad UI framework here before latency/focus are proven.

**Acceptance gate:** Candidate UI remains correctly positioned and responsive while typing in Notepad, Chromium, Office-like rich text, and at least one high-DPI display setting.

### Phase 4 - Settings And Product UI

**Goal:** Build a modern TypeDuck settings experience without carrying old Weasel settings debt.

**Working assumption:** WinUI 3 / Windows App SDK is the default for settings and diagnostics.

**Checklist:**
- [ ] Cantonese-first labels with clear English support text.
- [ ] Schema selection and current engine status.
- [ ] TypeDuck profile controls mapped to real Yune config/profile behavior.
- [ ] Candidate UI controls: horizontal/vertical, page size, fonts, comments, Jyutping display, dictionary panel.
- [ ] Diagnostics page: engine version, schema versions, package path, user data dir, last deploy result, log open/export.
- [ ] Safe reset/redeploy controls with confirmation.

**Acceptance gate:** A non-developer can configure the IME without editing YAML, and every visible control either changes a real setting or is absent.

### Phase 5 - Packaging, Installer, Updates, And Diagnostics

**Goal:** Make the IME installable, diagnosable, and maintainable outside a developer shell.

**Checklist:**
- [ ] Package Yune runtime, schema assets, TSF shell, server/host process, settings app, and candidate UI.
- [ ] Install and register TSF service with clear rollback on failure.
- [ ] Uninstall cleanly, including service shutdown and registry cleanup.
- [ ] Add versioned logs and a support bundle export.
- [ ] Decide update mechanism after security review; do not add auto-update silently.
- [ ] Add crash/error handling that produces actionable diagnostics without leaking private typed content by default.

**Acceptance gate:** Fresh install, upgrade, uninstall, and support-bundle export work on a non-development Windows user account.

### Phase 6 - Product Hardening

**Goal:** Turn the prototype into a daily-usable Windows IME.

**Checklist:**
- [ ] App compatibility matrix: Notepad, Chromium, Edge, Office, VS Code, terminal, Electron apps, UWP/WinUI apps.
- [ ] Accessibility matrix: screen-reader behavior, keyboard-only settings navigation, contrast, high-DPI.
- [ ] Performance matrix: first activation, first key latency, candidate render latency, deploy time, memory use.
- [ ] Failure-mode matrix: missing schema, corrupt user data, Yune DLL missing, profile accessor missing, IPC server crash, settings write failure.
- [ ] Release checklist: signing decision, installer artifact, changelog, known issues, rollback path.

**Acceptance gate:** The product can be dogfooded as a daily IME with known issues documented and no critical install/uninstall or text-entry failures.

## Repo Decision Matrix

| Choice | Choose when | Avoid when |
|---|---|---|
| Fresh repo | We can isolate TSF/IPC/installer shell contracts; old UI/settings are mostly debt; product UX needs a clean architecture | TSF/installer extraction becomes the main project and blocks basic typing |
| Existing repo | The old platform shell is too intertwined to extract safely; build/release path already works; fastest daily-dogfood path matters most | In-place work keeps old librime assumptions, stale UI architecture, or branch drift as permanent constraints |
| Hybrid extraction | We want fresh architecture but need proven TSF/IPC/installer pieces to reduce risk | The extracted code becomes an unreviewed copy of the old repo with a new name |

**Default recommendation for review:** pursue **hybrid extraction toward a fresh Yune-first repo**, unless Phase 0 proves the old shell cannot be extracted safely. This honors the product goal of doing better than Rime/Weasel while avoiding a blind TSF rewrite.

## Claude Review Prompt

Send this message for a second opinion:

```text
We have finished Phase 1 of Yune: a Rust RIME-compatible engine with basic oracle parity for the named targets. Upstream behavior is pinned to rime/librime 1.17.0; TypeDuck/Cantoboard fork behavior is isolated as the TypeDuck jyut6ping3 profile against TypeDuck-HK/librime v1.1.2. M10 also proved TypeDuck-Windows can load packaged Yune and pass a stock server/client IPC smoke through the named TypeDuck profile ABI.

Now we are planning Phase 2: a Windows IME product/frontend for TypeDuck that switches fully to Yune. I do not want to keep librime as a runtime fallback. I also do not want to be constrained by old Weasel/Rime Windows UX or architecture if we can build something better.

Question: should we continue developing inside the existing TypeDuck-HK/TypeDuck-Windows repo, start a brand-new Windows IME repo, or use a hybrid approach where we start fresh but extract only the proven TSF/IPC/installer platform shell from TypeDuck-Windows?

Please review this from first principles:
- Windows IME/TSF technical risk
- candidate window and settings UI architecture
- installer/update/diagnostics risk
- how much of TypeDuck-Windows/Weasel should be reused versus treated as reference
- whether WinUI 3 / Windows App SDK should be used for settings/product UI
- whether WebView2 is appropriate anywhere
- how to keep Yune's default upstream ABI clean while using the TypeDuck profile ABI
- what the first 3 closeable milestones should be

My current leaning is: Yune-only engine, no librime fallback; prefer a fresh Yune-first product repo if audit proves the TSF/IPC/installer shell can be isolated; otherwise use a hybrid extraction path. Please challenge this and propose a concrete plan with decision gates.
```

## Open Questions For Review

- Should the first production candidate window be native only, or can WebView2/WinUI safely handle candidate rendering without focus/latency problems?
- Is C++/WinRT the best long-term TSF shell language, or should a Rust `windows-rs` spike be attempted after the C++ baseline is understood?
- Should the settings app be a separate WinUI executable, part of the server process, or launched through a tray/deployer process?
- Should the first repo be private/new while the old TypeDuck-Windows repo remains the reference, or should the existing repo receive the new architecture directly?
- What signing, installer, and update policy is acceptable for early dogfooding?

## Non-Negotiables

- Yune is the only runtime engine.
- The default Yune `rime_get_api()` ABI remains upstream-shaped.
- TypeDuck fork-only behavior stays behind named profile/API surfaces.
- Candidate/output behavior changes require oracle or dogfood evidence, not visual guesswork.
- The old TypeDuck-Windows repo is useful evidence, but the product goal is a better Windows IME, not a Weasel clone.
