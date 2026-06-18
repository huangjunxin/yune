import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";

import type { TypeDuckFilesystem } from "../../../packages/yune-typeduck-runtime/src/filesystem.js";

import { FakeTypeDuckFilesystem } from "../../../packages/yune-typeduck-runtime/test/fake-filesystem.js";
import { FakeTypeDuckModule } from "../../../packages/yune-typeduck-runtime/test/fake-module.js";

vi.mock("@yune-ime/typeduck-runtime", async () => {
  return await import("../../../packages/yune-typeduck-runtime/src/index.ts");
});

let cleanupYuneRuntime: typeof import("./adapter.js").cleanupYuneRuntime;
let initYuneRuntime: typeof import("./adapter.js").initYuneRuntime;

const assets = {
  defaultYaml: "config_version: typeduck-web\nschema_list:\n  - schema: luna_pinyin\n",
  schemaYaml: "schema:\n  schema_id: luna_pinyin\ntranslator:\n  dictionary: luna_pinyin\n",
  dictionaryYaml: "---\nname: luna_pinyin\n...\nni\t你\t1\n",
};

const initOptions = {
  sharedDataDir: "/usr/share/rime-data",
  userDataDir: "/rime",
  schemaId: "luna_pinyin",
};

function recordingFs(fs: FakeTypeDuckFilesystem, order: string[]): TypeDuckFilesystem {
  return new Proxy(fs, {
    get(target, property, receiver) {
      if (property === "writeFile") {
        return (path: string, data: string | Uint8Array, opts?: { flags?: string }) => {
          order.push(`write:${path}`);
          target.writeFile(path, data, opts);
        };
      }
      if (property === "syncfs") {
        return (populate: boolean, callback: (error?: unknown) => void) => {
          order.push(`syncfs(${populate})`);
          target.syncfs(populate, callback);
        };
      }
      const value = Reflect.get(target, property, receiver);
      return typeof value === "function" ? value.bind(target) : value;
    },
  });
}

describe("initYuneRuntime browser filesystem ordering", () => {
  beforeAll(async () => {
    const adapter = await import("./adapter.js");
    cleanupYuneRuntime = adapter.cleanupYuneRuntime;
    initYuneRuntime = adapter.initYuneRuntime;
  });

  afterEach(() => {
    cleanupYuneRuntime();
  });

  it("loads persisted state before preloading schema assets and flushes after init", async () => {
    const order: string[] = [];
    const fs = recordingFs(new FakeTypeDuckFilesystem(), order);
    const module = new FakeTypeDuckModule();

    await initYuneRuntime(module, fs, initOptions, assets, "luna_pinyin", [
      { path: "opencc/hk2s.json", content: "{}" },
    ]);

    expect(order).toEqual([
      "syncfs(true)",
      "write:/usr/share/rime-data/default.yaml",
      "write:/usr/share/rime-data/luna_pinyin.schema.yaml",
      "write:/usr/share/rime-data/luna_pinyin.dict.yaml",
      "write:/rime/build/default.yaml",
      "write:/rime/build/luna_pinyin.schema.yaml",
      "write:/usr/share/rime-data/opencc/hk2s.json",
      "syncfs(false)",
    ]);
    expect(module.calls("yune_typeduck_init")).toEqual([
      ["/usr/share/rime-data", "/rime", "luna_pinyin"],
    ]);
  });

  it("fails visibly before asset writes or runtime init when before-init sync fails", async () => {
    const fs = new FakeTypeDuckFilesystem();
    fs.syncError = "IDBFS unavailable";
    const module = new FakeTypeDuckModule();

    await expect(initYuneRuntime(module, fs, initOptions, assets, "luna_pinyin")).rejects.toMatchObject({
      name: "TypeDuckFilesystemError",
      direction: "fromPersistence",
    });

    expect(fs.calls("writeFile")).toEqual([]);
    expect(module.calls("yune_typeduck_init")).toEqual([]);
  });
});
