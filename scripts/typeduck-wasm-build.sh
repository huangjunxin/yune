#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
EXPORT_LIST="$REPO_ROOT/scripts/typeduck-exports.txt"
FALLBACK_TEST="cargo test -p yune-rime-api --test typeduck_web"

if [ ! -f "$EXPORT_LIST" ]; then
  echo "TypeDuck WASM build failed: missing export list at scripts/typeduck-exports.txt" >&2
  exit 1
fi

EXPORTS=$(grep -v '^[[:space:]]*$' "$EXPORT_LIST")
if [ -z "$EXPORTS" ]; then
  echo "TypeDuck WASM build failed: scripts/typeduck-exports.txt is empty" >&2
  exit 1
fi

run_native_fallback() {
  echo "Native fallback still available: $FALLBACK_TEST"
  (cd "$REPO_ROOT" && cargo test -p yune-rime-api --test typeduck_web)
}

block_missing_target() {
  echo "TypeDuck WASM build blocked: missing wasm32-unknown-emscripten Rust target."
  echo "Install with: rustup target add wasm32-unknown-emscripten"
  run_native_fallback
}

block_missing_emscripten() {
  TOOL_NAME=$1
  echo "TypeDuck WASM build blocked: missing Emscripten linker \`$TOOL_NAME\` on PATH."
  echo "Install/activate Emscripten SDK so \`emcc\` and \`emar\` are available, then rerun this command."
  run_native_fallback
}

find_native_library() {
  for candidate in \
    "$REPO_ROOT/target/debug/libyune_rime_api.dylib" \
    "$REPO_ROOT/target/debug/libyune_rime_api.so" \
    "$REPO_ROOT/target/debug/yune_rime_api.dll.lib" \
    "$REPO_ROOT/target/debug/yune_rime_api.dll"
  do
    if [ -f "$candidate" ]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done
  return 1
}

find_tool() {
  for tool in "$@"; do
    if command -v "$tool" >/dev/null 2>&1; then
      command -v "$tool"
      return 0
    fi

    if [ "${EMSDK+x}" = x ]; then
      for candidate in "$EMSDK/upstream/bin/$tool" "$EMSDK/upstream/bin/$tool.exe"; do
        if [ -x "$candidate" ]; then
          printf '%s\n' "$candidate"
          return 0
        fi
      done
    fi
  done
  return 1
}

verify_native_exports() {
  NATIVE_LIBRARY=$1
  SYMBOL_TOOL=$(find_tool nm llvm-nm) || {
    echo "TypeDuck WASM build failed: missing native symbol inspector \`nm\` or \`llvm-nm\` on PATH." >&2
    exit 1
  }

  NM_OUTPUT=$("$SYMBOL_TOOL" -g "$NATIVE_LIBRARY" 2>/dev/null || "$SYMBOL_TOOL" "$NATIVE_LIBRARY")
  for symbol in $EXPORTS; do
    if ! printf '%s\n' "$NM_OUTPUT" | grep -Eq "(^|[[:space:]])_?${symbol}($|[[:space:]])"; then
      echo "TypeDuck WASM build failed: native library is missing export $symbol" >&2
      exit 1
    fi
  done
  echo "TypeDuck native exports verified: $NATIVE_LIBRARY"
}

join_exported_functions() {
  PREFIXED=""
  for symbol in $EXPORTS; do
    if [ -z "$PREFIXED" ]; then
      PREFIXED="_$symbol"
    else
      PREFIXED="$PREFIXED,_$symbol"
    fi
  done
  printf '%s\n' "$PREFIXED"
}

configure_emscripten_linker() {
  if [ "${CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER+x}" = x ]; then
    return
  fi

  if [ "${EMSDK+x}" != x ]; then
    return
  fi

  for candidate in "$EMSDK/upstream/emscripten/emcc.exe" "$EMSDK/upstream/emscripten/emcc.bat"; do
    if [ -x "$candidate" ]; then
      if command -v cygpath >/dev/null 2>&1; then
        export CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER=$(cygpath -w "$candidate")
      else
        export CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER="$candidate"
      fi
      return
    fi
  done
}

find_first_artifact() {
  EXT=$1
  ARTIFACT_DIR="$REPO_ROOT/target/wasm32-unknown-emscripten/debug"
  if [ ! -d "$ARTIFACT_DIR" ]; then
    return 1
  fi
  find "$ARTIFACT_DIR" -type f -name "*$EXT" -print | sort | head -n 1
}

