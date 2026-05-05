import { describe, expect, it } from "vitest";

import {
  assertTypeDuckAssetsReady,
  customizeAndSync,
  deployAndSync,
  isTypeDuckLogicalId,
  mountTypeDuckPersistence,
  prepareTypeDuckFilesystem,
  requiredTypeDuckAssetPaths,
  syncAfterUserDataChange,
  syncFromPersistenceBeforeInit,
  syncToPersistenceAfterMutation,
  TypeDuckFilesystemError,
} from "../src/filesystem.js";
import { TypeDuckRuntime } from "../src/typeduck.js";
import { FakeTypeDuckFilesystem } from "./fake-filesystem.js";
import { FakeTypeDuckModule } from "./fake-module.js";

const defaultYaml = "config_version: typeduck-web\nschema_list:\n  - schema: typeduck_luna\n";
const schemaYaml = "schema:\n  schema_id: typeduck_luna\ntranslator:\n  dictionary: typeduck\n";
const dictionaryYaml = "---\nname: typeduck\nversion: '1'\n...\nba\t吧\t9\n";
const defaultInitPtr = 1;

function filesystemOptions(overrides: Partial<Parameters<typeof prepareTypeDuckFilesystem>[1]> = {}) {
  return {
    sharedDataDir: "/yune/shared",
    userDataDir: "/yune/user",
    schemaId: "typeduck_luna",
    dictionaryId: "typeduck",
    assets: {
      defaultYaml,
      schemaYaml,
      dictionaryYaml,
    },
    ...overrides,
  };
}

function initializedRuntime(fake = new FakeTypeDuckModule()): TypeDuckRuntime {
  return TypeDuckRuntime.init(fake, {
    sharedDataDir: "/yune/shared",
    userDataDir: "/yune/user",
    schemaId: "typeduck_luna",
  });
}

