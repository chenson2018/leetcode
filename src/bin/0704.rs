// https://leetcode.com/problems/binary-search/

use std::cmp::Ordering::*;
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_slice(&nums, target, 0)
            .map(|x| x as i32)
            .unwrap_or(-1)
    }

    pub fn search_slice(nums: &[i32], target: i32, idx_left: usize) -> Option<usize> {
        let mid = nums.len() / 2;
        match target.cmp(&nums[mid]) {
            Less | Greater if mid == 0 => None,
            Less => Self::search_slice(&nums[..mid], target, idx_left),
            Greater => Self::search_slice(&nums[mid..], target, idx_left + mid),
            Equal => Some(mid + idx_left),
        }
    }
}

fn main() {
    // println!("{}", Solution::search(vec![-1,0,3,5,9,12], 9));
    println!("{}", Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
}
