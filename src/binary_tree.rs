use std::collections::VecDeque;
#[cfg(test)]
mod tests {
    use super::BinaryTree;

    #[test]
    fn create_new_tree() {
        let tree = BinaryTree::new(1);

        assert_eq!(tree.value, 1);
    }

    #[test]
    fn insert_left() {
        let tree = BinaryTree::new(1).left(BinaryTree::new(2));

        if let Some(node) = tree.left {
            assert_eq!(node.value, 2);
        }

        assert_eq!(tree.right, None);
    }

    #[test]
    fn insert_right() {
        let tree = BinaryTree::new(1).right(BinaryTree::new(2));

        if let Some(node) = tree.right {
            assert_eq!(node.value, 2);
        }

        assert_eq!(tree.left, None);
    }
    #[test]
    fn insert() {
        let mut tree = BinaryTree::new(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
        tree.insert(5);
    
        assert_eq!(
            tree,
            BinaryTree::new(1)
                .left(BinaryTree::new(2).left(BinaryTree::new(4)).right(BinaryTree::new(5)))
                .right(BinaryTree::new(3))
        );
    
        tree.insert(6);
    
        assert_eq!(
            tree,
            BinaryTree::new(1)
                .left(BinaryTree::new(2).left(BinaryTree::new(4)).right(BinaryTree::new(5)))
                .right(BinaryTree::new(3).left(BinaryTree::new(6)))
        )
    }
    #[test]
    fn create_new_tree_with_from() {
        // `BinaryTree::from` takes in a reference of an array because borrowing is sufficient
        let tree = BinaryTree::from(&[1, 2, 3, 4, 5, 6]);
    
        assert_eq!(
            tree,
            BinaryTree::new(1)
                .left(BinaryTree::new(2).left(BinaryTree::new(4)).right(BinaryTree::new(5)))
                .right(BinaryTree::new(3).left(BinaryTree::new(6)))
        )
    }
}

/// A binary tree data structure.
#[derive(PartialEq, Debug)]
pub struct BinaryTree<T> {
    /// The value stored in the node.
    pub value: T,
    /// The left child of the node.
    pub left: Option<Box<BinaryTree<T>>>,
    /// The right child of the node.
    pub right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T>
where
    T: Copy + PartialEq,
{
    /// Creates a new binary tree node with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the node.
    ///
    /// # Returns
    ///
    /// A new binary tree node with the given value and no children.
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    /// Inserts a new value into the binary tree.
    ///
    /// The value is inserted into the first available position in the tree,
    /// following a breadth-first traversal.
    ///
    /// # Arguments
    ///
    /// * `new_value` - The value to insert into the tree.
    pub fn insert(&mut self, new_value: T) {
        let mut queue: VecDeque<&mut BinaryTree<T>> = VecDeque::new();
        queue.push_front(self);

        loop {
            let BinaryTree {
                ref mut left,
                ref mut right,
                ..
            } = queue.pop_back().unwrap();

            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *left = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *right = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }
        }
    }

    /// Creates a binary tree from a slice of values.
    ///
    /// The first value in the slice is used as the root of the tree,
    /// and the remaining values are inserted into the tree in breadth-first order.
    ///
    /// # Arguments
    ///
    /// * `new_values` - A slice containing the values to insert into the tree.
    ///
    /// # Returns
    ///
    /// A binary tree containing the values from the input slice.
    pub fn from(new_values: &[T]) -> Self {
        let (first, rest) = new_values.split_first().unwrap();
        let mut root: BinaryTree<T> = BinaryTree::new(*first);

        for value in rest {
            root.insert(*value)
        }
        root
    }

    /// Adds a left child to the current node.
    ///
    /// # Arguments
    ///
    /// * `node` - The binary tree node to add as the left child.
    ///
    /// # Returns
    ///
    /// A new binary tree node with the specified left child.
    fn left(mut self, node: BinaryTree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    /// Adds a right child to the current node.
    ///
    /// # Arguments
    ///
    /// * `node` - The binary tree node to add as the right child.
    ///
    /// # Returns
    ///
    /// A new binary tree node with the specified right child.
    fn right(mut self, node: BinaryTree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}
