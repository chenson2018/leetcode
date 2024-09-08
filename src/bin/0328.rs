// https://leetcode.com/problems/odd-even-linked-list/description/

use leetcode::strcutures::ListNode;

struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // the lists we are constructing
        let mut even = None;
        let mut odd = None;

        // references to their tails
        let mut even_tl = &mut even;
        let mut odd_tl = &mut odd;

        let mut parity = false;
        while let Some(mut node) = head {
            head = node.next.take();
            parity = !parity;
            if parity {
                *even_tl = Some(node);
                even_tl = &mut even_tl.as_mut()?.next;
            } else {
                *odd_tl = Some(node);
                odd_tl = &mut odd_tl.as_mut()?.next;
            }
        }
        *even_tl = odd;
        even
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
    let ex = Solution::generate((1..=5).collect());
    // println!("{:?}", ex);
    println!("{:?}", Solution::odd_even_list(ex));
}
