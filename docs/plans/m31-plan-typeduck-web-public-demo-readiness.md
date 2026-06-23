# M31 TypeDuck-Web Public Demo Readiness Implementation Plan

> **Status:** Planned - **Milestone:** M31 (TypeDuck-Web public demo readiness) - **Created:** 2026-06-22 - **Type:** execution plan
>
> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the internal TypeDuck-Web dogfood harness ready to publish as a public Yune engine demo, with owner-approved TypeDuck assets, payload pruning, browser-honest output-standard support only where needed, and a reproducible Cloudflare deployment path.

**Architecture:** M31 is a deterministic web-demo readiness milestone, not a new AI product milestone and not the primary Windows product track. TypeDuck-Web shell/dictionary redistribution is owner-approved by the project owner, who is also the TypeDuck author, so this plan does not carry a separate licensing blocker. Keep a short public provenance note so visitors understand the page is exercising the Yune engine through a TypeDuck-Web-derived harness. Then add only the OpenCC output standards Yune can prove and that the public demo actually needs. Finally make the patched TypeDuck-Web source reproducible from checked-in Yune state, pin and prune the WASM/schema assets, and publish through a Cloudflare static-assets deployment with smoke evidence.

**Tech Stack:** Rust (`yune-core`, `yune-rime-api`), OpenCC source dictionaries/config chains, `packages/yune-typeduck-runtime`, TypeDuck-Web Vite + React + Tailwind source under `third_party/typeduck-web/source/`, Playwright, Wrangler, and Cloudflare Workers Static Assets or Pages deployment.

---

## Status

Planned. P2-WIN-02 remains the next Yune-side unblocker and P2-WIN-01 remains the primary product track after that. M31 may proceed as optional/parallel public-demo readiness only when it does not compete with the Windows product work. Do not start public deployment until P2-WIN-02 is complete, payload size is measured and pruned, and the milestone record states that TypeDuck-Web shell/dictionary use is owner-approved.

## Scope

In scope:

- OpenCC output-standard support that is real in Yune, not only a browser label.
- Runtime and TypeDuck-Web UI for supported output standards.
- Public provenance note stating that TypeDuck-Web shell/dictionary use is owner-approved by the TypeDuck author and that the page is a Yune engine demo.
- Public payload pruning so the deployed demo ships only the assets needed for the public surface.
- Reproducible reconstruction of the patched `third_party/typeduck-web/source/` checkout from checked-in lock/patch/bridge state.
- Release WASM and schema asset packaging for a public demo.
- Cloudflare deployment config, local preview, deployed smoke evidence, and docs.
- AI posture gate: default-off, local-only, no remote calls, no telemetry, no secrets, and AI-off byte identity.

Out of scope:

- Richer AI-native UX or remote-provider work. That belongs to M32.
- Windows TSF/frontend work. That belongs to P2-WIN-01 after P2-WIN-02.
- Candidate ranking, Jyutping composition, TypeDuck comment-byte compatibility, `RimeApi`, or `RimeCandidate` changes unless a focused OpenCC fixture proves the need.
- Treating `typeduck.hk/web` as an oracle. It can be a comparison target only.
- Splitting `third_party/typeduck-web/` into a separate repository or submodule.
- Blocking P2-WIN-01 on Cloudflare/demo polish.

## Preconditions

- M30 is complete and the current branch is synced with `origin/main`.
- P2-WIN-02 is complete before public deployment. Engine/UI work may be prepared earlier only if the plan record says deployment is blocked pending P2-WIN-02.
- P2-WIN-01 keeps product priority after P2-WIN-02. If M31 conflicts with Windows product work, pause M31 unless the user explicitly chooses public web visibility first.
- TypeDuck-Web shell and dictionary use is owner-approved by the project owner, who is also the TypeDuck author. No separate M31 licensing clearance gate is required.
- Cloudflare deployment docs are checked at execution time. As of the M31 plan date, Cloudflare recommends Workers Static Assets for new static/full-stack Worker apps, React + Vite examples use `wrangler.jsonc`, and Wrangler JSON config is recommended for new projects.

## Acceptance Gates

