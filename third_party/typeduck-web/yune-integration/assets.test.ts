import { afterEach, describe, expect, it, vi } from "vitest";

import { loadAssetContent, validateExplicitAssets } from "./assets.js";

describe("TypeDuck-Web explicit asset loading", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("loads app-owned YAML from browser URLs", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async () => new Response("schema:\n  schema_id: luna_pinyin\n", { status: 200 })),
    );

    await expect(loadAssetContent({ type: "url", url: "schema/luna_pinyin.schema.yaml" })).resolves.toBe(
      "schema:\n  schema_id: luna_pinyin\n",
    );
  });

  it("surfaces missing URL assets as clear init blockers", async () => {
    vi.stubGlobal("fetch", vi.fn(async () => new Response("missing", { status: 404 })));

    await expect(loadAssetContent({ type: "url", url: "schema/default.yaml" })).rejects.toThrow(
      "Asset URL loading failed: schema/default.yaml (404)",
    );
  });

  it("rejects empty explicit YAML instead of fabricating fallback assets", () => {
    expect(() =>
      validateExplicitAssets({
        defaultYaml: "",
        schemaYaml: "schema:\n  schema_id: luna_pinyin\n",
        dictionaryYaml: "---\nname: luna_pinyin\n...\n",
      }),
    ).toThrow("TypeDuck-Web asset default.yaml is empty");
  });
});
