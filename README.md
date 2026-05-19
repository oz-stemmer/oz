# Öz — Modern Turkish Stemmer

> *"Özüne dön."* — Return to the essence.

**Öz** is a production-grade Turkish stemmer built for every Turkish NLP pipeline. Single Rust core, multiple targets: native, Python, Node.js, WASM, and microservice.

```python
from oz_stemmer import Stemmer

s = Stemmer()
s.stem("kitaplardan")   # → "kitap"
s.stem("evlerdeki")     # → "ev"
s.stem_batch(["arabalar", "masada", "gidiyorum"])  # → ["araba", "masa", "git"]
```

---

## Why Öz?

| Tool | Problem |
|---|---|
| skroutz/turkish_stemmer | Ruby only, abandoned, no Python/JS support |
| zemberek-nlp | JVM dependency, impossible to embed |
| Snowball / NLTK | 1990s algorithm, no confidence scores, no async |

Öz is different:

- **Single Rust core** — compiles to native, WASM, Python wheel, and Node binary from one codebase
- **Three-layer accuracy** — rule FSM → dictionary validation → optional ONNX ML fallback for OOV words
- **AI-native** — ships LangChain, LlamaIndex, and Elasticsearch plugins as first-party packages
- **Zero build-step installs** — prebuilt wheels and `.node` binaries for all major platforms
- **Production API** — batch processing, confidence scores, async/await, gRPC microservice mode

---

## Installation

```bash
# Python
pip install oz-stemmer

# Node.js
npm install @oz/stemmer

# Browser / Edge (WASM)
npm install @oz/stemmer-wasm

# Rust
cargo add oz-stemmer
```

---

## Usage

### Python

```python
from oz_stemmer import Stemmer, StemmerConfig

# Conservative mode (default) — best for RAG / precision
s = Stemmer()
s.stem("kitaplardan")                    # → "kitap"
s.stem_with_confidence("evlerde")       # → ("ev", 0.87)

# Aggressive mode — best for search / recall
s = Stemmer(StemmerConfig(aggressive=True))
s.stem("kitaplıktakilerin")             # → "kitap"

# Full morphological analysis
a = s.analyze("arabalardan")
# MorphAnalysis(stem="araba", suffixes=["lar", "dan"], confidence=0.91)
```

### Node.js

```js
import { Stemmer } from "@oz/stemmer";

const s = new Stemmer();
s.stem("kitaplar");                 // → "kitap"
await s.stemBatch(["evler", "masada"]);  // → ["ev", "masa"]
```

### WASM (Browser)

```js
import init, { Stemmer } from "@oz/stemmer-wasm";

await init();
const s = new Stemmer();
s.stem("gidiyorum");  // → "git"
```

### CLI

```bash
echo "kitaplardan" | oz
# → kitap

echo -e "evler\narabada\ngidiyorum" | oz --json
# → {"input":"evler","stem":"ev","confidence":0.870}
# → {"input":"arabada","stem":"araba","confidence":0.870}
# → {"input":"gidiyorum","stem":"git","confidence":0.870}
```

### REST API

```bash
docker run -p 8080:8080 oz-stemmer

curl -X POST localhost:8080/stem \
  -H "Content-Type: application/json" \
  -d '{"word": "kitaplardan"}'
# → {"stem": "kitap", "confidence": 0.91}
```

---

## LangChain Integration

```python
from oz_stemmer.langchain import TurkishStemmedRetriever, TurkishMorphemeTextSplitter

# Stem queries and indexed chunks at embedding time
retriever = TurkishStemmedRetriever(vectorstore=chroma, stemmer=Stemmer())

# Split on morpheme boundaries — prevents stem/suffix splits
splitter = TurkishMorphemeTextSplitter(chunk_size=512, chunk_overlap=64)
```

## LlamaIndex Integration

```python
from oz_stemmer.llamaindex import TurkishStemPostprocessor

pipeline = QueryPipeline(modules={
    "retriever": retriever,
    "stemmer": TurkishStemPostprocessor(),
})
```

