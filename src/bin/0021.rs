// https://leetcode.com/problems/merge-two-sorted-lists/description/

use leetcode::strcutures::ListNode;
struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (&list1, &list2) {
            (Some(b1), Some(b2)) => {
                let (fst, snd) = if b1.val < b2.val {
                    (b1, list2)
                } else {
                    (b2, list1)
                };
                return Some(Box::new(ListNode {
                    val: fst.val,
                    next: Solution::merge_two_lists(fst.next.clone(), snd),
                }));
            }
            (None, Some(_)) => list2,
            (Some(_), None) => list1,
            (None, None) => None,
        }
    }
}

fn main() {}