- `M31-PUBLIC-00`: Public provenance is recorded before deployment. The milestone notes that TypeDuck-Web shell/dictionary use is owner-approved by the TypeDuck author, and the public page identifies that it is exercising the Yune engine through a TypeDuck-Web-derived harness.
- `M31-PUBLIC-01`: P2-WIN-02 status and P2-WIN-01 priority are checked before deployment; public deploy is blocked or explicitly risk-noted if P2-WIN-02 is still active, and M31 pauses if it would pull effort away from the Windows product track.
- `M31-PUBLIC-02`: Every exposed OpenCC output standard has engine/runtime/browser evidence. Unsupported standards are absent from the UI or shown as unavailable with a reason.
- `M31-PUBLIC-03`: The output-standard control changes candidate/commit output through Yune, not a browser-only postprocessor.
- `M31-PUBLIC-04`: The public demo can be rebuilt from checked-in Yune state: lock file, patch, Yune bridge, WASM, schema assets, and build commands.
- `M31-PUBLIC-05`: Cloudflare local preview and deployed smoke cover app boot, WASM load, schema asset load, `jyut6ping3_mobile` typing, OpenCC standard toggle, and route/base-path behavior.
- `M31-PUBLIC-06`: AI stays default-off/local-only; no remote calls, telemetry, secrets, or third-party model keys are present in the public build. AI-off output is byte-identical to the classic path.
- `M31-PUBLIC-07`: Public payload is measured, pruned, and documented. The deployed bundle ships only the schemas, compiled assets, dictionaries, fonts, and static files needed by the public surface; source dictionaries and unused schemas are excluded unless the runtime requires them.
- `M31-PUBLIC-08`: Rust, runtime, TypeDuck-Web build, Playwright, patch reverse/forward, and `git diff --check` gates pass.

## File Responsibilities

- `crates/yune-core/src/filter/mod.rs`: owns OpenCC-backed conversion behavior and filter wiring.
- `crates/yune-rime-api/src/schema_install.rs`: owns schema option/config installation into browser/runtime deploy state.
- `crates/yune-rime-api/tests/typeduck_web.rs`: owns native runtime contract tests for browser-facing options.
- `packages/yune-typeduck-runtime/`: owns TypeScript wrapper and response/control contract.
- `third_party/typeduck-web/yune-integration/`: owns the Yune bridge layered over TypeDuck-Web.
- `third_party/typeduck-web/source/`: owns the patched demo app UI; changes here require patch regeneration.
- `third_party/typeduck-web/patches/yune-typeduck-runtime.patch`: committed representation of source changes.
- `third_party/typeduck-web/e2e/`: owns browser evidence and public-demo smoke tests.
- `third_party/typeduck-web/cloudflare/` or an equivalent committed deployment folder: owns Cloudflare config, worker entry, and deployment notes.
- `third_party/typeduck-web/public-demo/PROVENANCE.md`: records owner-approved TypeDuck-Web shell/dictionary use and the Yune engine demo positioning.
- `third_party/typeduck-web/public-demo/asset-manifest.md`: owns public payload inventory and pruning decisions.
- `docs/roadmap.md` and `docs/requirements.md`: own milestone status and requirement traceability.

---

## Task 0 - Product Priority, Provenance, And Payload Checkpoint

**Files:**

- Read: `docs/roadmap.md`
- Read: `docs/plans/p2-win02-plan-typeduck-boundary-compat.md`
- Read: `docs/plans/archive/m30-plan-engine-representation-performance.md`
- Create: `third_party/typeduck-web/public-demo/PROVENANCE.md`
- Create: `third_party/typeduck-web/public-demo/asset-manifest.md`
- Create evidence: `third_party/typeduck-web/e2e/results/m31-public-demo/public-readiness-gate.md`

- [ ] **Step 0.1: Confirm repository and dependency state**

Run:

```powershell
git fetch origin --prune
git status --short --branch
git log --oneline -5 --decorate
```

Expected:

- Worktree is clean or unrelated active changes are identified before editing.
- `origin/main` contains the completed M30 closeout.

- [ ] **Step 0.2: Check Windows product priority and P2-WIN-02 deployment gate**

Run:

