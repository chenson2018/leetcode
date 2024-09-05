// https://leetcode.com/problems/binary-tree-level-order-traversal/

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::strcutures::TreeNode;

struct Solution;

pub fn max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    root.as_ref().map_or(0, |node| {
        let node = node.borrow();
        1 + max_depth(&node.left).max(max_depth(&node.right))
    })
}

pub fn push_order(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>, depth : usize) {
    if let Some(node) = root {
        let node = node.borrow();
        res[depth].push(node.val);
        push_order(&node.left, res, depth + 1);
        push_order(&node.right, res, depth + 1);
    }
}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]; max_depth(&root)];
        push_order(&root, &mut res, 0);
        res        
    }
}

fn main() {}
