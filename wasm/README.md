# @oz/stemmer-wasm — Browser bindings for Öz

WASM build of the [Öz Turkish stemmer](https://github.com/oz-stemmer/oz) for browsers and edge runtimes. Built with wasm-bindgen; no polyfills, no DOM required.

## Install

```bash
npm install @oz/stemmer-wasm
```

## Quickstart

```js
import init, { Stemmer, version } from "@oz/stemmer-wasm";

await init(); // load the .wasm

const s = new Stemmer();
s.stem("kitaplardan");                     // → "kitap"
s.stemWithConfidence("evlerde");           // → { stem: "ev", confidence: 0.87 }
s.stemBatch(["arabalar", "masada"]);       // → ["araba", "masa"]

s.analyze("arabalardan");
// { input, stem, suffixes: [{ surface, label }, ...],
//   vowelHarmonyOk, dictValidated, confidence }

// Aggressive mode (better recall for search)
const sIr = new Stemmer({ aggressive: true });
```

Call `s.free()` (or the `[Symbol.dispose]()` fallback) when done to release the WASM-side allocation.

## Targets

| wasm-pack target | Use case |
| --- | --- |
| `--target web` | Native `<script type="module">` / Vite / modern bundlers |
| `--target bundler` | Webpack / Rollup bundler integration |
| `--target nodejs` | Node.js / Deno via CommonJS-style init |

## Build

```bash
cd wasm
wasm-pack build --target web --release
# output: wasm/pkg/
```

## Tests

```bash
wasm-pack test --node wasm   # 4 tests
```
