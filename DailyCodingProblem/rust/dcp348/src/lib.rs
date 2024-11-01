/*
    This problem was asked by Zillow.

    A ternary search tree is a trie-like data structure where each node may have up to three
    children. Here is an example which represents the words code, cob, be, ax, war, and we.

           c
        /  |  \
       b   o   w
     / |   |   |
    a  e   d   a
    |    / |   | \
    x   b  e   r  e
    The tree is structured according to the following rules:

    left child nodes link to words lexicographically earlier than the parent prefix
    right child nodes link to words lexicographically later than the parent prefix
    middle child nodes continue the current word

    For instance, since code is the first word inserted in the tree, and cob lexicographically
    precedes cod, cob is represented as a left child extending from cod.

    Implement insertion and search functions for a ternary search tree.
*/

#[derive(Debug, Default)]
pub struct TernaryTree {
    ch: Option<char>,
    left: Option<Box<TernaryTree>>,
    mid: Option<Box<TernaryTree>>,
    right: Option<Box<TernaryTree>>,
}

impl TernaryTree {
    pub fn new() -> Self {
        Self {
            ch: None,
            left: None,
            mid: None,
            right: None,
        }
    }

    pub fn insert(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }

        let mut chars = word.chars();
        let ch = chars.next().unwrap();
        let rest = chars.as_str();

        if self.ch.is_none() {
            self.ch = Some(ch);
            if rest.is_empty() {
                return;
            }
            let mut subtree = Box::new(TernaryTree::new());
            subtree.insert(rest);
            self.mid = Some(subtree);
            return;
        }

        // Node already has a character and possibly subtrees. Find the right subtree to insert the
        // word on.

        if let Some(mych) = self.ch {
            match mych {
                x if ch < x => {
                    self.left
                        .get_or_insert(Box::new(TernaryTree::new()))
                        .insert(word);
                }
                x if ch == x => {
                    self.mid
                        .get_or_insert(Box::new(TernaryTree::new()))
                        .insert(rest);
                }
                _ => {
                    self.right
                        .get_or_insert(Box::new(TernaryTree::new()))
                        .insert(word);
                }
            }
        }
    }

    pub fn search(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }

        if self.ch.is_none() {
            return false;
        }

        let mut chars = word.chars();
        let ch = chars.next().unwrap();
        let rest = chars.as_str();

        let mych = self.ch.unwrap();
        match mych {
            x if ch < x => self.left.as_ref().map_or(false, |n| n.search(word)),
            x if ch == x => {
                if rest.is_empty() {
                    return true;
                }
                self.mid.as_ref().map_or(false, |n| n.search(rest))
            }
            _ => self.right.as_ref().map_or(false, |n| n.search(word)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut tree = TernaryTree::new();
        tree.insert("code");
        tree.insert("cob");
        tree.insert("be");
        tree.insert("ax");
        tree.insert("war");
        tree.insert("we");

        assert_eq!(false, tree.search(""));
        assert!(tree.search("code"));
        assert!(tree.search("cob"));
        assert!(tree.search("we"));
        assert!(tree.search("be"));
        assert!(tree.search("ax"));
        assert!(tree.search("war"));
    }
}
