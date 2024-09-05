// https://leetcode.com/problems/valid-palindrome/

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let f: Vec<_> = s
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect();
        f.iter().eq(f.iter().rev())
    }
}

fn main() {}
