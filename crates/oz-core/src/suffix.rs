use crate::harmony::{HarmonyReq, last_vowel, vowel_features, VowelKind};

/// Broad morphological class of a suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuffixClass {
    NounInflection,
    VerbInflection,
    NominalVerb,   // copular / evidential suffixes
    Derivational,
}

/// Buffer consonant inserted between a vowel-final stem and a vowel-initial suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Buffer {
    None,
    Y, // araba+y+a
    N, // araba+sı+n+ı
    S, // araba+s+ı
}

/// One entry in the suffix table — describes a single suffix allomorph group.
#[derive(Debug, Clone)]
pub struct SuffixDef {
    pub label:   &'static str,
    pub class:   SuffixClass,
    pub harmony: HarmonyReq,
    pub buffer:  Buffer,
    /// All surface allomorphs, longest first.
    pub forms:   &'static [&'static str],
    /// Minimum syllable count the stem must have after stripping.
    pub min_stem_syllables: usize,
}

/// Built-in suffix table, covering the most frequent Turkish inflectional suffixes.
///
/// Ordering: longer / more specific first within each conceptual group.
/// The FSM walker tries each suffix and keeps the longest match that satisfies
/// harmony and minimum-syllable constraints.
pub static SUFFIX_TABLE: &[SuffixDef] = &[
    // ── Plural ────────────────────────────────────────────────────────────────
    SuffixDef {
        label: "Plural",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["ler", "lar"],
        min_stem_syllables: 1,
    },

    // ── Possessive (3sg) ──────────────────────────────────────────────────────
    SuffixDef {
        label: "Poss3sg",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::S, // vowel-final stems: araba+s+ı
        forms: &["si", "sı", "su", "sü", "i", "ı", "u", "ü"],
        min_stem_syllables: 1,
    },

    // ── Possessive (1sg) ──────────────────────────────────────────────────────
    SuffixDef {
        label: "Poss1sg",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["im", "ım", "um", "üm"],
        min_stem_syllables: 1,
    },

    // ── Possessive (2sg) ──────────────────────────────────────────────────────
    SuffixDef {
        label: "Poss2sg",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["in", "ın", "un", "ün"],
        min_stem_syllables: 1,
    },

    // ── Accusative ────────────────────────────────────────────────────────────
    SuffixDef {
        label: "Acc",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::Y, // vowel-final stems: araba+y+ı
        forms: &["yi", "yı", "yu", "yü", "i", "ı", "u", "ü"],
        min_stem_syllables: 1,
    },

    // ── Dative ────────────────────────────────────────────────────────────────
    SuffixDef {
        label: "Dat",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::Y,
        forms: &["ye", "ya", "e", "a"],
        min_stem_syllables: 1,
    },

    // ── Locative ──────────────────────────────────────────────────────────────
    SuffixDef {
        label: "Loc",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["de", "da", "te", "ta"],
        min_stem_syllables: 1,
    },

    // ── Ablative ──────────────────────────────────────────────────────────────
    SuffixDef {
        label: "Abl",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["den", "dan", "ten", "tan"],
        min_stem_syllables: 1,
    },

    // ── Genitive ──────────────────────────────────────────────────────────────
    SuffixDef {
        label: "Gen",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["nin", "nın", "nun", "nün", "in", "ın", "un", "ün"],
        min_stem_syllables: 1,
    },

    // ── Instrumental / Comitative ─────────────────────────────────────────────
    SuffixDef {
        label: "Ins",
        class: SuffixClass::NounInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["yle", "yla", "le", "la"],
        min_stem_syllables: 1,
    },

    // ── Nominal-verb copular (present) ────────────────────────────────────────
    SuffixDef {
        label: "NomVerb1sg",
        class: SuffixClass::NominalVerb,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["yim", "yım", "yum", "yüm", "im", "ım", "um", "üm"],
        min_stem_syllables: 1,
    },
    SuffixDef {
        label: "NomVerb2sg",
        class: SuffixClass::NominalVerb,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["sin", "sın", "sun", "sün"],
        min_stem_syllables: 1,
    },
    SuffixDef {
        label: "NomVerb1pl",
        class: SuffixClass::NominalVerb,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["yiz", "yız", "yuz", "yüz", "iz", "ız", "uz", "üz"],
        min_stem_syllables: 1,
    },

    // ── Derivational: noun→noun diminutive ───────────────────────────────────
    SuffixDef {
        label: "Dim",
        class: SuffixClass::Derivational,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["cik", "cık", "cuk", "cük", "çik", "çık", "çuk", "çük"],
        min_stem_syllables: 1,
    },

    // ── Derivational: noun→adjective -li ─────────────────────────────────────
    SuffixDef {
        label: "With",
        class: SuffixClass::Derivational,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["li", "lı", "lu", "lü"],
        min_stem_syllables: 1,
    },

    // ── Derivational: noun→adjective -siz ────────────────────────────────────
    SuffixDef {
        label: "Without",
        class: SuffixClass::Derivational,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["siz", "sız", "suz", "süz"],
        min_stem_syllables: 1,
    },

    // ── Derivational: noun→noun -lik ─────────────────────────────────────────
    SuffixDef {
        label: "Ness",
        class: SuffixClass::Derivational,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["lik", "lık", "luk", "lük"],
        min_stem_syllables: 1,
    },

    // ── Verb: progressive -iyor ──────────────────────────────────────────────
    SuffixDef {
        label: "Prog",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::Any,
        buffer: Buffer::None,
        forms: &["iyor", "ıyor", "uyor", "üyor"],
        min_stem_syllables: 1,
    },

    // ── Verb: definite past -dı ───────────────────────────────────────────────
    SuffixDef {
        label: "PastDef",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["di", "dı", "du", "dü", "ti", "tı", "tu", "tü"],
        min_stem_syllables: 1,
    },

    // ── Verb: evidential past -miş ────────────────────────────────────────────
    SuffixDef {
        label: "PastEv",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["miş", "mış", "muş", "müş"],
        min_stem_syllables: 1,
    },

    // ── Verb: aorist (positive) ───────────────────────────────────────────────
    SuffixDef {
        label: "AorPos",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["er", "ar", "ir", "ır", "ur", "ür"],
        min_stem_syllables: 1,
    },

    // ── Verb: aorist negative -maz ────────────────────────────────────────────
    SuffixDef {
        label: "AorNeg",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["mez", "maz"],
        min_stem_syllables: 1,
    },

    // ── Verb: future -acak ────────────────────────────────────────────────────
    SuffixDef {
        label: "Fut",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["acak", "ecek"],
        min_stem_syllables: 1,
    },

    // ── Verb: conditional -sa ─────────────────────────────────────────────────
    SuffixDef {
        label: "Cond",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["sa", "se"],
        min_stem_syllables: 1,
    },

    // ── Verb: necessitative -malı ────────────────────────────────────────────
    SuffixDef {
        label: "Nec",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["malı", "meli"],
        min_stem_syllables: 1,
    },

    // ── Verb: infinitive -mek ────────────────────────────────────────────────
    SuffixDef {
        label: "Inf",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["mek", "mak"],
        min_stem_syllables: 1,
    },

    // ── Verb: negation -me ───────────────────────────────────────────────────
    SuffixDef {
        label: "Neg",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["me", "ma"],
        min_stem_syllables: 1,
    },

    // ── Verb: gerund -arak ────────────────────────────────────────────────────
    SuffixDef {
        label: "GerDiv",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["arak", "erek"],
        min_stem_syllables: 1,
    },

    // ── Verb: gerund -ınca ────────────────────────────────────────────────────
    SuffixDef {
        label: "GerWhen",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["ınca", "ince", "unca", "ünce"],
        min_stem_syllables: 1,
    },

    // ── Verb: gerund -ırken ───────────────────────────────────────────────────
    SuffixDef {
        label: "GerWhile",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["ırken", "irken", "urken", "ürken", "arken", "erken"],
        min_stem_syllables: 1,
    },

    // ── Verb person endings ───────────────────────────────────────────────────
    SuffixDef {
        label: "P1sg",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["yım", "yim", "yum", "yüm", "m"],
        min_stem_syllables: 1,
    },
    SuffixDef {
        label: "P2sg",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["sin", "sın", "sun", "sün", "n"],
        min_stem_syllables: 1,
    },
    SuffixDef {
        label: "P1pl",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["yız", "yiz", "yuz", "yüz", "k"],
        min_stem_syllables: 1,
    },
    SuffixDef {
        label: "P2pl",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["siniz", "sınız", "sunuz", "sünüz"],
        min_stem_syllables: 1,
    },
    SuffixDef {
        label: "P3pl",
        class: SuffixClass::VerbInflection,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["ler", "lar"],
        min_stem_syllables: 1,
    },

    // ── Verb derivational: causative -dır ────────────────────────────────────
    SuffixDef {
        label: "Caus",
        class: SuffixClass::Derivational,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["dır", "dir", "dur", "dür", "tır", "tir", "tur", "tür"],
        min_stem_syllables: 1,
    },

    // ── Verb derivational: passive -il ───────────────────────────────────────
    SuffixDef {
        label: "Pass",
        class: SuffixClass::Derivational,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["ıl", "il", "ul", "ül"],
        min_stem_syllables: 1,
    },

    // ── Verb derivational: reflexive -in ─────────────────────────────────────
    SuffixDef {
        label: "Refl",
        class: SuffixClass::Derivational,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["ın", "in", "un", "ün"],
        min_stem_syllables: 1,
    },

    // ── Verb derivational: reciprocal -iş ────────────────────────────────────
    SuffixDef {
        label: "Recip",
        class: SuffixClass::Derivational,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["ış", "iş", "uş", "üş"],
        min_stem_syllables: 1,
    },

    // ── Verb: agent noun -ıcı ─────────────────────────────────────────────────
    SuffixDef {
        label: "VerbAgent",
        class: SuffixClass::Derivational,
        harmony: HarmonyReq::MatchBackness,
        buffer: Buffer::None,
        forms: &["ıcı", "ici", "ucu", "ücü"],
        min_stem_syllables: 1,
    },
];

