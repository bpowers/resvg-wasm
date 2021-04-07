// Copyright 2021 Bobby Powers. All rights reserved.
// Use of this source code is governed by the Mozilla Public
// License, Version 2.0, that can be found in the LICENSE file.

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

export async function render(svgXml: string): Promise<Uint8Array | undefined> {
  const wasm = await getWasmModule();
  return wasm.render(svgXml);
}
