# M27 TypeDuck-Web Patch Checks

> **Status:** Complete - **Milestone:** M27 (TypeDuck-Web startup/runtime init) - **Updated:** 2026-06-22 - **Type:** evidence

TypeDuck-Web source files changed for browser startup diagnostics, asset cache versioning, and AI control loading behavior, so `third_party/typeduck-web/patches/yune-typeduck-runtime.patch` was regenerated from the patched source checkout.

## Regeneration

```powershell
git -C third_party\typeduck-web\source diff HEAD --submodule=diff --binary --output=..\patches\yune-typeduck-runtime.patch
```

Result: passed.

## Reverse Check

```powershell
git -C third_party\typeduck-web\source apply --reverse --check ..\patches\yune-typeduck-runtime.patch
```

Result: passed.

## Forward Check

```powershell
$target = (Resolve-Path .).Path + '\target\m27-typeduck-web-forward-check'
git -C third_party\typeduck-web\source worktree add --detach $target 03f9afd2cf6ca75653197f2193f24d1cd0adbd83
git -C $target submodule update --init --recursive
git -C $target apply --check ..\..\third_party\typeduck-web\patches\yune-typeduck-runtime.patch
git -C third_party\typeduck-web\source worktree remove --force $target
```

Result: passed.