/// Try to strip one suffix layer from `word`.
///
/// Returns `Some((candidate_stem, suffix_label))` for the longest matching
/// suffix that satisfies harmony, or `None` if no suffix applies.
pub fn strip_one(word: &str, aggressive: bool) -> Option<(String, &'static str)> {
    let mut best: Option<(String, &'static str, usize)> = None; // (stem, label, form_len)

    'outer: for def in SUFFIX_TABLE {
        for &form in def.forms {
            if !word.ends_with(form) {
                continue;
            }

            let stem_end = word.len() - form.len();
            let candidate = &word[..stem_end];

            if candidate.is_empty() {
                continue;
            }

            let stem_syllables = crate::harmony::syllable_count(candidate);
            if stem_syllables < def.min_stem_syllables {
                continue;
            }

            // Vowel harmony check: compare last vowel of candidate vs first vowel of suffix.
            let stem_last_v = last_vowel(candidate);
            let suf_first_v = form.chars().find(|&c| crate::harmony::is_vowel(c));
            if let Some(sv) = suf_first_v {
                let suf_features = vowel_features(sv);
                if !crate::harmony::harmony_ok(stem_last_v, suf_features, def.harmony) {
                    // Voicing-assimilated locative/ablative: te/ta/de/da
                    // are handled via multiple allomorphs in the table already.
                    continue;
                }
            }

            let form_len = form.len();
            let is_better = best.as_ref().map_or(true, |(_, _, len)| form_len > *len);
            if is_better {
                best = Some((candidate.to_owned(), def.label, form_len));
            }

            if !aggressive {
                // In conservative mode, stop after we find the first (longest) match.
                break 'outer;
            }
        }
    }

    best.map(|(stem, label, _)| (stem, label))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_plural_lar() {
        let (stem, label) = strip_one("kitaplar", false).unwrap();
        assert_eq!(stem, "kitap");
        assert_eq!(label, "Plural");
    }

    #[test]
    fn strip_plural_ler() {
        let (stem, label) = strip_one("evler", false).unwrap();
        assert_eq!(stem, "ev");
        assert_eq!(label, "Plural");
    }

    #[test]
    fn harmony_guard_blocks_wrong_allomorph() {
        // "kitap" + "-ler" would fail harmony (back stem needs -lar).
        // strip_one should return the correct allomorph or nothing.
        let result = strip_one("kitaplar", false);
        assert!(result.is_some());
        // "kitapler" is not real Turkish but let's ensure no false match from -ler:
        // "kitapler" ends with "ler" but "kitap" is a back-vowel stem → harmony fails.
        let bad = strip_one("kitapler", false);
        // Could still strip -er (aorist) — just verify no "Plural" from -ler
        if let Some((_, label)) = bad {
            assert_ne!(label, "Plural");
        }
    }

    #[test]
    fn strip_locative() {
        let (stem, label) = strip_one("evde", false).unwrap();
        assert_eq!(stem, "ev");
        assert_eq!(label, "Loc");
    }

    #[test]
    fn strip_ablative() {
        let (stem, label) = strip_one("evden", false).unwrap();
        assert_eq!(stem, "ev");
        assert_eq!(label, "Abl");
    }

    #[test]
    fn no_strip_on_single_vowel_word() {
        let result = strip_one("el", false);
        if let Some((stem, _)) = result {
            assert!(!stem.is_empty());
            assert!(crate::harmony::syllable_count(&stem) >= 1);
        }
    }

    // ── Verb tense tests ──────────────────────────────────────────────────────

    #[test]
    fn strip_progressive() {
        let (stem, label) = strip_one("gidiyor", false).unwrap();
        assert_eq!(stem, "gid");
        assert_eq!(label, "Prog");
    }

    #[test]
    fn strip_past_definite() {
        let (stem, label) = strip_one("gitti", false).unwrap();
        // "gitti" ends with "ti" (devoiced -di after t-final stem)
        assert_eq!(label, "PastDef");
        assert!(!stem.is_empty());
    }

    #[test]
    fn strip_evidential_past() {
        let (stem, label) = strip_one("gelmiş", false).unwrap();
        assert_eq!(stem, "gel");
        assert_eq!(label, "PastEv");
    }

    #[test]
    fn strip_future() {
        let (stem, label) = strip_one("gelecek", false).unwrap();
        assert_eq!(stem, "gel");
        assert_eq!(label, "Fut");
    }

    #[test]
    fn strip_infinitive() {
        let (stem, label) = strip_one("gelmek", false).unwrap();
        assert_eq!(stem, "gel");
        assert_eq!(label, "Inf");
    }

    #[test]
    fn strip_necessitative() {
        let (stem, label) = strip_one("gelmeli", false).unwrap();
        assert_eq!(stem, "gel");
        assert_eq!(label, "Nec");
    }

    #[test]
    fn strip_gerund_arak() {
        let (stem, label) = strip_one("gelerek", false).unwrap();
        assert_eq!(stem, "gel");
        assert_eq!(label, "GerDiv");
    }

    #[test]
    fn strip_causative() {
        let (stem, label) = strip_one("yaptır", false).unwrap();
        assert_eq!(stem, "yap");
        assert_eq!(label, "Caus");
    }

    #[test]
    fn strip_passive() {
        let (stem, label) = strip_one("yapıl", false).unwrap();
        assert_eq!(stem, "yap");
        assert_eq!(label, "Pass");
    }

    #[test]
    fn strip_agent_noun() {
        let (stem, label) = strip_one("yazıcı", false).unwrap();
        assert_eq!(stem, "yaz");
        assert_eq!(label, "VerbAgent");
    }

    #[test]
    fn strip_p2pl() {
        let (stem, label) = strip_one("geliyorsunuz", false).unwrap();
        assert_eq!(label, "P2pl");
        assert!(!stem.is_empty());
    }
}
