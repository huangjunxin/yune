# M26 TypeDuck-Web Patch Checks

> **Status:** Complete - **Milestone:** M26 (performance hardening) - **Updated:** 2026-06-22 - **Type:** evidence

TypeDuck-Web source files changed for browser diagnostics and startup attribution, so `third_party/typeduck-web/patches/yune-typeduck-runtime.patch` was regenerated from the patched source checkout.

Regeneration command:

```powershell
git -C third_party/typeduck-web/source diff HEAD --submodule=diff --binary --output=..\patches\yune-typeduck-runtime.patch
```

Reverse check from the patched checkout:

```powershell
Push-Location third_party/typeduck-web/source
git apply --reverse --check ..\patches\yune-typeduck-runtime.patch
Pop-Location
```

Result: passed.

Forward check from a temporary clean upstream worktree reset to `third_party/typeduck-web/typeduck-web.lock.json` revision `03f9afd2cf6ca75653197f2193f24d1cd0adbd83`:

```powershell
$target = (Resolve-Path .).Path + '\target\m26-typeduck-web-forward-check'
git -C third_party/typeduck-web/source worktree add --detach $target 03f9afd2cf6ca75653197f2193f24d1cd0adbd83
git -C $target submodule update --init --recursive
git -C $target apply --check ..\..\third_party\typeduck-web\patches\yune-typeduck-runtime.patch
git -C third_party/typeduck-web/source worktree remove --force $target
```

Result: passed.
