import unittest

from max_pairwise_product import max_pairwise_product


# Add tests to test_max_pairwise_product function
class TestMaxPairwiseProduct(unittest.TestCase):
    def test_max_pairwise_product(self):
        assert max_pairwise_product([1, 2, 3]) == 6
        assert max_pairwise_product([9, 2, 3]) == 27
        assert max_pairwise_product([1, 5, 3, 4, 2]) == 20
        assert max_pairwise_product([1, 5, 3, 4, 5, 5]) == 25
        assert max_pairwise_product([1, 2, 3, 4, 5, 6, 7]) == 42
        assert max_pairwise_product([1, 2, 3, 4, 5, 6, 7, 8]) == 56
        assert max_pairwise_product([1, 2, 3, 4, 5, 6, 7, 8, 9]) == 72
        assert max_pairwise_product([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) == 90
        assert max_pairwise_product([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]) == 110
        assert max_pairwise_product([100000, 90000]) == 9000000000
