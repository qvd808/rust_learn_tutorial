pub mod binary_tree {
    pub struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T: std::fmt::Display> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }

        pub fn in_order(&self) {
            if let Some(ref left) = self.left {
                left.in_order()
            }
            println!("{}", self.key);
            if let Some(ref right) = self.right {
                right.in_order()
            }
        }
    }
}
