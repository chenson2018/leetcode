// https://leetcode.com/problems/maximum-depth-of-binary-tree/

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::strcutures::TreeNode;

struct Solution;

pub fn max_depth_borrow(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            1 + std::cmp::max(
                max_depth_borrow(&node.borrow().left),
                max_depth_borrow(&node.borrow().right),
            )
        }
        None => 0,
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_depth_borrow(&root)
    }
}

fn main() {}