```powershell
rg -n "P2-WIN-01|P2-WIN-02|TypeDuck Windows boundary compatibility|M31|M32|primary product" docs\roadmap.md docs\plans\p2-win01-plan-typeduck-windows-next.md docs\plans\p2-win02-plan-typeduck-boundary-compat.md
```

Expected:

- If P2-WIN-02 is complete, M31 can proceed through deployment.
- If P2-WIN-02 is active, M31 may complete local engine/UI/build work, but Task 5 deployed smoke must remain blocked and the closeout must say why.
- If P2-WIN-01 is ready and actively staffed, pause M31 unless the user explicitly chooses public web visibility first.

- [ ] **Step 0.3: Record owner-approved TypeDuck provenance**

Run:

```powershell
git status --short --branch
rg -n "TypeDuck|Yune engine|public demo|owner-approved" docs\roadmap.md docs\plans\m31-plan-typeduck-web-public-demo-readiness.md
```

Expected:

- `PROVENANCE.md` states that TypeDuck-Web shell and TypeDuck dictionary assets are owner-approved for this Yune-hosted demo because the project owner is the TypeDuck author.
- The public UI can use a TypeDuck/TypeDuck-Web-derived shell, but the milestone record still labels the technical purpose clearly: test and demonstrate the Yune engine.
- No courtesy-contact or license-clearance task is required for TypeDuck-owned assets in this milestone.

- [ ] **Step 0.4: Measure the asset payload before choosing the deployment surface**

Run:

```powershell
Get-ChildItem -Recurse third_party\typeduck-web\source\public\schema -File | Measure-Object Length -Sum
Get-ChildItem -Recurse third_party\typeduck-web\source\public -File | Sort-Object Length -Descending | Select-Object -First 20 FullName,Length
```

Expected:

- `asset-manifest.md` records total public asset bytes, largest files, and which schema/dictionary/font/assets are intended for deployment.
- The public bundle plan prefers the minimum viable surface: `jyut6ping3_mobile` plus required compiled assets, public WASM/JS/CSS, and only dictionaries/assets needed at runtime.
- Source dictionaries, unused schemas, and dev/test-only assets are excluded unless the runtime requires them.

## Task 1 - Scope OpenCC Output Standards From Real Demo Need

**Files:**

- Inspect: `crates/yune-core/src/filter/mod.rs`
- Inspect: `crates/yune-rime-api/src/schema_install.rs`
- Inspect: `crates/yune-core/tests/cantonese_parity.rs`
- Inspect: OpenCC data under the Yune source tree
- Create: `third_party/typeduck-web/e2e/results/m31-public-demo/opencc-support-audit.md`

- [ ] **Step 1.1: Decide whether OpenCC breadth is required for the first public demo**

Record one of these decisions in `opencc-support-audit.md`:

- `opencc_scope = current_simplification_only`: the first public demo needs only the already-proven `simplification`/`hk2s` path. Do not add broad OpenCC matrix work in M31; proceed to packaging/deployment with the existing real selector.
- `opencc_scope = add_limited_supported_standards`: add only the output standards backed by checked-in Yune data and required for the public demo.
- `opencc_scope = split_follow_up`: broader OpenCC output standards are valuable but too large for M31; create a follow-up plan and do not block the public demo on them.

Expected:

- M31 does not implement broad OpenCC support just because a UI label would be nice.
- No unsupported standard is shown as an enabled live option.

- [ ] **Step 1.2: Inventory available OpenCC data/config chains**

Run:

```powershell
rg -n "hk2s|opencc|simplification|traditional|tw|hk|s2t|t2s" crates docs packages third_party\typeduck-web -g "!third_party/typeduck-web/source/node_modules/**"
rg --files | rg "opencc|hk2s|s2t|t2s|tw|hk"
```

Expected:

- The audit lists which conversion chains are backed by checked-in data.
- The audit distinguishes proven support from desired UI labels.

- [ ] **Step 1.3: Choose the M31 exposed standard set**

Record the supported standards in `opencc-support-audit.md` using this table:

