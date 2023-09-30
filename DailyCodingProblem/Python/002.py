"""
Difficulty: Medium

This problem was asked by Uber.

Given an array of integers, return a new array such that each element at
index i of the new array is the product of all the numbers in the
original array except the one at i.

For example, if our input was [1, 2, 3, 4, 5], the expected output would
be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected
output would be [2, 3, 6].

Follow-up: what if you can't use division?
"""

import unittest

"""
Time Complexity: O(N)
Space Complexity: O(N)
"""
def product_with_division(data):
    # Calculate product of all of the members.
    prod = 1
    for val in data:
        prod *= val
    result = []
    for val in data:
        result.append(prod / val)
    return result

"""
Time Complexity: O(N^2)
Space Complexity: O(N)
"""
def product(data):
    # Calculate product of all of the members.
    result = [1 for _ in range(len(data))]
    for i, val in enumerate(data):
        for j in range(i):
            result[i] *= data[j]
        for j in range(i+1, len(data)):
            result[i] *= data[j]
    return result

class TestProblem002(unittest.TestCase):
    def testCase1(self):
        input = [1, 2, 3, 4, 5]
        expected = [120, 60, 40, 30, 24]
        self.assertEqual(expected, product_with_division(input))
        self.assertEqual(expected, product(input))

    def testCase2(self):
        input = [3, 2, 1]
        expected = [2, 3, 6]
        self.assertEqual(expected, product_with_division(input))
        self.assertEqual(expected, product(input))

    def testEmptyList(self):
        input = []
        expected = []
        self.assertEqual(expected, product_with_division(input))
        self.assertEqual(expected, product(input))

if __name__ == "__main__":
    unittest.main()