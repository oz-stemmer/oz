//! Integration tests for the `oz` CLI.

use assert_cmd::Command;
use predicates::prelude::*;

fn oz() -> Command {
    Command::cargo_bin("oz").expect("oz binary built by cargo")
}

#[test]
fn positional_args_plain_output() {
    oz()
        .args(["evler", "arabalar"])
        .assert()
        .success()
        .stdout("ev\naraba\n");
}

#[test]
fn stdin_plain_output() {
    oz()
        .write_stdin("evler\narabalar\n")
        .assert()
        .success()
        .stdout("ev\naraba\n");
}

#[test]
fn batch_splits_on_whitespace() {
    // Note: `masada` is intentionally avoided — oz-core currently applies
    // final consonant mutation too eagerly on locative strip and returns
    // `masad` instead of `masa`. Pre-existing stemmer bug, out of scope
    // for this CLI PR.
    oz()
        .arg("--batch")
        .write_stdin("evler arabalar   masalar\n")
        .assert()
        .success()
        .stdout("ev\naraba\nmasa\n");
}

#[test]
fn confidence_flag_emits_tab_separated_score() {
    oz()
        .args(["--confidence", "evler"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^ev\t\d+\.\d{3}\n$").unwrap());
}

#[test]
fn json_flag_emits_one_object_per_word() {
    oz()
        .args(["--json", "evler", "arabalar"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(r#"^\{"input":"evler","stem":"ev","confidence":[0-9.]+\}\n\{"input":"arabalar","stem":"araba","confidence":[0-9.]+\}\n$"#).unwrap(),
        );
}

#[test]
fn analyze_flag_emits_full_structure() {
    oz()
        .args(["--analyze", "evler"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"suffixes\""))
        .stdout(predicate::str::contains("\"vowel_harmony_ok\""))
        .stdout(predicate::str::contains("\"stem\":\"ev\""));
}

#[test]
fn empty_lines_are_skipped() {
    oz()
        .write_stdin("evler\n\n\narabalar\n")
        .assert()
        .success()
        .stdout("ev\naraba\n");
}

#[test]
fn version_flag_works() {
    oz()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("oz "));
}

#[test]
fn help_flag_lists_main_options() {
    oz()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--aggressive"))
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("--batch"));
}
