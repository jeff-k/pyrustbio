import pytest

from pyrustbio import levenshtein_py


def test_levenshtein():
    assert levenshtein_py("asdf", "asdg") == 1
