export interface TypeDuckFilesystem {
  mkdirTree?(path: string, mode?: number): void;
  mkdir?(path: string, mode?: number): void;
  writeFile(path: string, data: string | Uint8Array, opts?: { flags?: string }): void;
  readFile(path: string, opts?: { encoding?: "utf8" | "binary" }): string | Uint8Array;
  analyzePath(path: string, dontResolveLastLink?: boolean): { exists: boolean; error?: unknown };
}

export interface TypeDuckFilesystemAssets {
  defaultYaml: string | Uint8Array;
  schemaYaml: string | Uint8Array;
  dictionaryYaml: string | Uint8Array;
}

export interface PrepareTypeDuckFilesystemOptions {
  sharedDataDir: string;
  userDataDir: string;
  schemaId: string;
  dictionaryId: string;
  assets: TypeDuckFilesystemAssets;
}

export class TypeDuckFilesystemError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "TypeDuckFilesystemError";
  }
}

export function isTypeDuckLogicalId(id: string): boolean {
  return /^[A-Za-z0-9_-]+$/.test(id);
}

export function joinTypeDuckVirtualPath(base: string, ...parts: string[]): string {
  const allParts = [base, ...parts]
    .map((part) => part.split("/").filter((segment) => segment.length > 0))
    .flat();
  const joined = allParts.join("/");
  return base.startsWith("/") ? `/${joined}` : joined;
}

export function typeDuckBuildDir(userDataDir: string): string {
  return joinTypeDuckVirtualPath(userDataDir, "build");
}

export function requiredTypeDuckAssetPaths(options: PrepareTypeDuckFilesystemOptions): string[] {
  assertTypeDuckLogicalId(options.schemaId, "schemaId");
  assertTypeDuckLogicalId(options.dictionaryId, "dictionaryId");
  const buildDir = typeDuckBuildDir(options.userDataDir);
  return [
    joinTypeDuckVirtualPath(options.sharedDataDir, "default.yaml"),
    joinTypeDuckVirtualPath(options.sharedDataDir, `${options.schemaId}.schema.yaml`),
    joinTypeDuckVirtualPath(options.sharedDataDir, `${options.dictionaryId}.dict.yaml`),
    joinTypeDuckVirtualPath(buildDir, "default.yaml"),
    joinTypeDuckVirtualPath(buildDir, `${options.schemaId}.schema.yaml`),
  ];
}

export function prepareTypeDuckFilesystem(
  fs: TypeDuckFilesystem,
  options: PrepareTypeDuckFilesystemOptions,
): void {
  assertTypeDuckLogicalId(options.schemaId, "schemaId");
  assertTypeDuckLogicalId(options.dictionaryId, "dictionaryId");

  const buildDir = typeDuckBuildDir(options.userDataDir);
  ensureTypeDuckDirectory(fs, options.sharedDataDir);
  ensureTypeDuckDirectory(fs, options.userDataDir);
  ensureTypeDuckDirectory(fs, buildDir);

  fs.writeFile(joinTypeDuckVirtualPath(options.sharedDataDir, "default.yaml"), options.assets.defaultYaml, {
    flags: "w",
  });
  fs.writeFile(
    joinTypeDuckVirtualPath(options.sharedDataDir, `${options.schemaId}.schema.yaml`),
    options.assets.schemaYaml,
    { flags: "w" },
  );
  fs.writeFile(
    joinTypeDuckVirtualPath(options.sharedDataDir, `${options.dictionaryId}.dict.yaml`),
    options.assets.dictionaryYaml,
    { flags: "w" },
  );
  fs.writeFile(joinTypeDuckVirtualPath(buildDir, "default.yaml"), options.assets.defaultYaml, {
    flags: "w",
  });
  fs.writeFile(joinTypeDuckVirtualPath(buildDir, `${options.schemaId}.schema.yaml`), options.assets.schemaYaml, {
    flags: "w",
  });

  assertTypeDuckAssetsReady(fs, options);
}

export function assertTypeDuckAssetsReady(
  fs: TypeDuckFilesystem,
  options: PrepareTypeDuckFilesystemOptions,
): void {
  const missing = requiredTypeDuckAssetPaths(options).filter((path) => !fs.analyzePath(path).exists);
  if (missing.length > 0) {
    throw new TypeDuckFilesystemError(`Missing TypeDuck filesystem assets: ${missing.join(", ")}`);
  }
}

function ensureTypeDuckDirectory(fs: TypeDuckFilesystem, path: string): void {
  if (fs.analyzePath(path).exists) {
    return;
  }
  if (fs.mkdirTree !== undefined) {
    fs.mkdirTree(path);
    return;
  }
  if (fs.mkdir === undefined) {
    throw new TypeDuckFilesystemError("Emscripten filesystem directory creation is unavailable");
  }
  const segments = path.split("/").filter((segment) => segment.length > 0);
  let current = path.startsWith("/") ? "" : ".";
  for (const segment of segments) {
    current = current === "." ? segment : `${current}/${segment}`;
    const directory = path.startsWith("/") ? `/${current}` : current;
    if (!fs.analyzePath(directory).exists) {
      fs.mkdir(directory);
    }
  }
}

function assertTypeDuckLogicalId(id: string, label: string): void {
  if (!isTypeDuckLogicalId(id)) {
    throw new TypeDuckFilesystemError(`Invalid TypeDuck logical id for ${label}: ${id}`);
  }
}
