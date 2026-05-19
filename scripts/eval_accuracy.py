#!/usr/bin/env python3
"""Evaluate Öz stemmer accuracy against the ground-truth corpus."""

import argparse
import subprocess
import sys


def load_corpus(path: str) -> list[tuple[str, str]]:
    pairs = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split("\t")
            if len(parts) >= 2:
                pairs.append((parts[0], parts[1]))
    return pairs


def stem_words(stemmer_bin: str, words: list[str]) -> list[str]:
    input_text = "\n".join(words)
    result = subprocess.run(
        [stemmer_bin],
        input=input_text,
        capture_output=True,
        text=True,
        check=True,
        encoding="utf-8",
    )
    return result.stdout.strip().splitlines()


def main() -> None:
    parser = argparse.ArgumentParser(description="Evaluate Öz stemmer accuracy")
    parser.add_argument("--corpus",   required=True, help="Path to ground-truth TSV")
    parser.add_argument("--stemmer",  required=True, help="Path to oz CLI binary")
    parser.add_argument("--threshold", type=float, default=0.94,
                        help="Minimum required accuracy (default: 0.94)")
    args = parser.parse_args()

    corpus = load_corpus(args.corpus)
    if not corpus:
        print("ERROR: corpus is empty", file=sys.stderr)
        sys.exit(1)

    words   = [w for w, _ in corpus]
    stems   = stem_words(args.stemmer, words)
    expected = [s for _, s in corpus]

    correct = sum(s == e for s, e in zip(stems, expected))
    total   = len(corpus)
    accuracy = correct / total

    print(f"Accuracy: {correct}/{total} = {accuracy:.4f} ({accuracy*100:.2f}%)")

    if accuracy < args.threshold:
        print(
            f"FAIL: accuracy {accuracy:.4f} < threshold {args.threshold}",
            file=sys.stderr,
        )
        sys.exit(1)

    print(f"PASS: accuracy ≥ {args.threshold}")


if __name__ == "__main__":
    main()
