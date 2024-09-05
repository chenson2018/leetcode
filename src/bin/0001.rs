// https://leetcode.com/problems/two-sum/description/

struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut table: HashMap<i32, usize> = HashMap::new();
        for (idx, value) in nums.iter().enumerate() {
            let complement = target - value;
            if let Some(idx_match) = table.get(&complement) {
                return vec![idx as i32, *idx_match as i32];
            }
            table.insert(*value, idx);
        }
        return vec![];
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
    println!("{:?}", Solution::two_sum(vec![3, 3], 6));
}
