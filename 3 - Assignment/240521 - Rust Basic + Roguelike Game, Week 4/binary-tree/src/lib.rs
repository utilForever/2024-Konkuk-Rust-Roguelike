// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

// A container storing a set of values, using a binary tree.
//
// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

// Implement `new`, `insert`, `len`, and `has`.

impl<T: Ord> Subtree<T> {
    pub fn len(&self) -> i32 {
        if self.0.is_none() {
            return 0;
        }
        let node = self.0.as_ref().unwrap();
        node.left.len() + node.right.len() + 1
    }

    pub fn insert(&mut self, value: T) {
        if self.0.is_none() {
            self.0 = Some(Box::new(Node {
                value: value,
                left: Subtree(None),
                right: Subtree(None),
            }));
            return;
        }
        let node = self.0.as_mut().unwrap();
        match value.cmp(&node.value) {
            std::cmp::Ordering::Less => node.left.insert(value),
            std::cmp::Ordering::Greater => node.right.insert(value),
            std::cmp::Ordering::Equal => {}
        }
    }

    pub fn has(&self, value: &T) -> bool {
        if self.0.is_none() {
            return false;
        }
        let node = self.0.as_ref().unwrap();
        match value.cmp(&node.value) {
            std::cmp::Ordering::Less => node.left.has(value),
            std::cmp::Ordering::Greater => node.right.has(value),
            std::cmp::Ordering::Equal => true,
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree {
            root: Subtree(None),
        }
    }

    pub fn len(&self) -> i32 {
        self.root.len()
    }

    pub fn insert(&mut self, value: T) {
        self.root.insert(value)
    }

    pub fn has(&self, value: &T) -> bool {
        self.root.has(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();

        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();

        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|val| tree.has(&(val as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();

        for i in 0..100 {
            tree.insert(i);
        }

        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
