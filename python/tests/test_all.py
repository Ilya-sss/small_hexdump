import pytest
import small_hexdump


def test_sum_as_string():
    assert small_hexdump.sum_as_string(1, 1) == "2"
