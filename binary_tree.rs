#![allow(unused)]

use core::panic;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


struct TreeNode<T> {
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
    pub key: T,
}


impl<T: std::fmt::Display> TreeNode<T> {
    pub fn new(key: T) -> Self {
        TreeNode { left: None, right: None, key }
    }

    pub fn left(mut self ,node: TreeNode<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self ,node: TreeNode<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn in_order(&self){

        if let Some(ref left) = self.left {
            left.in_order()
        }
        println!("{}", self.key);
        if let Some(ref right) = self.right {
            right.in_order()
        }
    }

}
fn main() {

    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));

    node1.in_order();


}

