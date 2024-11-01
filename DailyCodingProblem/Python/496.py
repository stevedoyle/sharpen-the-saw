# This problem was asked by Pivotal.
#
# Write an algorithm that finds the total number of set bits in all integers between 1 and N.

from functools import reduce


def count_set_bits(n):
    count = 0
    for i in range(1, n + 1):
        count += i.bit_count()
    return count


def count_set_bits2(n):
    return reduce(lambda acc, x: acc + x.bit_count(), range(1, n + 1), 0)


class TestCountSetBitS:
    def test_empty(self):
        assert count_set_bits(0) == 0
        assert count_set_bits2(0) == 0

    def test_one(self):
        assert count_set_bits(1) == 1
        assert count_set_bits2(1) == 1

    def test_two(self):
        assert count_set_bits(2) == 2
        assert count_set_bits2(2) == 2

    def test_ten(self):
        assert count_set_bits(10) == 17
        assert count_set_bits2(10) == 17
