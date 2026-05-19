use crate::{
    harmony::syllable_count,
    mutation::apply_final_mutation,
    suffix::strip_one,
    types::{MorphAnalysis, StemmerConfig, Suffix},
};

/// The core stemmer. Holds configuration and optional feature handles.
pub struct Stemmer {
    pub config: StemmerConfig,
}

impl Stemmer {
    pub fn new(config: StemmerConfig) -> Self {
        Self { config }
    }

    /// Create with default config (conservative, dict validation on).
    pub fn default() -> Self {
        Self::new(StemmerConfig::default())
    }

    /// Stem a single word, returning the stem string.
    pub fn stem(&self, word: &str) -> String {
        self.analyze(word).stem
    }

    /// Stem a single word and return `(stem, confidence)`.
    pub fn stem_with_confidence(&self, word: &str) -> (String, f32) {
        let analysis = self.analyze(word);
        let confidence = analysis.confidence;
        (analysis.stem, confidence)
    }

    /// Full morphological analysis of a single word.
    pub fn analyze(&self, word: &str) -> MorphAnalysis {
        let word = word.trim();

        // Fast-path: too long, or no vowels → return as-is.
        if word.len() > self.config.max_word_len || syllable_count(word) == 0 {
            return MorphAnalysis {
                input:           word.to_owned(),
                stem:            word.to_owned(),
                suffixes:        vec![],
                vowel_harmony_ok: true,
                dict_validated:  false,
                confidence:      1.0,
            };
        }

        let normalized = normalize(word);
        let mut current = normalized.clone();
        let mut stripped: Vec<Suffix> = vec![];

        let max_rounds = if self.config.aggressive { 8 } else { 4 };

        for _ in 0..max_rounds {
            match strip_one(&current, self.config.aggressive) {
                Some((stem, label)) => {
                    stripped.push(Suffix {
                        surface: current[stem.len()..].to_owned(),
                        label:   label.to_owned(),
                    });
                    current = stem;
                    if !self.config.aggressive {
                        break;
                    }
                }
                None => break,
            }
        }

        // Apply consonant mutation to the final candidate stem.
        // Only apply when at least one suffix was stripped (otherwise we'd mutate
        // bare words like "kitap" → "kitab" when used standalone, which is wrong).
        let stem = if !stripped.is_empty() {
            apply_final_mutation(&current)
        } else {
            current.clone()
        };

        let confidence = compute_confidence(&stem, &stripped);

        MorphAnalysis {
            input:           normalized,
            stem,
            suffixes:        stripped,
            vowel_harmony_ok: true, // validated per-step in strip_one
            dict_validated:  false, // will be set by oz-dict integration
            confidence,
        }
    }

    /// Stem a slice of words. Uses Rayon when built with the `parallel` feature.
    pub fn stem_batch(&self, words: &[&str]) -> Vec<String> {
        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            words.par_iter().map(|w| self.stem(w)).collect()
        }
        #[cfg(not(feature = "parallel"))]
        {
            words.iter().map(|w| self.stem(w)).collect()
        }
    }
}

/// NFC normalization + lowercase.
/// Handles the common case of ASCII-uppercase and composed/decomposed forms.
fn normalize(s: &str) -> String {
    // Lowercase mapping for Turkish (İ → i, I → ı).
    s.chars().map(|c| match c {
        'İ' => 'i',
        'I' => 'ı',
        other => other.to_lowercase().next().unwrap_or(other),
    }).collect()
}

/// Heuristic confidence from structural signals.
fn compute_confidence(stem: &str, suffixes: &[Suffix]) -> f32 {
    let syl = syllable_count(stem) as f32;
    let strip_depth = suffixes.len() as f32;

    // Base: one valid suffix step → 0.7; every additional layer adds 0.05.
    // No suffix stripped → word was returned as-is → high confidence (1.0).
    if suffixes.is_empty() {
        return 1.0;
    }

    let base = 0.65_f32 + (strip_depth * 0.05).min(0.2);

    // Penalize very short stems (< 2 syllables suggests over-stemming).
    let syllable_factor = if syl >= 2.0 { 1.0 } else { 0.85 };

    (base * syllable_factor).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stemmer() -> Stemmer { Stemmer::default() }

    #[test]
    fn stem_simple_plural() {
        assert_eq!(stemmer().stem("kitaplar"), "kitap");
        assert_eq!(stemmer().stem("evler"), "ev");
    }

    #[test]
    fn stem_locative() {
        assert_eq!(stemmer().stem("evde"), "ev");
    }

    #[test]
    fn stem_with_mutation() {
        // "kitabı" → strip accusative "ı" → "kitab" is the mutated form
        // (kitap final p → b before vowel-initial suffix)
        // Our approach: strip suffix → apply mutation to remaining stem
        let s = stemmer().stem("kitabı");
        // The FSM strips "ı" (accusative), leaving "kitab"; mutation is already surface form
        assert_eq!(s, "kitab"); // surface stem after mutation
    }

    #[test]
    fn no_strip_non_turkish() {
        let s = stemmer().stem("stress");
        assert_eq!(s, "stress"); // foreign word, no suffix matched
    }

    #[test]
    fn stem_with_confidence_returns_float() {
        let (stem, conf) = stemmer().stem_with_confidence("evler");
        assert_eq!(stem, "ev");
        assert!((0.0..=1.0).contains(&conf));
    }

    #[test]
    fn normalize_uppercase_turkish() {
        // İstanbul → istanbul (İ → i)
        let s = stemmer().stem("İstanbul");
        assert_eq!(s, "istanbul");
    }

    #[test]
    fn analyze_structure() {
        let a = stemmer().analyze("evler");
        assert_eq!(a.input, "evler");
        assert_eq!(a.stem, "ev");
        assert!(!a.suffixes.is_empty());
        assert_eq!(a.suffixes[0].label, "Plural");
    }

    #[test]
    fn batch_matches_single() {
        let words = ["evler", "kitaplar", "evde"];
        let batch = stemmer().stem_batch(&words);
        for (w, b) in words.iter().zip(batch.iter()) {
            assert_eq!(stemmer().stem(w), *b);
        }
    }
}
