// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut counts = vec![0; k as usize];
        for x in arr {
            counts[x.rem_euclid(k) as usize] += 1;
        }
        for i in 1..k / 2 + 1 {
            if counts[i as usize] != counts[(k - i) as usize] {
                return false;
            }
        }
        counts[0] % 2 == 0
    }

    pub fn can_arrange_alt(arr: Vec<i32>, k: i32) -> bool {
        let mut counts = HashMap::new();
        for x in arr {
            *counts.entry(x.rem_euclid(k)).or_default() += 1;
        }
        for i in 1..k / 2 + 1 {
            if counts.get(&i) != counts.get(&(k - i)) {
                return false;
            }
        }
        counts.get(&0).map_or(true, |x| x % 2 == 0)
    }
}

fn main() {}
