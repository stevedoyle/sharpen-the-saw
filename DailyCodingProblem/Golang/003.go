/*
Diffuculty: Hard

## The Problem

This problem was asked by Google.

Given the root to a binary tree, implement serialize(root), which serializes the
tree into a string, and deserialize(s), which deserializes the string back into
the tree.

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
*/

package dailycodingproblem

import (
	"strings"
)

type Node struct {
	val   string
	left  *Node
	right *Node
}

func serialize(node *Node) string {
	// Using ',' as a node name separator in the serialized string.
	// Using an empty name string to represent an empty (non-existent) node
	// in the tree.
	if node == nil {
		return ","
	}
	str := node.val + ","
	str += serialize(node.left)
	str += serialize(node.right)
	return str
}

func deserialize(str string) *Node {
	elems := strings.Split(str, ",")
	// Using a channel to pass an iterator over the elements to recursive calls
	eliter := make(chan string)
	go func() {
		for _, elem := range elems {
			defer close(eliter)
			eliter <- elem
		}
	}()
	return deserializePreOrder(eliter)
}

func deserializePreOrder(eliter chan string) *Node {
	elem, ok := <-eliter
	if !ok || len(elem) == 0 {
		return nil
	}
	node := &Node{elem,
		deserializePreOrder(eliter),
		deserializePreOrder(eliter)}
	return node
}
