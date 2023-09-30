"""
Diffuculty: Hard

## The Problem

This problem was asked by Google.

Given the root to a binary tree, implement serialize(root), which
serializes the tree into a string, and deserialize(s), which
deserializes the string back into the tree.

For example, given the following Node class

```python
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
```

The following test should pass:

```python
node = Node('root', Node('left', Node('left.left')), Node('right'))
assert deserialize(serialize(node)).left.left.val == 'left.left'
```
"""

import unittest

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def serialize(node):
    # Using a ',' as a node seperator in the serialized string
    # An empty string (i.e. ',,') denotes an empty node in the tree.
    # Following a convention of left branch traversal first.
    str = ','
    if node:
        str = node.val + ','
        str += serialize(node.left)
        str += serialize(node.right)
    return str

def deserialize(str):
    elems = str.split(',')
    eliter = iter(elems)
    return deserializePreorder(eliter)

def deserializePreorder(eliter):
    try:
        name = next(eliter)
        if not name:
            return None
        node = Node(name)
        node.left = deserializePreorder(eliter)
        node.right = deserializePreorder(eliter)
        return node
    except StopIteration:
        return None


class TestSerialize(unittest.TestCase):
    def testSerialize(self):
        node = Node('root', Node('left', Node('left.left')), Node('right'))
        self.assertEqual(serialize(node), 'root,left,left.left,,,,right,,,')
        self.assertEqual(deserialize(serialize(node)).left.left.val, 'left.left')

    def testSerializeNullTree(self):
        node = None
        self.assertEqual(serialize(node), ',')
        self.assertEqual(deserialize(serialize(node)), None)

if __name__ == "__main__":
    unittest.main()
