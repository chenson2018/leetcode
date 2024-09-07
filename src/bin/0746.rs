// https://leetcode.com/problems/min-cost-climbing-stairs/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        cost.extend([0, 0]);
        let mut table = HashMap::new();
        Self::aux(&cost, &mut table, cost.len() - 1)
    }

    pub fn aux(cost: &[i32], table: &mut HashMap<usize, i32>, idx: usize) -> i32 {
        if idx <= 1 {
            cost[idx]
        } else if let Some(memo) = table.get(&idx) {
            *memo
        } else {
            let res = cost[idx]
                + std::cmp::min(
                    Self::aux(cost, table, idx - 1),
                    Self::aux(cost, table, idx - 2),
                );
            table.insert(idx, res);
            res
        }
    }
}

fn main() {
    println!("{}", Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    println!(
        "{}",
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
    );
}
