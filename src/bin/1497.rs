// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/

struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut counts = vec![0; k as usize];
        for x in arr {
            counts[x.rem_euclid(k) as usize] += 1;
        }
        for i in 1..k/2+1 {
            if counts[i as usize] != counts[(k - i) as usize] {return false}
        }
        counts[0] % 2 == 0
    }
}

fn main() {
}