```markdown
| Standard id | User label | Engine support | Data/config chain | M31 UI status | Evidence owner |
| --- | --- | --- | --- | --- | --- |
| keep_original | Original output | no conversion | none | enabled | browser smoke |
| simplified | Simplified Chinese | supported only if chain is present | exact checked-in chain | enabled or blocked | native + browser |
| hong_kong_traditional | Hong Kong Traditional | supported only if chain is present | exact checked-in chain | enabled or blocked | native + browser |
| taiwan_traditional | Taiwan Traditional | supported only if chain is present | exact checked-in chain | enabled or blocked | native + browser |
```

Expected:

- No standard is enabled unless Yune can apply it through engine/runtime code.
- Labels are browser-facing, but conversion stays engine-owned.
- If only the existing simplification path is used, the audit states that M31's OpenCC work is satisfied by preserving and documenting the existing real control rather than adding a larger selector.

## Task 2 - Add Engine And Runtime OpenCC Support

**Files:**

- Modify: `crates/yune-core/src/filter/mod.rs`
- Modify: `crates/yune-rime-api/src/schema_install.rs`
- Modify: `crates/yune-rime-api/tests/typeduck_web.rs`
- Modify only if needed: `crates/yune-core/tests/cantonese_parity.rs`
- Create evidence: `third_party/typeduck-web/e2e/results/m31-public-demo/opencc-native-gates.md`

Skip code changes in this task if Task 1 records `opencc_scope = current_simplification_only`. In that case, record the existing native/browser evidence owner and proceed to Task 3 without adding a broader output-standard selector.

- [ ] **Step 2.1: Add failing native tests for each enabled standard**

Add focused tests whose names encode the exposed standards, for example:

```rust
#[test]
fn m31_typeduck_web_output_standard_simplified_changes_candidate_output() {
    // Drive the real TypeDuck-Web runtime path.
    // Assert the selected output standard changes candidate or commit text.
    // Assert disabling the standard returns to the original output.
}

#[test]
fn m31_typeduck_web_output_standard_unsupported_values_are_rejected() {
    // Drive the same runtime option/customize path.
    // Assert unknown standard ids fail or are ignored without mutating output.
}
```

Run:

```powershell
cargo test -p yune-rime-api --test typeduck_web m31_typeduck_web_output_standard -- --nocapture
```

Expected before implementation:

- New tests fail because the standard selector is not yet wired.

- [ ] **Step 2.2: Implement the smallest engine-owned selector**

Wire output-standard selection so the runtime controls Yune conversion state instead of transforming text in React. Preserve existing `simplification` behavior and add a normalized setting such as `output_standard` only if the existing option cannot express the supported standards clearly.

Required behavior:

- Unknown standard ids do not panic.
- Switching standards while composing refreshes candidates through the existing engine path.
- Candidate text, commit text, and dictionary comments remain valid UTF-8.
- Classic behavior is unchanged when the standard is `keep_original`.

- [ ] **Step 2.3: Run focused OpenCC gates**

Run:

```powershell
cargo fmt --check
cargo test -p yune-rime-api --test typeduck_web m31_typeduck_web_output_standard -- --nocapture
cargo test -p yune-core --test cantonese_parity -- opencc --nocapture
```

Expected:

- Focused tests pass.
- If `cantonese_parity` has no matching `opencc` tests, record `no matching cantonese_parity opencc tests` in `opencc-native-gates.md` and rely on the new `typeduck_web` runtime tests.

## Task 3 - Expose Output Standards In TypeDuck-Web

**Files:**

- Modify: `packages/yune-typeduck-runtime/src/`
- Modify: `third_party/typeduck-web/yune-integration/`
- Modify: `third_party/typeduck-web/source/src/`
- Modify: `third_party/typeduck-web/e2e/yune-typeduck.spec.ts`
- Regenerate: `third_party/typeduck-web/patches/yune-typeduck-runtime.patch`
- Create evidence: `third_party/typeduck-web/e2e/results/m31-public-demo/opencc-browser-evidence.md`

Only add a new output-standard selector if Task 1 chooses `opencc_scope = add_limited_supported_standards`. If Task 1 chooses `current_simplification_only`, this task preserves the existing real simplification control, adds public-demo documentation/evidence for it, and does not invent disabled placeholder standards.

