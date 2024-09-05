// https://leetcode.com/problems/insert-interval/

struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let [mut l, mut r] = new_interval[..] else {
            panic!()
        };
        let mut merge = true;
        fn push_merge(m: &mut bool, l: &mut i32, r: &mut i32, res: &mut Vec<Vec<i32>>) {
            if *m {
                *m = false;
                res.push(vec![*l, *r]);
            }
        }

        for i in intervals {
            let [old_left, old_right] = i[..] else {
                panic!()
            };

            // the inserted interval is first, with no intersection
            if r < old_left {
                push_merge(&mut merge, &mut l, &mut r, &mut res);
            }

            // untouched intervals to the left
            if old_right < l {
                res.push(i)
            }
            // past the merges
            else if r < old_left {
                push_merge(&mut merge, &mut l, &mut r, &mut res);
                res.push(i)
            }
            // in the merge
            else {
                l = l.min(old_left);
                r = r.max(old_right);
            }
        }
        // push merge if it is last interval
        push_merge(&mut merge, &mut l, &mut r, &mut res);
        res
    }
}

fn main() {}
