// https://leetcode.com/problems/swap-nodes-in-pairs/

use leetcode::strcutures::ListNode;

struct Solution;

impl Solution {
    //          n1.next             n2.next
    //  [1]     ->          [2]     ->          [3]  ->  ...
    //
    //
    //      n1.next =  swap_pairs(n2.next)
    //
    //
    //  [2]     ->
    //                  swap ([3] -> ...)
    //  [1]     ->
    //
    //
    //      n2.next = Some(n1)
    //
    //
    //  [2] -> [1] -> swap([3] -> ...)
    //

    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut n1) => match n1.next {
                Some(mut n2) => {
                    n1.next = Self::swap_pairs(n2.next);
                    n2.next = Some(n1);
                    Some(n2)
                }
                None => Some(n1),
            },
            None => None,
        }
    }
}

fn main() {
    let ex = ListNode::generate((1..=5).collect());
    println!("{:?}", Solution::swap_pairs(ex));
}