- [ ] **Step 3.1: Add runtime wrapper support**

Add a typed API for the output-standard setting in `packages/yune-typeduck-runtime` and the Yune integration bridge only if the existing `simplification` option cannot express the Task 1 standard set. Use string ids from `opencc-support-audit.md`; do not expose unimplemented ids.

Run:

```powershell
npm.cmd --prefix packages\yune-typeduck-runtime test
npm.cmd --prefix packages\yune-typeduck-runtime run build
```

Expected:

- Runtime tests/build pass.

- [ ] **Step 3.2: Add a minimal UI control**

Add an output-standard control under display/browser-facing controls. Requirements:

- Label explains that it changes Chinese output standard.
- Enabled options are exactly the supported standard ids from Task 1.
- Unsupported standards are absent, or disabled with a short reason.
- The control does not trigger a full engine reload unless the runtime evidence proves deploy-time state is required.

- [ ] **Step 3.3: Add browser evidence**

Add Playwright coverage that:

- loads the app from a fresh build
- types a stable input with visible candidates
- toggles each enabled output standard
- verifies candidate or commit output changes where expected
- verifies `keep_original` returns to baseline
- verifies no literal OpenCC control/debug marker appears in the UI

Run:

```powershell
npm.cmd --prefix third_party\typeduck-web\source run build
npm.cmd --prefix third_party\typeduck-web\e2e run test:e2e -- --grep "M31 OpenCC" --workers=1
```

Expected:

- Focused browser test passes and writes screenshots/state JSON under `third_party/typeduck-web/e2e/results/m31-public-demo/`.

## Task 4 - Make The Public Demo Build Reproducible

**Files:**

- Modify or create: `third_party/typeduck-web/typeduck-web.lock.json`
- Modify: `third_party/typeduck-web/patches/yune-typeduck-runtime.patch`
- Create: `third_party/typeduck-web/public-demo/README.md`
- Create or update: `third_party/typeduck-web/public-demo/PROVENANCE.md`
- Create or update: `third_party/typeduck-web/public-demo/asset-manifest.md`
- Create: `third_party/typeduck-web/public-demo/build.ps1`
- Create evidence: `third_party/typeduck-web/e2e/results/m31-public-demo/reproducible-build.md`

- [ ] **Step 4.1: Capture the source reconstruction contract**

`public-demo/README.md` must state:

- which TypeDuck-Web upstream revision the source checkout is based on
- which patch file is applied
- which Yune integration files are copied or referenced
- where release WASM and schema assets come from
- how `/web/` versus root deployment is configured
- how the public page describes the relationship between TypeDuck-Web and the Yune engine
- where the TypeDuck owner-approved provenance note lives

- [ ] **Step 4.2: Add a reproducible build script**

Create `public-demo/build.ps1` that performs these actions:

```powershell
param(
  [string] $OutputDir = "third_party\typeduck-web\public-demo\dist"
)

$ErrorActionPreference = "Stop"
npm.cmd --prefix packages\yune-typeduck-runtime run build
npm.cmd --prefix third_party\typeduck-web\source run build
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
Copy-Item -Recurse -Force third_party\typeduck-web\source\dist\* $OutputDir
```

After writing the initial script, run `rg --files third_party\typeduck-web\source\dist | rg "wasm|schema|js|css"` and add explicit `Copy-Item` lines for any release WASM or schema assets that are not already present in `source\dist`. Prune the deployed asset set before preview:

- Ship only the public schemas exposed by the UI, starting with `jyut6ping3_mobile` unless Task 1 deliberately exposes more.
- Prefer compiled runtime assets required by the browser. Do not ship source `*.dict.yaml` files unless the runtime requires them.
- Do not ship `cangjie`, `luna_pinyin`, `loengfan`, test fixtures, or dev-only source dictionaries unless the public UI exposes them.
- Keep `PROVENANCE.md` and `asset-manifest.md` in the public-demo source folder; include them in the deployed static assets if the public page links to them.

Do not copy build output into git unless the project already tracks that exact artifact type.

- [ ] **Step 4.3: Record public asset manifest**

Run after the public-demo build:

