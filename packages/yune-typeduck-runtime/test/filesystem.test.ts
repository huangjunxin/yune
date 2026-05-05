import { describe, expect, it } from "vitest";

import {
  assertTypeDuckAssetsReady,
  isTypeDuckLogicalId,
  prepareTypeDuckFilesystem,
  requiredTypeDuckAssetPaths,
  TypeDuckFilesystemError,
} from "../src/filesystem.js";
import { FakeTypeDuckFilesystem } from "./fake-filesystem.js";

const defaultYaml = "config_version: typeduck-web\nschema_list:\n  - schema: typeduck_luna\n";
const schemaYaml = "schema:\n  schema_id: typeduck_luna\ntranslator:\n  dictionary: typeduck\n";
const dictionaryYaml = "---\nname: typeduck\nversion: '1'\n...\nba\t吧\t9\n";

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
});
