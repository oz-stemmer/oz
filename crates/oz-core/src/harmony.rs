/// Turkish vowel harmony checker.
///
/// Turkish vowels split along two axes:
///   - backness : front (e i ö ü) vs back (a ı o u)
///   - roundness: rounded (o ö u ü) vs unrounded (a e ı i)
///
/// A suffix vowel must agree in backness with the last stem vowel.
/// Rounded suffixes additionally require a rounded stem vowel.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VowelKind {
    Front,
    Back,
    None, // consonant or not a Turkish vowel
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Roundness {
    Rounded,
    Unrounded,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VowelFeatures {
    pub kind:      VowelKind,
    pub roundness: Roundness,
}

impl VowelFeatures {
    pub const fn none() -> Self {
        Self { kind: VowelKind::None, roundness: Roundness::None }
    }
}

/// Extract vowel features for a single Unicode scalar.
/// Returns `VowelFeatures::none()` for consonants.
pub const fn vowel_features(c: char) -> VowelFeatures {
    match c {
        'a' | 'A' => VowelFeatures { kind: VowelKind::Back,  roundness: Roundness::Unrounded },
        'e' | 'E' => VowelFeatures { kind: VowelKind::Front, roundness: Roundness::Unrounded },
        'ı' | 'I' => VowelFeatures { kind: VowelKind::Back,  roundness: Roundness::Unrounded },
        'i' | 'İ' => VowelFeatures { kind: VowelKind::Front, roundness: Roundness::Unrounded },
        'o' | 'O' => VowelFeatures { kind: VowelKind::Back,  roundness: Roundness::Rounded   },
        'ö' | 'Ö' => VowelFeatures { kind: VowelKind::Front, roundness: Roundness::Rounded   },
        'u' | 'U' => VowelFeatures { kind: VowelKind::Back,  roundness: Roundness::Rounded   },
        'ü' | 'Ü' => VowelFeatures { kind: VowelKind::Front, roundness: Roundness::Rounded   },
        _ =>         VowelFeatures::none(),
    }
}

pub fn is_vowel(c: char) -> bool {
    !matches!(vowel_features(c).kind, VowelKind::None)
}

/// Return the last vowel found in `s`, scanning right-to-left.
pub fn last_vowel(s: &str) -> VowelFeatures {
    s.chars().rev().find_map(|c| {
        let f = vowel_features(c);
        if f.kind != VowelKind::None { Some(f) } else { None }
    }).unwrap_or_else(VowelFeatures::none)
}

/// Count the number of vowels (== syllables) in `s`.
pub fn syllable_count(s: &str) -> usize {
    s.chars().filter(|&c| is_vowel(c)).count()
}

/// Harmony requirement a suffix declares for its own vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarmonyReq {
    /// Suffix front/back vowel must match stem's last vowel.
    MatchBackness,
    /// Additionally the stem must have a rounded vowel.
    MatchBacknessRounded,
    /// No harmony requirement (some adverbial/invariant suffixes).
    Any,
}

/// Returns `true` if the suffix vowel `suf` is harmonically legal
/// after a stem whose last vowel has features `stem`.
pub fn harmony_ok(stem: VowelFeatures, suf: VowelFeatures, req: HarmonyReq) -> bool {
    match req {
        HarmonyReq::Any => true,
        HarmonyReq::MatchBackness => stem.kind == suf.kind,
        HarmonyReq::MatchBacknessRounded => {
            stem.kind == suf.kind && stem.roundness == Roundness::Rounded
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front_back_classification() {
        assert_eq!(vowel_features('e').kind, VowelKind::Front);
        assert_eq!(vowel_features('a').kind, VowelKind::Back);
        assert_eq!(vowel_features('ü').kind, VowelKind::Front);
        assert_eq!(vowel_features('u').kind, VowelKind::Back);
    }

    #[test]
    fn last_vowel_extraction() {
        assert_eq!(last_vowel("kitap").kind, VowelKind::Back);
        assert_eq!(last_vowel("ev").kind, VowelKind::Front);
        assert_eq!(last_vowel("xyz"), VowelFeatures::none()); // no vowel
    }

    #[test]
    fn syllable_count_basic() {
        assert_eq!(syllable_count("araba"), 3);
        assert_eq!(syllable_count("ev"),    1);
        assert_eq!(syllable_count("kitap"), 2);
    }

    #[test]
    fn harmony_check_simple() {
        let back  = vowel_features('a');
        let front = vowel_features('e');
        assert!( harmony_ok(back,  back,  HarmonyReq::MatchBackness));
        assert!(!harmony_ok(back,  front, HarmonyReq::MatchBackness));
        assert!( harmony_ok(front, front, HarmonyReq::MatchBackness));
    }
}
