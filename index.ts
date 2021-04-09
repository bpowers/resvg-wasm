// Copyright 2021 Bobby Powers. All rights reserved.
// Use of this source code is governed by the Mozilla Public
// License, Version 2.0, that can be found in the LICENSE file.

import type { Context } from "./iresvg";
export type { Context };

let cachedWasmModule: typeof import("./core/resvg_wasm") | undefined;
function getWasmModule(): Promise<typeof import("./core/resvg_wasm")> {
  return new Promise((resolve, reject) => {
    if (cachedWasmModule) {
      resolve(cachedWasmModule);
      return;
    }

    import("./core/resvg_wasm")
      .then((module) => {
        cachedWasmModule = module;
        resolve(module);
      })
      .catch(reject);
  });
}

export async function newContext(): Promise<Context> {
  const wasm = await getWasmModule();
  return wasm.newContext();
}
