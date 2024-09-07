// https://leetcode.com/problems/invert-binary-tree/description/

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        (0..n).for_each(|_| (a, b) = (a + b, a));
        b
    }
}

fn main() {
    println!("{}", Solution::climb_stairs(6))
}
