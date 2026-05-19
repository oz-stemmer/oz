use fst::{Set, IntoStreamer, Streamer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DictError {
    #[error("FST error: {0}")]
    Fst(#[from] fst::Error),
    #[error("Dictionary data not found or corrupted")]
    NotFound,
}

/// A compiled FST word list for validating stemmer output.
pub struct DictValidator {
    set: Set<Vec<u8>>,
}

impl DictValidator {
    /// Load from raw FST bytes (e.g., `include_bytes!("../data/lexicon.fst")`).
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, DictError> {
        let set = Set::new(bytes)?;
        Ok(Self { set })
    }

    /// Build from a sorted iterator of UTF-8 words (for testing / offline build).
    pub fn build_from_iter<I>(words: I) -> Result<Vec<u8>, DictError>
    where
        I: IntoIterator<Item = String>,
    {
        let mut builder = fst::SetBuilder::memory();
        for w in words {
            builder.insert(w).map_err(|e| DictError::Fst(fst::Error::Fst(e)))?;
        }
        Ok(builder.into_inner().map_err(|e| DictError::Fst(fst::Error::Fst(e)))?)
    }

    /// Returns `true` if `word` is present in the dictionary.
    pub fn contains(&self, word: &str) -> bool {
        self.set.contains(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(words: &[&str]) -> DictValidator {
        let mut sorted: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        sorted.sort();
        let bytes = DictValidator::build_from_iter(sorted).unwrap();
        DictValidator::from_bytes(bytes).unwrap()
    }

    #[test]
    fn contains_inserted_word() {
        let d = build(&["araba", "ev", "kitap"]);
        assert!( d.contains("araba"));
        assert!( d.contains("ev"));
        assert!(!d.contains("xyz"));
    }

    #[test]
    fn empty_dict() {
        let d = build(&[]);
        assert!(!d.contains("ev"));
    }
}