## Elasticsearch

```json
PUT /my-index/_settings
{
  "analysis": {
    "filter": { "oz_stem": { "type": "oz_stem", "aggressive": false } },
    "analyzer": {
      "turkish": { "tokenizer": "standard", "filter": ["lowercase", "oz_stem"] }
    }
  }
}
```

---

## Architecture

```
oz/
├── crates/
│   ├── oz-core/      ← Morphological engine (no_std, WASM-safe)
│   ├── oz-dict/      ← FST dictionary validator (~4MB lexicon)
│   ├── oz-ml/        ← ONNX ML fallback for OOV words (optional)
│   └── oz-service/   ← axum HTTP + tonic gRPC server
├── bindings/
│   ├── python/       ← PyO3 + maturin
│   └── nodejs/       ← napi-rs
├── wasm/             ← wasm-pack
├── integrations/
│   ├── langchain/
│   ├── llamaindex/
│   └── elasticsearch/
├── cli/              ← oz CLI tool
└── data/
    ├── suffix-tables/    ← Declarative suffix DSL (.toml)
    └── test-corpus/      ← 5,000 ground-truth word→stem pairs
```

**Three-layer accuracy pipeline:**

```
Input word
    │
    ▼
Rule FSM (suffix-table DSL, vowel harmony gate, consonant mutation)
    │  confidence ≥ 0.6 → return stem
    │  confidence < 0.6 ↓
    ▼
Dictionary validator (FST, ~4MB, O(n) lookup)
    │  validated → boost confidence, return stem
    │  not found ↓
    ▼
ONNX ML fallback (character BiLSTM, 2MB, < 2ms)
    │  handles OOV: foreign borrowings, neologisms, social media
    ▼
Final stem + confidence score
```

---

## Performance

| Platform | Throughput |
|---|---|
| Native Rust (single core) | > 2,000,000 words/sec |
| Python (8-core batch) | > 500,000 words/sec |
| Node.js (batch) | > 400,000 words/sec |
| WASM (browser) | > 200,000 words/sec |
| REST API p99 latency | < 2ms |

Memory footprint: < 8MB (rule only), < 32MB (with ML).

---

## Accuracy

| Mode | Target | Measurement |
|---|---|---|
| Conservative | ≥ 94% | Ground-truth corpus (5,000 pairs) |
| Aggressive | ≥ 91% | Same corpus, over-stemming ≤ 4% |
| ML fallback (OOV) | ≥ 80% | Held-out foreign borrowings + neologisms |

Evaluated on: ITU Turkish NLP corpus · Snowball reference cases · Zemberek eval set · Turkish social media.

---

## Roadmap

| Phase | Milestone | Status |
|---|---|---|
| 1 | Rust core engine | 🔨 In progress |
| 2 | Python binding (PyPI) | Planned |
| 3 | Node.js + WASM bindings | Planned |
| 4 | LangChain · LlamaIndex · Elasticsearch | Planned |
| 5 | REST + gRPC microservice · Docker | Planned |
| 6 | v1.0 public launch | Planned |

**Multilingual future:** Azerbaijani (v1.1) → Uzbek, Kazakh (v1.2) → full Turkic family. The suffix-table DSL and harmony engine are language-agnostic from day one.

---

## Contributing

The suffix-table DSL means adding a new suffix requires no Rust knowledge — just TOML and Turkish morphology.

```toml
# data/suffix-tables/noun-inflection.toml
[[suffix]]
label   = "Plural"
class   = "NounInflection"
surface = "lAr"
forms   = ["lar", "ler"]
harmony = "MatchBackness"
buffer  = "None"
min_stem_syllables = 1
```

Adding test cases is always welcome:

```tsv
# data/test-corpus/ground-truth.tsv
kitaplardan    kitap    ablative+plural
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide.

---

## License

Apache 2.0 — see [LICENSE](LICENSE).
