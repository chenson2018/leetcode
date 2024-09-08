// https://leetcode.com/problems/search-a-2d-matrix/

struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let row = Self::find_row(&matrix, target, 0, matrix.len() - 1);
        match (matrix.get(row), matrix.get(row + 1)) {
            (Some(v1), Some(v2)) => {
                Self::search_slice(v1, target, 0).is_some()
                    || Self::search_slice(v2, target, 0).is_some()
            }
            (Some(v1), None) => Self::search_slice(v1, target, 0).is_some(),
            _ => false,
        }
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

    pub fn find_row(matrix: &Vec<Vec<i32>>, target: i32, left: usize, right: usize) -> usize {
        if matrix.len() == 1 {
            0
        } else if left + 1 == right {
            if matrix[left][0] > target {
                left - 1
            } else {
                right - 1
            }
        } else {
            let mid = left + (right - left) / 2;
            if matrix[mid][0] > target {
                Self::find_row(matrix, target, left, mid)
            } else {
                Self::find_row(matrix, target, mid, right)
            }
        }
    }
}

fn main() {
    // println!("{}", Solution::search_matrix(vec![vec![1], vec![3]], 3));
}
