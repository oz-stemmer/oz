//! Browser-equivalent smoke tests for the WASM Stemmer.
//! Run with `wasm-pack test --node` (or `--chrome`, `--firefox`, etc.).

use oz_wasm::{version, Stemmer};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;


#[wasm_bindgen_test]
fn version_exposed() {
    assert!(!version().is_empty());
}

#[wasm_bindgen_test]
fn stem_simple_plural() {
    let s = Stemmer::new(JsValue::NULL).unwrap();
    assert_eq!(s.stem("evler"), "ev");
    assert_eq!(s.stem("arabalar"), "araba");
}

#[wasm_bindgen_test]
fn stem_batch_matches_single() {
    let s = Stemmer::new(JsValue::NULL).unwrap();
    let words = vec!["evler".to_owned(), "arabalar".to_owned(), "masada".to_owned()];
    let batch = s.stem_batch(words.clone());
    let singles: Vec<String> = words.iter().map(|w| s.stem(w)).collect();
    assert_eq!(batch, singles);
}

#[wasm_bindgen_test]
fn non_turkish_passthrough() {
    let s = Stemmer::new(JsValue::NULL).unwrap();
    assert_eq!(s.stem("stress"), "stress");
}
