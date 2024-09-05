// https://leetcode.com/problems/merge-intervals/

struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sorted = intervals;
        sorted.sort();

        let mut res = Vec::new();
        let mut merging = Vec::new();

        for x in sorted {
            if merging.is_empty() {
                merging = x;
            } else {
            let [acc_l, acc_r] = merging[..] else {panic!()};
            let [x_l, x_r] = x[..] else {panic!()};
        
            if acc_r < x_l {
                res.push(merging);
                merging = x;
            } else {
                merging = vec![acc_l.min(x_l), acc_r.max(x_r)];
            }
            }
        }
        res.push(merging);
        res
    }
}

fn main() {}
