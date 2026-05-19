// Phase 3: wasm-bindgen exports.
// The core logic is in oz-core; this crate just re-exports it for the WASM target.

pub use oz_core::{MorphAnalysis, Stemmer, StemmerConfig};
