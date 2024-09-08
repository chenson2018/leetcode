// https://leetcode.com/problems/first-bad-version/

struct Solution;

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            self.aux(1, n)
        }
    }

    pub fn aux(&self, left: i32, right: i32) -> i32 {
        if left + 1 == right {
            if self.isBadVersion(left) {
                left
            } else {
                right
            }
        } else {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                self.aux(left, mid)
            } else {
                self.aux(mid, right)
            }
        }
    }

    pub fn isBadVersion(&self, n: i32) -> bool {
        n >= 37
    }
}

fn main() {
    let sol = Solution;
    println!("{}", sol.first_bad_version(1000))
}
