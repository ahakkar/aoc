# -*- encoding: utf-8 -*-
'''
@File    :   test_05.py
@Time    :   08/11/2022, 18:37:41
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2021
@Desc    :   Does not work because I have no idea what I'm doing.
'''

import pathlib
import sys
import pytest
import aoc05a

# current file directory
PUZZLE_DIR = pathlib.Path(__file__).parent

@pytest.fixture
def simple_input():
    simple_input = (PUZZLE_DIR / "simple.input").read_text().strip()
    return simple_input

@pytest.fixture
def simple_test():
    simple_test = (PUZZLE_DIR / "simple.test").read_text().strip()
    return simple_test
    
def test_print_lines(capsys, simple_input) -> None:
    """Test that input is parsed properly"""
    test = aoc05a.Lines("simple.input")
    test.get_lines()
    cap = capsys.readouterr()
    assert simple_input == cap.out


