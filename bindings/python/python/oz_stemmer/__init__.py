"""Öz — Modern Turkish stemmer, Python bindings.

Example:
    >>> from oz_stemmer import Stemmer
    >>> s = Stemmer()
    >>> s.stem("kitaplardan")
    'kitap'
    >>> s.stem_batch(["evler", "masada"])
    ['ev', 'masa']
"""

from ._oz_stemmer import Stemmer, StemmerConfig, __version__

__all__ = ["Stemmer", "StemmerConfig", "__version__"]
