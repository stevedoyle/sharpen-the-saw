"""
This problem was asked by Google.

Given two singly linked lists that intersect at some point, find the
intersecting node. The lists are non-cyclical.

For example, given A = 3 -> 7 -> 8 -> 10 and B = 99 -> 1 -> 8 -> 10,
return the node with value 8.

In this example, assume nodes with the same value are the exact same
node objects.

Do this in O(M + N) time (where M and N are the lengths of the lists)
and constant space.
"""

import unittest


def list_intersection(a, b):
    minlen = min(len(a), len(b))
    if len(a) < len(b):
        b = b[len(b) - minlen :]
    else:
        a = a[len(a) - minlen :]

    for i in range(minlen):
        if a[i] == b[i]:
            return a[i]
    return None


class Test020ListIntersect(unittest.TestCase):
    def testIntersectingLists(self):
        a = [3, 7, 8, 10]
        b = [99, 1, 8, 10]
        want = 8
        self.assertEqual(want, list_intersection(a, b))

    def testNonIntersectingLists(self):
        a = [3, 7, 8, 10]
        b = [99, 98, 97, 96]
        want = None
        self.assertEqual(want, list_intersection(a, b))

    def testIntersectingListsDifferentLengths(self):
        a = [3, 7, 8, 10]
        b = [99, 98, 97, 96, 8, 10]
        want = 8
        self.assertEqual(want, list_intersection(a, b))


if __name__ == "__main__":
    unittest.main()
