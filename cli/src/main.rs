use oz_core::{Stemmer, StemmerConfig};
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let aggressive = args.iter().any(|a| a == "--aggressive");
    let json       = args.iter().any(|a| a == "--json");

    let config = StemmerConfig { aggressive, ..Default::default() };
    let stemmer = Stemmer::new(config);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read stdin");
        let word = line.trim();
        if word.is_empty() {
            continue;
        }

        if json {
            let analysis = stemmer.analyze(word);
            // Minimal JSON without pulling in serde_json as a dep yet.
            println!(
                r#"{{"input":"{input}","stem":"{stem}","confidence":{conf:.3}}}"#,
                input = analysis.input,
                stem  = analysis.stem,
                conf  = analysis.confidence,
            );
        } else {
            println!("{}", stemmer.stem(word));
        }
    }
}
