# oz-stemmer — Python bindings for Öz

Python wheel for the [Öz Turkish stemmer](https://github.com/oz-stemmer/oz). Rust core via PyO3 + maturin; no JVM, no external runtime.

## Install

```bash
pip install oz-stemmer
```

## Quickstart

```python
from oz_stemmer import Stemmer, StemmerConfig

s = Stemmer()
s.stem("kitaplardan")                       # → "kitap"
s.stem_with_confidence("evlerde")           # → ("ev", 0.87)
s.stem_batch(["arabalar", "masada"])        # → ["araba", "masa"]

# Full morphological analysis
a = s.analyze("arabalardan")
# {"input": "arabalardan", "stem": "araba",
#  "suffixes": [("lar", "Plural"), ("dan", "Ablative")],
#  "vowel_harmony_ok": True, "dict_validated": False, "confidence": 0.75}

# Aggressive mode (strip more layers — better recall for IR)
s_ir = Stemmer(StemmerConfig(aggressive=True))
```

## API

| Method | Returns |
| --- | --- |
| `Stemmer(config=None).stem(word)` | `str` |
| `.stem_with_confidence(word)` | `tuple[str, float]` |
| `.stem_batch(words)` | `list[str]` — GIL released during work |
| `.analyze(word)` | `dict` (input, stem, suffixes, vowel_harmony_ok, dict_validated, confidence) |

## Development

```bash
# One-time: install maturin
pip install maturin

# Build and install into the current venv
cd bindings/python
maturin develop --release

# Run tests
pytest -q
```

Wheels are built in CI via `maturin build --release` for each (OS × Python) pair.
