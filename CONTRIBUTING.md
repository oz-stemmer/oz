# Contributing to Öz

Thank you for your interest in contributing. Öz is designed to be contribution-friendly — the most impactful contributions require no Rust knowledge at all.

## Ways to Contribute

| Type | Rust needed? | Difficulty |
|---|---|---|
| Add a test case to the corpus | No | Easy |
| Add or fix a suffix in the TOML DSL | No | Easy |
| Report a stemming error | No | Easy |
| Fix a bug in `oz-core` | Yes | Medium |
| Add a language binding | Yes | Hard |

---

## Adding a Test Case

The fastest way to improve Öz is to add word→stem pairs to the ground-truth corpus.

1. Open `data/test-corpus/ground-truth.tsv`
2. Add a line in this format:

   ```
   kelimeler<TAB>kelime<TAB>your-note-or-source
   ```

3. Find the right section (Plural, Locative, Verb, etc.) or add a new section header with `# ── Section name`.
4. Open a PR — corpus-only PRs are always welcome.

---

## Adding a Suffix

1. Find the right file in `data/suffix-tables/` (or create a new one for a new suffix class).
2. Add a TOML entry:

   ```toml
   [[suffix]]
   label   = "MyLabel"
   class   = "NounInflection"   # or VerbInflection | Derivational | NominalVerb
   surface = "lAr"              # canonical form with harmony markers
   forms   = ["lar", "ler"]     # all surface allomorphs, longest first
   harmony = "MatchBackness"    # or MatchBacknessRounded | Any
   buffer  = "None"             # or Y | N | S
   min_stem_syllables = 1
   ```

3. Add at least two test cases to `data/test-corpus/ground-truth.tsv`.
4. Open a PR.

**Harmony markers used in `surface`:**
| Marker | Meaning |
|---|---|
| `A` | a / e (back / front) |
| `I` | ı / i / u / ü |
| `U` | u / ü |
| `D` | d / t (voiced / devoiced) |
| `C` | c / ç |

---

## Reporting a Stemming Error

Open a GitHub Issue with:

- The input word
- What Öz currently produces
- What the correct stem should be
- A source (dictionary, corpus, your own knowledge of Turkish)

Use the **"Stemming error"** issue template.

---

## Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/oz-stemmer/oz.git
cd oz
cargo build --workspace

# Run tests
cargo test --workspace

# Run clippy (zero warnings policy)
cargo clippy --workspace -- -D warnings

# Run accuracy evaluation (after building the CLI)
cargo build --release -p oz-cli
python3 scripts/eval_accuracy.py \
  --corpus data/test-corpus/ground-truth.tsv \
  --stemmer target/release/oz \
  --threshold 0.94
```

---

## Pull Request Checklist

- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace -- -D warnings` produces no warnings
- [ ] `cargo fmt --all -- --check` passes
- [ ] New suffixes have at least 2 test cases in the corpus
- [ ] New Rust functions have at least 1 unit test
- [ ] Commit message follows the format: `type(scope): description`

---

## Commit Message Format

```
feat(oz-core): add causative suffix -dır
fix(oz-core): prevent over-stemming of monosyllabic k-final stems
test(corpus): add 30 verb tense pairs
docs: update README installation instructions
```

Types: `feat`, `fix`, `test`, `docs`, `refactor`, `perf`, `ci`

---

## Accuracy Policy

The CI accuracy gate requires ≥ 94% on the ground-truth corpus. A PR that drops accuracy below this threshold will not be merged. If you are adding new test cases that expose existing bugs, open a separate issue for the bug — don't block your corpus PR on it.

---

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.
