// https://leetcode.com/problems/reverse-linked-list/

use leetcode::strcutures::ListNode;
struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::rev_app(head, None)
    }

    pub fn rev_app(xs: Option<Box<ListNode>>, ys: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match xs {
            None => ys,
            Some(b) => {
                let ListNode { val, next } = *b;
                Self::rev_app(next, Some(Box::new(ListNode { val, next: ys })))
            }
        }
    }
}

fn main() {
    let xs = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    println!("{:?}", xs);
    println!("{:?}", Solution::reverse_list(xs));
}
