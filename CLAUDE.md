# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

resvg-wasm compiles the Rust resvg SVG rendering library to WebAssembly, providing SVG-to-PNG rendering for both Node.js and browser environments. Published as an npm package with zero runtime dependencies.

## Build Commands

- `yarn build` -- full build (Rust -> WASM -> TypeScript). Requires `cargo`, `wasm-bindgen`, and `wasm-opt` on PATH. Set `DISABLE_WASM_OPT=1` to skip wasm-opt.
- `yarn lint` -- ESLint on TypeScript source files
- `yarn format` -- Prettier formatting
- `yarn needs-format` -- check if formatting is needed
- `yarn clean` -- remove build artifacts

There are no automated tests beyond a placeholder in `tests/web.rs`.

## Architecture

**Rust layer** (`src/lib.rs`): The `Context` struct (exported via wasm-bindgen) holds registered font data and exposes `registerFontData()`, `render()`, and `listFonts()` to JS. `newContext()` is the factory function. Rendering parses SVG with usvg, converts text to paths using fontdb, rasterizes with tiny-skia, and encodes to PNG.

**TypeScript layer**: Two entry points compile to different module formats:
- `index.ts` -- browser entry (ESM), uses dynamic `import()` for WASM loading with a module cache
- `index_main.ts` -- Node.js entry (CommonJS), same pattern but compiled differently. During build, this replaces `index.js` in the `lib/` output.

`iresvg.ts` defines the `Context` TypeScript interface matching the Rust wasm-bindgen exports.

**Dual-target build** (`build.sh`):
1. `cargo build` compiles Rust to `wasm32-unknown-unknown`
2. `wasm-bindgen` generates JS/TS bindings twice: `--target bundler` (browser) and `--target nodejs`
3. `wasm-opt -O3` optimizes both WASM binaries
4. TypeScript compiles twice: `tsconfig.json` (CommonJS -> `lib/`) and `tsconfig.browser.json` (ESNext -> `lib.browser/`)
5. Build script reorganizes outputs so `lib/core/` has Node.js WASM bindings, `lib.browser/core/` has bundler bindings

**Package exports**: Conditional exports in `package.json` route `node` imports to `lib/` and `module` imports to `lib.browser/`.

## Rust Build Configuration

`.cargo/config` targets `wasm32-unknown-unknown` with SIMD, bulk-memory, and other WASM features enabled. Uses `build-std` to compile a custom standard library (requires nightly Rust).