```powershell
Get-ChildItem -Recurse third_party\typeduck-web\public-demo\dist -File | Measure-Object Length -Sum
Get-ChildItem -Recurse third_party\typeduck-web\public-demo\dist -File | Sort-Object Length -Descending | Select-Object -First 30 FullName,Length
```

Expected:

- `asset-manifest.md` records total deployed bytes, largest files, and why each schema/dictionary asset is present.
- The manifest explicitly states which potentially large source dictionaries were excluded.
- The evidence notes the current Cloudflare file-size and asset limits checked in Task 5.1 before claiming deployment feasibility.

- [ ] **Step 4.4: Verify patch discipline**

Run:

```powershell
git -C third_party\typeduck-web\source diff --binary --submodule=diff > third_party\typeduck-web\patches\yune-typeduck-runtime.patch
git -C third_party\typeduck-web\source apply --reverse --check ..\patches\yune-typeduck-runtime.patch
```

Then forward-check the patch in a clean detached checkout at the revision named by `typeduck-web.lock.json`.

Expected:

- Reverse and forward checks pass.
- `reproducible-build.md` records the exact commands and source revision.

## Task 5 - Add Cloudflare Deployment

**Files:**

- Create: `third_party/typeduck-web/public-demo/wrangler.jsonc`
- Create or modify: `third_party/typeduck-web/public-demo/worker/index.ts`
- Create evidence: `third_party/typeduck-web/e2e/results/m31-public-demo/cloudflare-smoke.md`
- Modify: `third_party/typeduck-web/e2e/yune-typeduck.spec.ts`

- [ ] **Step 5.1: Verify Cloudflare docs before writing config**

Open current Cloudflare docs and confirm:

- Workers Static Assets or Pages is the intended deployment target.
- `wrangler.jsonc` is still recommended for new Wrangler config.
- SPA fallback and static asset directory syntax are current.

Record the documentation URLs and access date in `cloudflare-smoke.md`.
Also record the current maximum static-asset/file-size limits and compare them with `asset-manifest.md`.

- [ ] **Step 5.2: Add local preview config**

Create `wrangler.jsonc` with:

```jsonc
{
  "$schema": "../../source/node_modules/wrangler/config-schema.json",
  "name": "yune-engine-web-demo",
  "compatibility_date": "2026-06-22",
  "assets": {
    "directory": "./dist",
    "not_found_handling": "single-page-application"
  }
}
```

Adjust paths only after verifying the actual public-demo folder layout. Keep the config committed; do not store account ids, tokens, or secrets.