verify_wasm_exports() {
  WASM_ARTIFACT=$1
  JS_ARTIFACT=$2

  if WASM_NM=$(find_tool wasm-nm); then
    WASM_SYMBOLS=$("$WASM_NM" "$WASM_ARTIFACT")
    for symbol in $EXPORTS; do
      if ! printf '%s\n' "$WASM_SYMBOLS" | grep -Eq "(^|[[:space:]])_?${symbol}($|[[:space:]])"; then
        echo "TypeDuck WASM build failed: wasm-nm did not find export $symbol" >&2
        exit 1
      fi
    done
    return 0
  fi

  if WASM_OBJDUMP=$(find_tool wasm-objdump); then
    WASM_SYMBOLS=$("$WASM_OBJDUMP" -x "$WASM_ARTIFACT")
    for symbol in $EXPORTS; do
      if ! printf '%s\n' "$WASM_SYMBOLS" | grep -Eq "(^|[[:space:]])_?${symbol}($|[[:space:]])"; then
        echo "TypeDuck WASM build failed: wasm-objdump did not find export $symbol" >&2
        exit 1
      fi
    done
    return 0
  fi

  if LLVM_NM=$(find_tool llvm-nm); then
    WASM_SYMBOLS=$("$LLVM_NM" "$WASM_ARTIFACT")
    for symbol in $EXPORTS; do
      if ! printf '%s\n' "$WASM_SYMBOLS" | grep -Eq "(^|[[:space:]])_?${symbol}($|[[:space:]])"; then
        echo "TypeDuck WASM build failed: llvm-nm did not find export $symbol" >&2
        exit 1
      fi
    done
    return 0
  fi

  if [ ! -f "$JS_ARTIFACT" ]; then
    echo "TypeDuck WASM export verification limited: no wasm-nm, wasm-objdump, llvm-nm, or generated JS artifact available for symbol scan."
    echo "TypeDuck WASM artifact exists and native exports were verified before target build."
    return 0
  fi

  for symbol in $EXPORTS; do
    if ! grep -q "$symbol" "$JS_ARTIFACT"; then
      echo "TypeDuck WASM build failed: JS text scan fallback did not find export $symbol" >&2
      exit 1
    fi
  done
}

(cd "$REPO_ROOT" && cargo build -p yune-rime-api)
NATIVE_LIBRARY=$(find_native_library) || {
  echo "TypeDuck WASM build failed: could not locate yune-rime-api native dynamic library under target/debug" >&2
  exit 1
}
verify_native_exports "$NATIVE_LIBRARY"

if ! command -v rustup >/dev/null 2>&1; then
  echo "TypeDuck WASM build blocked: missing rustup on PATH."
  echo "Install Rustup and then run: rustup target add wasm32-unknown-emscripten"
  run_native_fallback
  exit 0
fi

if ! rustup target list --installed | grep -qx 'wasm32-unknown-emscripten'; then
  block_missing_target
  exit 0
fi

if ! command -v emcc >/dev/null 2>&1; then
  block_missing_emscripten "emcc"
  exit 0
fi

if ! command -v emar >/dev/null 2>&1; then
  block_missing_emscripten "emar"
  exit 0
fi

configure_emscripten_linker

EXPORTED_FUNCTIONS=$(join_exported_functions)
RUNTIME_METHODS="ccall,cwrap,UTF8ToString"
EXTRA_RUSTFLAGS="-C link-arg=-sEXPORTED_FUNCTIONS=$EXPORTED_FUNCTIONS -C link-arg=-sEXPORTED_RUNTIME_METHODS=$RUNTIME_METHODS"
if [ "${RUSTFLAGS+x}" = x ] && [ -n "$RUSTFLAGS" ]; then
  export RUSTFLAGS="$RUSTFLAGS $EXTRA_RUSTFLAGS"
else
  export RUSTFLAGS="$EXTRA_RUSTFLAGS"
fi

(cd "$REPO_ROOT" && cargo build -p yune-rime-api --target wasm32-unknown-emscripten)
WASM_ARTIFACT=$(find_first_artifact .wasm) || {
  echo "TypeDuck WASM build failed: no .wasm artifact found under target/wasm32-unknown-emscripten/debug" >&2
  exit 1
}
JS_ARTIFACT=$(find_first_artifact .js || true)
verify_wasm_exports "$WASM_ARTIFACT" "$JS_ARTIFACT"

echo "TypeDuck WASM build verified: $WASM_ARTIFACT"
if [ -n "$JS_ARTIFACT" ]; then
  echo "TypeDuck Emscripten JS glue verified: $JS_ARTIFACT"
fi
