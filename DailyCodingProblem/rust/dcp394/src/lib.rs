/*
This problem was asked by Uber.

Given a binary tree and an integer k, return whether there exists a
root-to-leaf path that sums up to k.

For example, given k = 18 and the following binary tree:

    8
   / \
  4   13
 / \   \
2   6   19
Return True since the path 8 -> 4 -> 6 sums to 18.
*/

#[derive(Debug)]
pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert_left(&mut self, value: i32) {
        self.left = Some(Box::new(Node::new(value)));
    }

    pub fn insert_right(&mut self, value: i32) {
        self.right = Some(Box::new(Node::new(value)));
    }
}

pub fn path_exists(tree: &Node, target: i32) -> bool {
    println!("{:?}", tree.value);
    if tree.value == target {
        return true;
    }
    tree.left
        .as_ref()
        .map_or(false, |n| path_exists(n, target - tree.value))
        || tree
            .right
            .as_ref()
            .map_or(false, |n| path_exists(n, target - tree.value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_exists() {
        let mut tree = Node::new(8);
        tree.insert_left(4);
        tree.insert_right(13);
        tree.left.as_mut().unwrap().insert_left(2);
        tree.left.as_mut().unwrap().insert_right(6);
        tree.right.as_mut().unwrap().insert_right(19);

        assert!(path_exists(&tree, 18));
    }
}
