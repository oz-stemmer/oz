use crate::harmony::syllable_count;

/// Apply the Turkish final-consonant voicing alternation to the last character
/// of `stem`, returning the mutated form.
///
/// Rules (applied after suffix stripping):
///   p → b,  ç → c,  t → d,  k → ğ  (polysyllabic) / g (monosyllabic)
///
/// The reverse direction (b→p, etc.) is handled during suffix matching when the
/// FSM detects a voicing-assimilation allomorph.
pub fn apply_final_mutation(stem: &str) -> String {
    if stem.is_empty() {
        return stem.to_owned();
    }

    let syllables = syllable_count(stem);

    // Find the byte offset of the last char so we can replace it.
    let last_char_start = stem
        .char_indices()
        .last()
        .map(|(i, _)| i)
        .unwrap_or(0);
    let last_char = stem.chars().last().unwrap();

    let replacement: Option<&str> = match last_char {
        'p' => Some("b"),
        'ç' => Some("c"),
        't' => Some("d"),
        // k → ğ only in polysyllabic stems; monosyllabic keeps k (e.g. "tek")
        'k' if syllables >= 2 => Some("ğ"),
        _ => None,
    };

    match replacement {
        Some(r) => format!("{}{}", &stem[..last_char_start], r),
        None    => stem.to_owned(),
    }
}

/// Reverse: given a surface consonant in the suffixed form, recover the
/// citation-form final consonant (used by the FSM when it sees a voiced
/// initial consonant on the suffix).
pub fn devoice_final(c: char) -> Option<char> {
    match c {
        'b' => Some('p'),
        'c' => Some('ç'),
        'd' => Some('t'),
        'ğ' => Some('k'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_voicing() {
        assert_eq!(apply_final_mutation("kitap"), "kitab");
        assert_eq!(apply_final_mutation("ağaç"),  "ağac");
        assert_eq!(apply_final_mutation("kanat"),  "kanad");
    }

    #[test]
    fn k_polysyllabic() {
        assert_eq!(apply_final_mutation("ayak"), "ayağ");
        assert_eq!(apply_final_mutation("çocuk"), "çocuğ");
    }

    #[test]
    fn k_monosyllabic_unchanged() {
        // "tek", "ak", "ok" — single syllable: k stays
        assert_eq!(apply_final_mutation("tek"), "tek");
        assert_eq!(apply_final_mutation("ok"),  "ok");
    }

    #[test]
    fn no_mutation_for_other_consonants() {
        assert_eq!(apply_final_mutation("ev"),   "ev");
        assert_eq!(apply_final_mutation("araba"),"araba");
    }
}
