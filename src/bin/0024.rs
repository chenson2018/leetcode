// https://leetcode.com/problems/swap-nodes-in-pairs/

use leetcode::strcutures::ListNode;

struct Solution;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::aux(&mut head);
        head
    }

    #[cfg_attr(any(), rustfmt::skip)]
    pub fn aux(head : &mut Option<Box<ListNode>>) {
        if let Some(ListNode { val, next : Some(node) }) = head.as_deref_mut() {
            (*val, node.val) = (node.val, *val);
            Self::aux(&mut node.next)
        }
    }
}

fn main() {}
