//! `oz` — command-line interface for the Öz Turkish stemmer.
//!
//! Reads words from positional arguments or stdin (newline-separated by default,
//! whitespace-separated with `--batch`) and prints stems. Optional JSON output
//! and confidence scores.

use clap::Parser;
use oz_core::{Stemmer, StemmerConfig};
use serde::Serialize;
use std::io::{self, BufRead, Write};

/// Öz — Turkish stemmer.
#[derive(Parser, Debug)]
#[command(name = "oz", version, about, long_about = None)]
struct Cli {
    /// Words to stem. If none are given, read from stdin.
    #[arg(value_name = "WORD")]
    words: Vec<String>,

    /// Strip as many suffix layers as possible (better for search / IR).
    #[arg(long)]
    aggressive: bool,

    /// Emit one JSON object per input word instead of plain text.
    #[arg(long)]
    json: bool,

    /// Include the confidence score in plain-text output ("<stem>\t<score>").
    /// No effect when `--json` is set (JSON always includes confidence).
    #[arg(long)]
    confidence: bool,

    /// Emit the full morphological analysis (suffixes, harmony, …).
    /// Forces JSON output.
    #[arg(long)]
    analyze: bool,

    /// Split each stdin line on whitespace instead of treating it as one word.
    #[arg(long)]
    batch: bool,
}

#[derive(Serialize)]
struct StemOut<'a> {
    input:      &'a str,
    stem:       String,
    confidence: f32,
}

#[derive(Serialize)]
struct Suffix {
    surface: String,
    label:   String,
}

#[derive(Serialize)]
struct AnalyzeOut {
    input:            String,
    stem:             String,
    suffixes:         Vec<Suffix>,
    vowel_harmony_ok: bool,
    dict_validated:   bool,
    confidence:       f32,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let config = StemmerConfig { aggressive: cli.aggressive, ..Default::default() };
    let stemmer = Stemmer::new(config);

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut emit = |word: &str| -> io::Result<()> {
        let word = word.trim();
        if word.is_empty() {
            return Ok(());
        }
        if cli.analyze {
            let a = stemmer.analyze(word);
            let payload = AnalyzeOut {
                input:            a.input,
                stem:             a.stem,
                suffixes:         a.suffixes.into_iter()
                    .map(|s| Suffix { surface: s.surface, label: s.label })
                    .collect(),
                vowel_harmony_ok: a.vowel_harmony_ok,
                dict_validated:   a.dict_validated,
                confidence:       a.confidence,
            };
            writeln!(out, "{}", serde_json::to_string(&payload).unwrap())?;
        } else if cli.json {
            let (stem, confidence) = stemmer.stem_with_confidence(word);
            let payload = StemOut { input: word, stem, confidence };
            writeln!(out, "{}", serde_json::to_string(&payload).unwrap())?;
        } else if cli.confidence {
            let (stem, confidence) = stemmer.stem_with_confidence(word);
            writeln!(out, "{}\t{:.3}", stem, confidence)?;
        } else {
            writeln!(out, "{}", stemmer.stem(word))?;
        }
        Ok(())
    };

    if !cli.words.is_empty() {
        for w in &cli.words {
            emit(w)?;
        }
        return Ok(());
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if cli.batch {
            for token in line.split_whitespace() {
                emit(token)?;
            }
        } else {
            emit(&line)?;
        }
    }
    Ok(())
}
