// https://leetcode.com/problems/evaluate-reverse-polish-notation/

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack : Vec<i32> = Vec::new();
        for token in tokens {
            let s = token.as_str();
            if ["/", "+", "-", "*"].contains(&s) {
            let l = stack.pop().unwrap();
            let r = stack.pop().unwrap();
            let val = match token.as_str() {
                "/" => r / l,
                "+" => r + l,
                "-" => r - l,
                "*" => r * l,
                _ => panic!()
            };
            stack.push(val);
            } else {
                stack.push(token.parse().unwrap())
            }
        }
        stack.pop().unwrap()
    }
}

fn main() {}
