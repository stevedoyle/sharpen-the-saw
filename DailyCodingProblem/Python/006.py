"""
This problem was asked by Google.

An XOR linked list is a more memory efficient doubly linked list.
Instead of each node holding next and prev fields, it holds a field
named both, which is an XOR of the next node and the previous node.
Implement an XOR linked list; it has an add(element) which adds the
element to the end, and a get(index) which returns the node at index.

If using a language that has no pointers (such as Python), you can
assume you have access to get_pointer and dereference_pointer functions
that converts between nodes and memory addresses.
"""

class Node:
    def __init__(self, val):
        self.val = val
        self.both = None

class XorList:
    def __init__(self):
        self.root = None

    def add(self, element):
        pass

    def get(self, index):
        pass

import unittest

class TestXorList(unittest.TestCase):
    def testXorList(self):
        pass


if __name__ == "__main__":
    unittest.main()