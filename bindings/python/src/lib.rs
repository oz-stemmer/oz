//! Python bindings for the Öz Turkish stemmer via PyO3.
//!
//! Wraps `oz_core::Stemmer` with a thin PyO3 layer. The module is published
//! on PyPI as `oz-stemmer`; the import path is `oz_stemmer`.

use oz_core::{Stemmer as CoreStemmer, StemmerConfig};
use pyo3::prelude::*;

/// Configuration knobs mirroring `oz_core::StemmerConfig`.
#[pyclass(name = "StemmerConfig")]
#[derive(Clone)]
struct PyStemmerConfig {
    #[pyo3(get, set)]
    aggressive: bool,
    #[pyo3(get, set)]
    max_word_len: usize,
    #[pyo3(get, set)]
    ml_threshold: f32,
    #[pyo3(get, set)]
    dict_validation: bool,
}

#[pymethods]
impl PyStemmerConfig {
    #[new]
    #[pyo3(signature = (aggressive=false, max_word_len=64, ml_threshold=0.6, dict_validation=true))]
    fn new(aggressive: bool, max_word_len: usize, ml_threshold: f32, dict_validation: bool) -> Self {
        Self { aggressive, max_word_len, ml_threshold, dict_validation }
    }

    fn __repr__(&self) -> String {
        format!(
            "StemmerConfig(aggressive={}, max_word_len={}, ml_threshold={}, dict_validation={})",
            self.aggressive, self.max_word_len, self.ml_threshold, self.dict_validation
        )
    }
}

impl From<PyStemmerConfig> for StemmerConfig {
    fn from(c: PyStemmerConfig) -> Self {
        StemmerConfig {
            aggressive:      c.aggressive,
            max_word_len:    c.max_word_len,
            ml_threshold:    c.ml_threshold,
            dict_validation: c.dict_validation,
        }
    }
}

/// Turkish stemmer. Thread-safe for read-only use.
#[pyclass(name = "Stemmer")]
struct PyStemmer {
    inner: CoreStemmer,
}

#[pymethods]
impl PyStemmer {
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<PyStemmerConfig>) -> Self {
        let cfg = config.map(StemmerConfig::from).unwrap_or_default();
        Self { inner: CoreStemmer::new(cfg) }
    }

    /// Stem a single word and return the stem.
    fn stem(&self, word: &str) -> String {
        self.inner.stem(word)
    }

    /// Stem a single word and return ``(stem, confidence)``.
    fn stem_with_confidence(&self, word: &str) -> (String, f32) {
        self.inner.stem_with_confidence(word)
    }

    /// Stem a sequence of words. Releases the GIL during work.
    fn stem_batch(&self, py: Python<'_>, words: Vec<String>) -> Vec<String> {
        py.allow_threads(|| {
            let refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
            self.inner.stem_batch(&refs)
        })
    }

    /// Full morphological analysis as a dict.
    fn analyze<'py>(&self, py: Python<'py>, word: &str) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
        let a = self.inner.analyze(word);
        let d = pyo3::types::PyDict::new_bound(py);
        d.set_item("input", a.input)?;
        d.set_item("stem", a.stem)?;
        d.set_item("vowel_harmony_ok", a.vowel_harmony_ok)?;
        d.set_item("dict_validated", a.dict_validated)?;
        d.set_item("confidence", a.confidence)?;
        let suffixes: Vec<(String, String)> = a
            .suffixes
            .into_iter()
            .map(|s| (s.surface, s.label))
            .collect();
        d.set_item("suffixes", suffixes)?;
        Ok(d)
    }
}

#[pymodule]
fn _oz_stemmer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyStemmer>()?;
    m.add_class::<PyStemmerConfig>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