- [ ] **Step 5.3: Run local Cloudflare preview**

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File third_party\typeduck-web\public-demo\build.ps1
npx.cmd wrangler dev --config third_party\typeduck-web\public-demo\wrangler.jsonc --local
```

Expected:

- Preview serves the app.
- WASM loads with the correct MIME behavior.
- Schema assets load.
- `/web/` and root routing follow the decision recorded in `public-demo/README.md`.

- [ ] **Step 5.4: Deploy only after P2-WIN-02, provenance, and payload gates are satisfied**

If P2-WIN-02 is complete or explicitly waived, TypeDuck owner-approved provenance is recorded, and the asset manifest is below the verified Cloudflare limits, run:

```powershell
npx.cmd wrangler deploy --config third_party\typeduck-web\public-demo\wrangler.jsonc
```

Expected:

- Deployed URL is recorded in `cloudflare-smoke.md`.
- Deployed URL is also written as plain text to `third_party\typeduck-web\e2e\results\m31-public-demo\deployed-url.txt`.
- No Cloudflare token, account id, or secret is committed.
- If any gate is not satisfied, do not deploy. Record the blocker in `cloudflare-smoke.md` and keep M31 open or partial.

- [ ] **Step 5.5: Run deployed smoke**

Run the focused Playwright smoke against the deployed URL:

```powershell
$DeployedUrl = Get-Content third_party\typeduck-web\e2e\results\m31-public-demo\deployed-url.txt -Raw
$env:TYPEDUCK_APP_URL = ($DeployedUrl.TrimEnd("/") + "/web/")
npm.cmd --prefix third_party\typeduck-web\e2e run test:e2e -- --grep "M31 public demo" --workers=1
```

Expected:

- App boots.
- Typing `ngohaig` or the current canonical Jyutping smoke input produces candidates.
- Output-standard selector works.
- AI remains off by default.
- Network capture shows no remote AI/model/telemetry calls.
- The public page or footer links to the provenance note, or the evidence records why the in-repo provenance file is sufficient.

## Task 6 - AI Posture And Privacy Gate

**Files:**

- Modify: `third_party/typeduck-web/e2e/yune-typeduck.spec.ts`
- Create evidence: `third_party/typeduck-web/e2e/results/m31-public-demo/ai-posture.md`

- [ ] **Step 6.1: Add AI-off identity smoke**

Add a browser test that captures candidate state with AI disabled and compares it to the same input after toggling unrelated display controls. The AI option must remain disabled by default on fresh load. For the first public demo, the AI control should be hidden or developer-only unless a separate M32 decision explicitly exposes it.

- [ ] **Step 6.2: Add network posture check**

In Playwright, collect requests during startup and a short typing session. Assert no requests go to:

- AI/model provider domains
- telemetry or analytics endpoints
- secret-bearing URLs

The app may fetch its own WASM, JS, CSS, schema, and font/static assets.

- [ ] **Step 6.3: Record posture result**

`ai-posture.md` must state:

- default AI setting
- whether local AI is hidden, developer-only, or manually enable-able in this demo
- whether remote calls exist
- whether any secrets or telemetry are configured
- whether AI-off output stayed byte-identical through the tested path

## Task 7 - Closeout

**Files:**

- Modify: `docs/roadmap.md`
- Modify: `docs/requirements.md`
- Move when complete: `docs/plans/m31-plan-typeduck-web-public-demo-readiness.md` to `docs/plans/archive/`

- [ ] **Step 7.1: Run full gates**

Run:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p yune-core --test cantonese_parity
cargo test -p yune-rime-api --test typeduck_web
cargo test --workspace
npm.cmd --prefix packages\yune-typeduck-runtime test
npm.cmd --prefix packages\yune-typeduck-runtime run build
npm.cmd --prefix third_party\typeduck-web\source run build
npm.cmd --prefix third_party\typeduck-web\e2e run test:e2e -- --grep "M31" --workers=1
git diff --check
```

Expected:

- All gates pass.
- If P2-WIN-02 blocked public deployment, local preview evidence may close only the engine/UI/build portion; the roadmap must keep deployment active.

- [ ] **Step 7.2: Update docs and archive**

Update:

- `docs/requirements.md` with `M31-PUBLIC-REQ-*` rows matching the acceptance gates.
- `docs/roadmap.md` with either complete status and deployed URL, or partial/blocker status if public deployment is blocked.
- `third_party/typeduck-web/public-demo/PROVENANCE.md` and `asset-manifest.md` with the final public-readiness state.
- Archive this plan only when all acceptance gates close or the remaining deployment blocker is recorded as a new active plan.

- [ ] **Step 7.3: Commit scoped changes**

Stage only M31 files:

```powershell
git status --short
git add -- docs\roadmap.md docs\requirements.md docs\plans\m31-plan-typeduck-web-public-demo-readiness.md third_party\typeduck-web packages crates
git commit -m "Prepare TypeDuck-Web public demo"
git push origin main
```

Only include `third_party\typeduck-web`, `packages`, and `crates` paths that actually changed in M31.

## Review Questions

- Which OpenCC standards are truly backed by checked-in Yune data today, and which should remain absent until data/config support lands?
- Is OpenCC breadth truly required for the first public demo, or should M31 keep only the existing proven simplification path and split broader output standards later?
- Should the public URL serve from root or `/web/`, and how should redirects behave?
- Is Workers Static Assets still the preferred Cloudflare target at execution time, or should the worker choose Pages based on current docs and project needs?
- Can public deployment proceed before P2-WIN-02, or should the raw TypeDuck comment-boundary risk block sharing even though the web UI sanitizes display output?
- What exact public copy should explain that the demo exercises Yune through a TypeDuck-Web-derived harness?
- Which schema/dictionary assets are actually needed by the first public bundle, and which should be excluded for payload size?
