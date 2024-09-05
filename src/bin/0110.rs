// https://leetcode.com/problems/balanced-binary-tree/description/

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

pub fn is_balanced_borrow(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(node) => {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            (max_depth_borrow(left) - max_depth_borrow(right)).abs() <= 1
                && is_balanced_borrow(left)
                && is_balanced_borrow(right)
        }
        None => true,
    }
}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_balanced_borrow(&root)
    }
}

fn main() {}
