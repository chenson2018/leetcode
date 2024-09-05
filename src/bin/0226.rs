// https://leetcode.com/problems/invert-binary-tree/description/

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::strcutures::TreeNode;

struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|node| {
            let mut node_ref = node.borrow_mut();
            (node_ref.left, node_ref.right) = (
                Self::invert_tree(node_ref.right.take()),
                Self::invert_tree(node_ref.left.take()),
            );
            node.clone()
        })
    }
}

fn main() {}
