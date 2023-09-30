"""
Difficulty: Hard

## The Problem

This problem was asked by Stripe.

Given an array of integers, find the first missing positive integer in
linear time and constant space. In other words, find the lowest positive
integer that does not exist in the array. The array can contain
duplicates and negative numbers as well.

For example, the input [3, 4, -1, 1] should give 2. The input [1, 2, 0]
should give 3.

You can modify the input array in-place.
"""

import unittest

"""
Naive approach.
Time complexity: O(n^2)
Space complexity: O(1)
"""
def find_first_missing_int_naive(data) -> int:
    for i in range(1, len(data)+1):
        found = False
        for val in data:
            if i == val:
                found = True
                break
        if not found:
            return i
    return len(data)+1

"""
Build a tracker to track the positive integers
Time complexity: O(n) ... assuming set operations are O(1)
Space complexity: O(n)
"""
def find_first_missing_int_tracker(data) -> int:
    tracker = set(data)

    # Find the missing value
    for i in range(1, len(data)+1):
        if i not in tracker:
            return i
    return len(data)+1

"""
Place the positive integers in the array at the index given by their
value, i.e. data[idx] = idx
Time complexity: O(n)
Space complexity: O(1)
"""
def find_first_missing_int_index(data) -> int:
    n = len(data)

    for i in  range(len(data)):
        want = i+1
        while data[i] > 0:
            if data[i] == want: break
            if data[i] <= 0 or data[i] > n:
                data[i] = -1
                break
            j = data[i]
            data[j-1], data[i] = data[i], data[j-1]

    # Find the missing value
    for i in range(1, len(data)):
        if data[i-1] != i:
            return i

    return len(data)+1

class TestFindFirstMissingInt(unittest.TestCase):
    def testEarlyGap(self):
        input = [3, 4, -1, 1]
        want = 2
        self.assertEqual(want, find_first_missing_int_naive(input))
        self.assertEqual(want, find_first_missing_int_tracker(input))
        self.assertEqual(want, find_first_missing_int_index(input))

    def testNoGap(self):
        input = [1, 2, 3, 4, 5, 6]
        want = 7
        self.assertEqual(want, find_first_missing_int_naive(input))
        self.assertEqual(want, find_first_missing_int_tracker(input))
        self.assertEqual(want, find_first_missing_int_index(input))

    def testEmptyInput(self):
        input = []
        want = 1
        self.assertEqual(want, find_first_missing_int_naive(input))
        self.assertEqual(want, find_first_missing_int_tracker(input))
        self.assertEqual(want, find_first_missing_int_index(input))

if __name__ == "__main__":
    unittest.main()
