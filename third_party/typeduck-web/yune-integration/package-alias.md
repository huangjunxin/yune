# Local Package Alias for @yune-ime/typeduck-runtime

TypeDuck-Web integration imports `@yune-ime/typeduck-runtime` from the repository-owned package without publishing to npm.

## Alias Methods

### Method 1: Package.json Alias (Preferred)

Add alias to TypeDuck-Web's `package.json`:

```json
{
  "dependencies": {
    "@yune-ime/typeduck-runtime": "file:../../../packages/yune-typeduck-runtime"
  }
}
```

Then install:

```bash
cd third_party/typeduck-web/source
bun install
```

Alias resolves to local package directory, making `import "@yune-ime/typeduck-runtime"` work.

### Method 2: Vite Resolve Alias

If upstream uses Vite (TypeDuck-Web does), add resolve alias to `vite.config.ts`:

```typescript
import { defineConfig } from "vite";
import path from "path";

export default defineConfig({
  resolve: {
    alias: {
      "@yune-ime/typeduck-runtime": path.resolve(
        __dirname,
        "../../../packages/yune-typeduck-runtime/src/index.ts"
      ),
    },
  },
});
```

Alias resolves imports to TypeScript source directly during dev server.

### Method 3: Relative Import (Fallback)

If alias mechanisms unavailable, use relative import in adapter:

```typescript
// Instead of:
import { TypeDuckRuntime } from "@yune-ime/typeduck-runtime";

// Use:
import { TypeDuckRuntime } from "../../../packages/yune-typeduck-runtime/src/index.js";
```

Less maintainable, but works without package alias configuration.

## Verify Alias

After alias configuration, verify import resolves:

```bash
cd third_party/typeduck-web/source

# Check package installed (Method 1)
ls node_modules/@yune-ime/typeduck-runtime

# Or test import (Method 2/3)
bun run worker
```

Worker build should compile adapter imports without errors.

## Build Local Package

Before alias, ensure local package builds:

```bash
# From repository root
npm --prefix packages/yune-typeduck-runtime run build
```

Output in `packages/yune-typeduck-runtime/dist/` used by alias.

## Notes

- Alias only for local development/testing, not for distribution
- Do not publish `@yune-ime/typeduck-runtime` to npm during Phase 10
- Alias path assumes TypeDuck-Web checkout at `third_party/typeduck-web/source`
- Adjust path if checkout location differs

---

**Phase**: 10-typeduck-web-app-integration-and-e2e
**Plan**: 10-02 (Yune seam patch/configuration layer)