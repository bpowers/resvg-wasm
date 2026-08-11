# resvg-wasm: resvg compiled to wasm

SVG-to-PNG rendering powered by [resvg](https://github.com/linebender/resvg), compiled to WebAssembly. Works in Node.js, browsers (via bundlers), and Cloudflare Workers, with zero runtime dependencies.

## Usage

```js
import { newContext } from "resvg-wasm";

const ctx = await newContext();
// optional: register fonts so <text> elements can be rendered
ctx.registerFontData(new Uint8Array(fontFileContents));
const png = ctx.render(svgString, /* scale */ 2); // Uint8Array of PNG data
```

### Node.js

Works out of the box (Node.js 22+).

### Browsers / bundlers

The `module` export condition serves an ESM build that loads the WASM binary with a dynamic `import()`. Bundlers with WebAssembly ESM integration (e.g. webpack 5 with `asyncWebAssembly`) handle this natively.

### Cloudflare Workers

The `workerd` export condition — which wrangler resolves automatically — serves a build that imports the `.wasm` binary as a precompiled `WebAssembly.Module`, as the Workers runtime requires. No extra configuration is needed:

```js
import { newContext } from "resvg-wasm";

export default {
  async fetch(request) {
    const ctx = await newContext();
    const png = ctx.render(
      '<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><circle cx="50" cy="50" r="40" fill="red"/></svg>',
    );
    return new Response(png, { headers: { "content-type": "image/png" } });
  },
};
```
