// https://leetcode.com/problems/swap-nodes-in-pairs/

use leetcode::strcutures::ListNode;

struct Solution;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::aux(&mut head);
        head
    }

    // Note: this defeats the point, because we copy the values. Imagine if
    // instead of i32 we had some expensive type. What we really want is to swap
    // the next points, i.e. the tail of the lists. I don't think we can do this
    // with the same nice pattern matching without box patterns...
    //
    // I really wish that the leetcode data structures used generics, it would
    // have made this immediately obvious.

    #[cfg_attr(any(), rustfmt::skip)]
    pub fn aux(head : &mut Option<Box<ListNode>>) {
        if let Some(ListNode { val, next : Some(node) }) = head.as_deref_mut() {
            (*val, node.val) = (node.val, *val);
            Self::aux(&mut node.next)
        }
    }
}

fn main() {}
