# This problem was asked by Pivotal.
#
# Write an algorithm that finds the total number of set bits in all integers between 1 and N.


def count_set_bits(n):
    count = 0
    for i in range(1, n + 1):
        count += i.bit_count()
    return count


class TestCountSetBitS:
    def test_empty(self):
        assert count_set_bits(0) == 0

    def test_one(self):
        assert count_set_bits(1) == 1

    def test_two(self):
        assert count_set_bits(2) == 2

    def test_ten(self):
        assert count_set_bits(10) == 22
