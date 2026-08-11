#!/usr/bin/env bash
set -euo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR"

# Clean previous packages
rm -rf lib lib.browser lib.workerd pkg pkg-node pkg-workerd core

# Get the package name
PKG_NAME=resvg_wasm

cargo build --lib --release --target wasm32-unknown-unknown

echo "running wasm-bindgen"
wasm-bindgen ./target/wasm32-unknown-unknown/release/${PKG_NAME}.wasm --out-dir pkg --typescript --target bundler
wasm-bindgen ./target/wasm32-unknown-unknown/release/${PKG_NAME}.wasm --out-dir pkg-node --typescript --target nodejs
# workerd (Cloudflare Workers) can't compile WASM from bytes at runtime; the
# web target's init() accepts the precompiled WebAssembly.Module that a
# wrangler-bundled `import ... from "*.wasm"` provides.
wasm-bindgen ./target/wasm32-unknown-unknown/release/${PKG_NAME}.wasm --out-dir pkg-workerd --typescript --target web

WASM_OPT_FLAGS="--enable-simd --enable-bulk-memory --enable-mutable-globals --enable-sign-ext --enable-nontrapping-float-to-int --enable-multivalue --enable-reference-types --enable-tail-call --enable-extended-const"

if [ "1" != "${DISABLE_WASM_OPT-0}" ]; then
  echo "running wasm-opt"
  wasm-opt pkg/${PKG_NAME}_bg.wasm -o pkg/${PKG_NAME}_bg.wasm-opt.wasm -O3 $WASM_OPT_FLAGS
  wasm-opt pkg-node/${PKG_NAME}_bg.wasm -o pkg-node/${PKG_NAME}_bg.wasm-opt.wasm -O3 $WASM_OPT_FLAGS
  wasm-opt pkg-workerd/${PKG_NAME}_bg.wasm -o pkg-workerd/${PKG_NAME}_bg.wasm-opt.wasm -O3 $WASM_OPT_FLAGS
  mv pkg/${PKG_NAME}_bg.{wasm-opt.,}wasm
  mv pkg-node/${PKG_NAME}_bg.{wasm-opt.,}wasm
  mv pkg-workerd/${PKG_NAME}_bg.{wasm-opt.,}wasm
else
  echo "skipping wasm-opt"
fi

mv pkg core

pnpm tsc
pnpm tsc -p tsconfig.browser.json
pnpm tsc -p tsconfig.workerd.json

rm -rf lib/core lib/pkg-node
rm -rf lib.browser/core lib.browser/pkg-node
rm -rf lib.workerd/core lib.workerd/pkg-node

cp -r pkg-node lib/core
cp -r core lib.browser/
cp -r pkg-workerd lib.workerd/core

rm -r pkg-node pkg-workerd

mv lib/index{_main,}.js
mv lib/index{_main,}.js.map
mv lib/index{_main,}.d.ts
rm lib/index_workerd*
rm lib.browser/index_main* lib.browser/index_workerd*
mv lib.workerd/index{_workerd,}.js
mv lib.workerd/index{_workerd,}.js.map
mv lib.workerd/index{_workerd,}.d.ts
rm lib.workerd/index_main*

pnpm format
