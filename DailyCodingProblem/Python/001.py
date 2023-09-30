import unittest

"""
Difficulty: Easy

This problem was recently asked by Google.

Given a list of numbers and a number k, return whether any two numbers
from the list add up to k.

For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7
is 17.

Bonus: Can you do this in one pass?
"""

"""
Time Complexity:  O(n^2)
Space Complexity: O(1)
"""
def test_sum_original(lst, k):
    for count, value in enumerate(lst):
        b = k - value
        if b in lst[count+1:]:
            return True
    return False

"""
Time Complexity:  O(N)
Space Complexity: O(N)
"""
def test_sum(lst, k):
    prev = set()
    for value in lst:
        b = k - value
        if b in prev:
            return True
        prev.add(value)
    return False

class TestProblem001(unittest.TestCase):
    def testTrueCase(self):
        self.assertEqual(True, test_sum([10, 15, 3, 7], 17))

    def testSimpleFalseCase(self):
        self.assertEqual(False, test_sum([10, 15, 3, 7], 16))

    def testEmptyList(self):
        self.assertEqual(False, test_sum([], 1))

if __name__ == "__main__":
    unittest.main()