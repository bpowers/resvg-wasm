// Copyright 2021 Bobby Powers. All rights reserved.
// Use of this source code is governed by the Mozilla Public
// License, Version 2.0, that can be found in the LICENSE file.

// workerd (and other edge runtimes) import .wasm files as compiled modules.
declare module "*.wasm" {
  const wasmModule: WebAssembly.Module;
  export default wasmModule;
}