describe("TypeDuck browser filesystem preparation", () => {
  it("creates shared, user, and build directories in an Emscripten filesystem", () => {
    const fs = new FakeTypeDuckFilesystem();

    prepareTypeDuckFilesystem(fs, filesystemOptions());

    expect(fs.directories()).toEqual(["/", "/yune", "/yune/shared", "/yune/user", "/yune/user/build"]);
    expect(fs.calls("mkdirTree").map(([path]) => path)).toEqual([
      "/yune/shared",
      "/yune/user",
      "/yune/user/build",
    ]);
  });

  it("writes explicit shared and build assets at the native-required virtual paths", () => {
    const fs = new FakeTypeDuckFilesystem();

    prepareTypeDuckFilesystem(fs, filesystemOptions());

    expect(fs.readText("/yune/shared/default.yaml")).toBe(defaultYaml);
    expect(fs.readText("/yune/shared/typeduck_luna.schema.yaml")).toBe(schemaYaml);
    expect(fs.readText("/yune/shared/typeduck.dict.yaml")).toBe(dictionaryYaml);
    expect(fs.readText("/yune/user/build/default.yaml")).toBe(defaultYaml);
    expect(fs.readText("/yune/user/build/typeduck_luna.schema.yaml")).toBe(schemaYaml);
    expect(requiredTypeDuckAssetPaths(filesystemOptions())).toEqual([
      "/yune/shared/default.yaml",
      "/yune/shared/typeduck_luna.schema.yaml",
      "/yune/shared/typeduck.dict.yaml",
      "/yune/user/build/default.yaml",
      "/yune/user/build/typeduck_luna.schema.yaml",
    ]);
  });

  it("reports missing required assets without creating fallback files", () => {
    const fs = new FakeTypeDuckFilesystem();
    fs.mkdirTree("/yune/shared");
    fs.mkdirTree("/yune/user/build");
    fs.writeFile("/yune/shared/typeduck_luna.schema.yaml", schemaYaml);

    expect(() => assertTypeDuckAssetsReady(fs, filesystemOptions())).toThrow(TypeDuckFilesystemError);
    expect(() => assertTypeDuckAssetsReady(fs, filesystemOptions())).toThrow(
      "Missing TypeDuck filesystem assets: /yune/shared/default.yaml, /yune/shared/typeduck.dict.yaml, /yune/user/build/default.yaml, /yune/user/build/typeduck_luna.schema.yaml",
    );
    expect(fs.exists("/yune/shared/default.yaml")).toBe(false);
    expect(fs.exists("/yune/shared/typeduck.dict.yaml")).toBe(false);
    expect(fs.exists("/yune/user/build/default.yaml")).toBe(false);
    expect(fs.exists("/yune/user/build/typeduck_luna.schema.yaml")).toBe(false);
  });

  it("rejects invalid schema and dictionary ids before joining write paths", () => {
    const invalidIds = ["", "../typeduck_luna", "typeduck/luna", "typeduck\\luna"];

    for (const invalidId of invalidIds) {
      expect(isTypeDuckLogicalId(invalidId)).toBe(false);

      const invalidSchemaFs = new FakeTypeDuckFilesystem();
      expect(() =>
        prepareTypeDuckFilesystem(invalidSchemaFs, filesystemOptions({ schemaId: invalidId })),
      ).toThrow(TypeDuckFilesystemError);
      expect(invalidSchemaFs.calls("writeFile")).toEqual([]);

      const invalidDictionaryFs = new FakeTypeDuckFilesystem();
      expect(() =>
        prepareTypeDuckFilesystem(invalidDictionaryFs, filesystemOptions({ dictionaryId: invalidId })),
      ).toThrow(TypeDuckFilesystemError);
      expect(invalidDictionaryFs.calls("writeFile")).toEqual([]);
    }

    expect(isTypeDuckLogicalId("typeduck_luna-1")).toBe(true);
  });

  it("mounts a caller-provided persistence backend at the requested mountpoint", () => {
    const fs = new FakeTypeDuckFilesystem();
    const type = { name: "IDBFS" };
    const opts = { root: "typeduck" };

    mountTypeDuckPersistence(fs, type, opts, "/yune");

    expect(fs.directories()).toEqual(["/", "/yune"]);
    expect(fs.calls("mount")).toEqual([[type, opts, "/yune"]]);
  });

  it("syncs from persistence before init using the populate direction", async () => {
    const fs = new FakeTypeDuckFilesystem();

    await syncFromPersistenceBeforeInit(fs);

    expect(fs.calls("syncfs")).toEqual([[true]]);
  });

  it("syncs to persistence after mutation and explicit user data changes", async () => {
    const fs = new FakeTypeDuckFilesystem();

    await syncToPersistenceAfterMutation(fs);
    await syncAfterUserDataChange(fs);

    expect(fs.calls("syncfs")).toEqual([[false], [false]]);
  });

  it("surfaces sync callback errors with deterministic direction details", async () => {
    const fs = new FakeTypeDuckFilesystem();
    fs.syncError = new Error("fake sync failure");

    await expect(syncFromPersistenceBeforeInit(fs)).rejects.toMatchObject({
      name: "TypeDuckFilesystemError",
      message: "TypeDuck filesystem sync failed",
      direction: "fromPersistence",
    });

    await expect(syncToPersistenceAfterMutation(fs)).rejects.toMatchObject({
      name: "TypeDuckFilesystemError",
      message: "TypeDuck filesystem sync failed",
      direction: "toPersistence",
    });

    expect(fs.calls("syncfs")).toEqual([[true], [false]]);
  });

  it("deploys through the runtime before syncing to persistence and returns the deploy boolean", async () => {
    const module = new FakeTypeDuckModule();
    module.deployResult = 1;
    const runtime = initializedRuntime(module);
    const fs = new FakeTypeDuckFilesystem();

    await expect(deployAndSync(runtime, fs)).resolves.toBe(true);

    expect(module.calls("yune_typeduck_deploy")).toEqual([[defaultInitPtr]]);
    expect(fs.calls("syncfs")).toEqual([[false]]);
  });

  it("throws sync failures after deploy while preserving the runtime mutation call", async () => {
    const module = new FakeTypeDuckModule();
    module.deployResult = 0;
    const runtime = initializedRuntime(module);
    const fs = new FakeTypeDuckFilesystem();
    fs.syncError = "persist failed";

    await expect(deployAndSync(runtime, fs)).rejects.toMatchObject({
      name: "TypeDuckFilesystemError",
      message: "TypeDuck filesystem sync failed",
      direction: "toPersistence",
    });

    expect(module.calls("yune_typeduck_deploy")).toEqual([[defaultInitPtr]]);
    expect(fs.calls("syncfs")).toEqual([[false]]);
  });

  it("customizes through the runtime before syncing and preserves adapter arguments", async () => {
    const module = new FakeTypeDuckModule();
    module.customizeResult = 0;
    const runtime = initializedRuntime(module);
    const fs = new FakeTypeDuckFilesystem();

    await expect(
      customizeAndSync(runtime, fs, "typeduck_luna.schema", "schema/name", "TypeDuck Luna Web"),
    ).resolves.toBe(false);

    expect(module.calls("yune_typeduck_customize")).toEqual([
      [defaultInitPtr, "typeduck_luna.schema", "schema/name", "TypeDuck Luna Web"],
    ]);
    expect(fs.calls("syncfs")).toEqual([[false]]);
  });
});
