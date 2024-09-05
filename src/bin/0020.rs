// https://leetcode.com/problems/valid-parentheses/description/

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '{' | '(' | '[' => stack.push(c),
                _ => match (stack.pop(), c) {
                    (Some('{'), '}') | (Some('['), ']') | (Some('('), ')') => {}
                    _ => return false,
                },
            }
        }
        return stack.is_empty();
    }
}

fn main() {
    println!("{:?}", Solution::is_valid("()".to_string()));
    println!("{:?}", Solution::is_valid("()[]{}".to_string()));
    println!("{:?}", Solution::is_valid("(]".to_string()));
    println!("{:?}", Solution::is_valid("([])".to_string()));
}
