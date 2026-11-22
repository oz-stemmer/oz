# @oz/stemmer — Node.js bindings for Öz

Native Node.js bindings for the [Öz Turkish stemmer](https://github.com/oz-stemmer/oz). Rust core via napi-rs; prebuilt `.node` binaries for Linux/macOS/Windows on x64 and arm64.

## Install

```bash
npm install @oz/stemmer
```

## Quickstart

```js
import { Stemmer } from "@oz/stemmer";

const s = new Stemmer();

s.stem("kitaplardan");                    // → "kitap"
s.stemWithConfidence("evlerde");          // → { stem: "ev", confidence: 0.87 }
s.stemBatch(["arabalar", "masada"]);      // → ["araba", "masa"]

s.analyze("arabalardan");
// {
//   input: "arabalardan",
//   stem: "araba",
//   suffixes: [{ surface: "lar", label: "Plural" },
//              { surface: "dan", label: "Ablative" }],
//   vowelHarmonyOk: true,
//   dictValidated: false,
//   confidence: 0.75
// }

// Aggressive mode (better recall for search / IR)
const sIr = new Stemmer({ aggressive: true });
```

## API

| Method | Returns |
| --- | --- |
| `new Stemmer(config?)` | — |
| `.stem(word)` | `string` |
| `.stemWithConfidence(word)` | `{ stem: string, confidence: number }` |
| `.stemBatch(words)` | `string[]` |
| `.analyze(word)` | `MorphAnalysis` (input, stem, suffixes[], vowelHarmonyOk, dictValidated, confidence) |

`config` fields (all optional): `aggressive`, `maxWordLen`, `mlThreshold`, `dictValidation`.

## Development

```bash
cd bindings/node
npm install
npm run build        # release .node
node --test __test__ # tests
```
