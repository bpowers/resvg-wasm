#!/usr/bin/env bash
set -euo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR"

# Clean previous packages
rm -rf lib lib.browser pkg pkg-node core

# Get the package name
PKG_NAME=resvg_wasm

cargo build --lib --release --target wasm32-unknown-unknown

echo "running wasm-bindgen"
wasm-bindgen ./target/wasm32-unknown-unknown/release/${PKG_NAME}.wasm --out-dir pkg --typescript --target bundler
wasm-bindgen ./target/wasm32-unknown-unknown/release/${PKG_NAME}.wasm --out-dir pkg-node --typescript --target nodejs

WASM_OPT_FLAGS="--enable-simd --enable-bulk-memory --enable-mutable-globals --enable-sign-ext --enable-nontrapping-float-to-int --enable-multivalue --enable-reference-types --enable-tail-call --enable-extended-const"

if [ "1" != "${DISABLE_WASM_OPT-0}" ]; then
  echo "running wasm-opt"
  wasm-opt pkg/${PKG_NAME}_bg.wasm -o pkg/${PKG_NAME}_bg.wasm-opt.wasm -O3 $WASM_OPT_FLAGS
  wasm-opt pkg-node/${PKG_NAME}_bg.wasm -o pkg-node/${PKG_NAME}_bg.wasm-opt.wasm -O3 $WASM_OPT_FLAGS
  mv pkg/${PKG_NAME}_bg.{wasm-opt.,}wasm
  mv pkg-node/${PKG_NAME}_bg.{wasm-opt.,}wasm
else
  echo "skipping wasm-opt"
fi

mv pkg core

pnpm tsc
pnpm tsc -p tsconfig.browser.json

rm -rf lib/core lib/pkg-node
rm -rf lib.browser/core lib.browser/pkg-node

cp -r pkg-node lib/core
cp -r core lib.browser/

rm -r pkg-node

mv lib/index{_main,}.js
mv lib/index{_main,}.js.map
mv lib/index{_main,}.d.ts
rm lib.browser/index_main*

pnpm format
