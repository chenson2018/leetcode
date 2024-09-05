// https://leetcode.com/problems/validate-binary-search-tree/

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::strcutures::TreeNode;

struct Solution;

pub fn is_valid_bst_b(
    root: &Option<Rc<RefCell<TreeNode>>>,
    low: Option<i32>,
    high: Option<i32>,
) -> bool {
    root.as_ref().map_or(true, |node| {
        let node = node.borrow();
        low.map_or(true, |x| x < node.val)
            && high.map_or(true, |x| node.val < x)
            && is_valid_bst_b(&node.left, low, Some(node.val))
            && is_valid_bst_b(&node.right, Some(node.val), high)
    })
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_valid_bst_b(&root, None, None)
    }
}

fn main() {}
