// https://leetcode.com/problems/middle-of-the-linked-list/

use leetcode::strcutures::ListNode;

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::aux(&head, &head).clone()
    }

    pub fn aux<'a>(
        slow: &'a Option<Box<ListNode>>,
        fast: &'a Option<Box<ListNode>>,
    ) -> &'a Option<Box<ListNode>> {
        if let (
            Some(slow_ref),
            Some(ListNode {
                next: Some(node), ..
            }),
        ) = (slow, fast.as_deref())
        {
            Self::aux(&slow_ref.next, &node.next)
        } else {
            slow
        }
    }

    pub fn generate(data: Vec<i32>) -> Option<Box<ListNode>> {
        let mut res = None;
        for &val in data.iter().rev() {
            res = Some(Box::new(ListNode { val, next: res }))
        }
        res
    }
}

fn main() {
    let ex = Solution::generate((1..=10).collect());
    println!("{:?}", Solution::middle_node(ex));
}
