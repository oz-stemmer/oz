//! Node.js bindings for the Öz Turkish stemmer via napi-rs.
//!
//! Published on npm as `@oz/stemmer`. The `.node` binary is loaded via the
//! generated glue in `index.js`.

#![deny(clippy::all)]

use napi_derive::napi;
use oz_core::{Stemmer as CoreStemmer, StemmerConfig};

/// Configuration knobs mirroring `oz_core::StemmerConfig`.
#[napi(object)]
pub struct JsStemmerConfig {
    pub aggressive:      Option<bool>,
    pub max_word_len:    Option<u32>,
    pub ml_threshold:    Option<f64>,
    pub dict_validation: Option<bool>,
}

impl From<JsStemmerConfig> for StemmerConfig {
    fn from(c: JsStemmerConfig) -> Self {
        let defaults = StemmerConfig::default();
        StemmerConfig {
            aggressive:      c.aggressive.unwrap_or(defaults.aggressive),
            max_word_len:    c.max_word_len.map(|v| v as usize).unwrap_or(defaults.max_word_len),
            ml_threshold:    c.ml_threshold.map(|v| v as f32).unwrap_or(defaults.ml_threshold),
            dict_validation: c.dict_validation.unwrap_or(defaults.dict_validation),
        }
    }
}

/// One stripped suffix in a morphological analysis.
#[napi(object)]
pub struct JsSuffix {
    pub surface: String,
    pub label:   String,
}

/// Full morphological analysis of a single word.
#[napi(object)]
pub struct JsMorphAnalysis {
    pub input:            String,
    pub stem:             String,
    pub suffixes:         Vec<JsSuffix>,
    pub vowel_harmony_ok: bool,
    pub dict_validated:   bool,
    pub confidence:       f64,
}

/// Stem + confidence pair returned by `stemWithConfidence`.
#[napi(object)]
pub struct JsStemResult {
    pub stem:       String,
    pub confidence: f64,
}

/// Turkish stemmer.
#[napi]
pub struct Stemmer {
    inner: CoreStemmer,
}

#[napi]
impl Stemmer {
    #[napi(constructor)]
    pub fn new(config: Option<JsStemmerConfig>) -> Self {
        let cfg = config.map(StemmerConfig::from).unwrap_or_default();
        Self { inner: CoreStemmer::new(cfg) }
    }

    /// Stem a single word and return the stem.
    #[napi]
    pub fn stem(&self, word: String) -> String {
        self.inner.stem(&word)
    }

    /// Stem a single word and return `{ stem, confidence }`.
    #[napi]
    pub fn stem_with_confidence(&self, word: String) -> JsStemResult {
        let (stem, confidence) = self.inner.stem_with_confidence(&word);
        JsStemResult { stem, confidence: confidence as f64 }
    }

    /// Stem a batch of words synchronously.
    #[napi]
    pub fn stem_batch(&self, words: Vec<String>) -> Vec<String> {
        let refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        self.inner.stem_batch(&refs)
    }

    /// Full morphological analysis.
    #[napi]
    pub fn analyze(&self, word: String) -> JsMorphAnalysis {
        let a = self.inner.analyze(&word);
        JsMorphAnalysis {
            input:            a.input,
            stem:             a.stem,
            suffixes:         a.suffixes.into_iter()
                .map(|s| JsSuffix { surface: s.surface, label: s.label })
                .collect(),
            vowel_harmony_ok: a.vowel_harmony_ok,
            dict_validated:   a.dict_validated,
            confidence:       a.confidence as f64,
        }
    }
}
