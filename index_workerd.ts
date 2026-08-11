// Copyright 2021 Bobby Powers. All rights reserved.
// Use of this source code is governed by the Mozilla Public
// License, Version 2.0, that can be found in the LICENSE file.

import type { Context } from "./iresvg";
export type { Context };

// In workerd (Cloudflare Workers), importing a .wasm file yields a compiled
// WebAssembly.Module -- runtime compilation from bytes is not allowed, so the
// dynamic-import approach used by the browser/node entry points doesn't work.
import wasmModule from "./core/resvg_wasm_bg.wasm";
import * as bindings from "./core/resvg_wasm";

// This file is type-checked against the bundler-target bindings in core/, but
// lib.workerd ships wasm-bindgen's `--target web` output, which additionally
// default-exports an init function that accepts a precompiled module.
interface WebTargetBindings {
  default(options: { module_or_path: WebAssembly.Module }): Promise<unknown>;
  newContext(): Context;
}
const wasm = bindings as unknown as WebTargetBindings;

let initialized: Promise<unknown> | undefined;

export async function newContext(): Promise<Context> {
  if (!initialized) {
    initialized = wasm.default({ module_or_path: wasmModule });
  }
  await initialized;
  return wasm.newContext();
}
