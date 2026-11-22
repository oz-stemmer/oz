//! WASM bindings for the Öz Turkish stemmer.
//!
//! Published on npm as `@oz/stemmer-wasm`. Uses wasm-bindgen for the JS
//! interop layer. Suitable for browser and edge runtimes.

use oz_core::{Stemmer as CoreStemmer, StemmerConfig};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsStemmerConfig {
    #[serde(default)]
    aggressive: Option<bool>,
    #[serde(default)]
    max_word_len: Option<u32>,
    #[serde(default)]
    ml_threshold: Option<f32>,
    #[serde(default)]
    dict_validation: Option<bool>,
}

impl From<JsStemmerConfig> for StemmerConfig {
    fn from(c: JsStemmerConfig) -> Self {
        let defaults = StemmerConfig::default();
        StemmerConfig {
            aggressive:      c.aggressive.unwrap_or(defaults.aggressive),
            max_word_len:    c.max_word_len.map(|v| v as usize).unwrap_or(defaults.max_word_len),
            ml_threshold:    c.ml_threshold.unwrap_or(defaults.ml_threshold),
            dict_validation: c.dict_validation.unwrap_or(defaults.dict_validation),
        }
    }
}

/// Turkish stemmer, exported to JavaScript.
#[wasm_bindgen]
pub struct Stemmer {
    inner: CoreStemmer,
}

#[wasm_bindgen]
impl Stemmer {
    /// Create a stemmer. Optional `config` is a plain JS object matching
    /// `StemmerConfig` (camelCase keys: `aggressive`, `maxWordLen`, etc.).
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Result<Stemmer, JsValue> {
        let cfg = if config.is_null() || config.is_undefined() {
            StemmerConfig::default()
        } else {
            let js: JsStemmerConfig =
                serde_wasm_bindgen::from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;
            js.into()
        };
        Ok(Self { inner: CoreStemmer::new(cfg) })
    }

    /// Stem a single word and return the stem.
    #[wasm_bindgen]
    pub fn stem(&self, word: &str) -> String {
        self.inner.stem(word)
    }

    /// Stem a single word and return `{ stem, confidence }`.
    #[wasm_bindgen(js_name = stemWithConfidence)]
    pub fn stem_with_confidence(&self, word: &str) -> Result<JsValue, JsValue> {
        let (stem, confidence) = self.inner.stem_with_confidence(word);
        #[derive(Serialize)]
        struct Out<'a> { stem: &'a str, confidence: f32 }
        serde_wasm_bindgen::to_value(&Out { stem: &stem, confidence })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Stem an array of words.
    #[wasm_bindgen(js_name = stemBatch)]
    pub fn stem_batch(&self, words: Vec<String>) -> Vec<String> {
        let refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        self.inner.stem_batch(&refs)
    }

    /// Full morphological analysis as a plain JS object.
    #[wasm_bindgen]
    pub fn analyze(&self, word: &str) -> Result<JsValue, JsValue> {
        let a = self.inner.analyze(word);
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Suffix { surface: String, label: String }
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Out {
            input:            String,
            stem:             String,
            suffixes:         Vec<Suffix>,
            vowel_harmony_ok: bool,
            dict_validated:   bool,
            confidence:       f32,
        }
        let out = Out {
            input:            a.input,
            stem:             a.stem,
            suffixes:         a.suffixes.into_iter()
                .map(|s| Suffix { surface: s.surface, label: s.label })
                .collect(),
            vowel_harmony_ok: a.vowel_harmony_ok,
            dict_validated:   a.dict_validated,
            confidence:       a.confidence,
        };
        serde_wasm_bindgen::to_value(&out).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// Package version, exposed so JS callers can confirm they are on the right wheel.
#[wasm_bindgen(js_name = version)]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}
