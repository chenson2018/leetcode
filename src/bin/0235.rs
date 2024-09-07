// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::strcutures::TreeNode;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (p, q) {
            (Some(p), Some(q)) => {
                let p = p.borrow().val;
                let q = q.borrow().val;
                let (p, q) = if p < q { (p, q) } else { (q, p) };
                Self::lowest_common_ancestor_vals(root, p, q)
            }
            _ => None,
        }
    }

    // calling convention that p < q for simplicity
    pub fn lowest_common_ancestor_vals(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root.as_ref() {
            Some(node) => {
                let node = node.borrow();
                if p <= node.val && node.val <= q {
                    root.clone()
                } else if node.val < p {
                    Self::lowest_common_ancestor_vals(node.right.clone(), p, q)
                } else if q < node.val {
                    Self::lowest_common_ancestor_vals(node.left.clone(), p, q)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

fn main() {}
