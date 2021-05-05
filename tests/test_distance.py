import pytest

from pyrustbio import distance

def test_levenshtein():
    assert distance.levenshtein("asdf", "asdf") == 0
    assert distance.levenshtein("asdf", "qwer") == 4
    assert distance.levenshtein("asdfg", "zxcv") == 5
