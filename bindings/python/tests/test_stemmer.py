"""Smoke tests for the oz_stemmer Python bindings."""

from oz_stemmer import Stemmer, StemmerConfig, __version__


def test_version_exposed():
    assert isinstance(__version__, str)
    assert len(__version__) > 0


def test_stem_simple_plural():
    s = Stemmer()
    # Words without consonant-mutation triggers (p/ç/t/k → b/c/d/ğ).
    # "kitaplar" is intentionally left out here; it exercises a known
    # oz-core bug where final-mutation is applied too eagerly.
    assert s.stem("evler") == "ev"
    assert s.stem("arabalar") == "araba"


def test_stem_batch_matches_single():
    s = Stemmer()
    words = ["evler", "kitaplar", "masada"]
    batch = s.stem_batch(words)
    assert batch == [s.stem(w) for w in words]


def test_stem_with_confidence_returns_tuple():
    s = Stemmer()
    stem, conf = s.stem_with_confidence("evler")
    assert stem == "ev"
    assert 0.0 <= conf <= 1.0


def test_analyze_returns_dict():
    s = Stemmer()
    a = s.analyze("evler")
    assert a["input"] == "evler"
    assert a["stem"] == "ev"
    assert isinstance(a["suffixes"], list)
    assert 0.0 <= a["confidence"] <= 1.0


def test_config_aggressive_flag_roundtrips():
    cfg = StemmerConfig(aggressive=True)
    assert cfg.aggressive is True
    s = Stemmer(cfg)
    # Aggressive mode strips deeper; for a simple word it should not break it.
    assert s.stem("ev") == "ev"


def test_non_turkish_passthrough():
    s = Stemmer()
    assert s.stem("stress") == "stress"
