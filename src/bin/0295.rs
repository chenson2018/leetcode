// https://leetcode.com/problems/find-median-from-data-stream/description/

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct MedianFinder {
    max: BinaryHeap<i32>,
    min: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            min: BinaryHeap::new(),
            max: BinaryHeap::new(),
        }
    }

    fn balance(&mut self) {
        if self.max.len() > self.min.len() + 1 {
            let val = self.max.pop().unwrap();
            self.min.push(Reverse(val));
        }

        if self.min.len() > self.max.len() + 1 {
            let Reverse(val) = self.min.pop().unwrap();
            self.max.push(val);
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.max.is_empty() || self.min.is_empty() {
            self.max.push(num);
            self.balance();
        } else {
            let l = self.max.peek().unwrap();

            if &num < l {
                self.max.push(num)
            } else {
                self.min.push(Reverse(num))
            }

            self.balance();
        }
    }

    fn find_median(&self) -> f64 {
        match (self.max.peek(), self.min.peek()) {
            (Some(val), None) => *val as f64,
            (None, Some(_)) => unreachable!(),
            (Some(l), Some(Reverse(r))) => {
                let l = *l as f64;
                let r = *r as f64;
                let l_len = self.max.len();
                let r_len = self.min.len();
                if (l_len + r_len) % 2 == 0 {
                    (l + r) / 2.0
                } else if l_len > r_len {
                    l
                } else {
                    r
                }
            }
            (None, None) => 0.0,
        }
    }
}

fn main() {
    let mut finder = MedianFinder::new();
    finder.add_num(1);
    finder.add_num(2);
    println!("{:?}", finder);
    println!("median : {:?}", finder.find_median());

    finder.add_num(3);
    println!("{:?}", finder);
    println!("median : {:?}", finder.find_median());
}
