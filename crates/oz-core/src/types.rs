use serde::{Deserialize, Serialize};

/// Configuration knobs passed to every `Stemmer` instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StemmerConfig {
    /// Strip as many suffix layers as possible (IR/search).
    /// When false (default), stops at the first valid stem (RAG/precision).
    pub aggressive: bool,

    /// Maximum characters a word may have; longer tokens are returned as-is.
    pub max_word_len: usize,

    /// Confidence below which the ML fallback is invoked (0.0–1.0).
    /// Only meaningful when the binary is built with the `ml` feature.
    pub ml_threshold: f32,

    /// Whether to validate stems against the compiled FST dictionary.
    pub dict_validation: bool,
}

impl Default for StemmerConfig {
    fn default() -> Self {
        Self {
            aggressive:      false,
            max_word_len:    64,
            ml_threshold:    0.6,
            dict_validation: true,
        }
    }
}

/// A single identified suffix, produced by `Stemmer::analyze`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Suffix {
    /// Surface form as it appears in the input word.
    pub surface: String,
    /// Canonical suffix label from the suffix table.
    pub label: String,
}

/// Full morphological decomposition of one word.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphAnalysis {
    /// Original input word.
    pub input: String,
    /// Identified stem.
    pub stem: String,
    /// Ordered list of stripped suffixes (innermost first).
    pub suffixes: Vec<Suffix>,
    /// Harmony satisfied across the entire word.
    pub vowel_harmony_ok: bool,
    /// Stem was found in the FST dictionary (requires `dict` feature).
    pub dict_validated: bool,
    /// Confidence score in \[0.0, 1.0\].
    pub confidence: f32,
}
