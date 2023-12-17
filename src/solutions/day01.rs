use std::u64::MAX;

use crate::get_input::get_input;

pub struct Day01 {}

impl Day01 {
    fn p1(input: &str) {
        let result: u64 = input
            .lines()
            .map(|l| {
                let first = l.chars().find(|x| x.is_ascii_digit()).unwrap();
                let last = l.chars().rev().find(|x| x.is_ascii_digit()).unwrap();
                let string = format!("{}{}", first, last);
                string.parse::<u64>().unwrap()
            })
            .reduce(|a, b| a + b)
            .unwrap_or_default();
        println!("Day 01 P1: {}", result);
    }

    fn p2(input: &str) {
        let nums = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let result: u64 = input
            .lines()
            .map(|l| {
                let a = l.chars().position(|x| x.is_ascii_digit());
                let b = l.chars().rev().position(|x| x.is_ascii_digit());
                let mut min: u64 = a.unwrap_or(usize::MAX) as u64;
                let mut max: u64 = match b {
                    Some(val) => (l.chars().count() - val - 1) as u64,
                    None => 0,
                };
                let start_digit = l.chars().find(|x| x.is_ascii_digit());
                let end_digit = l.chars().rev().find(|x| x.is_ascii_digit());
                let mut start = match start_digit {
                    Some(val) => val.to_string().parse().unwrap(),
                    None => u64::MAX,
                };
                let mut end = match end_digit {
                    Some(val) => val.to_string().parse().unwrap(),
                    None => 0,
                };

                for (i, num) in nums.iter().enumerate() {
                    let first: u64 = l.find(num).unwrap_or(MAX.try_into().unwrap()) as u64;
                    let last: u64 = l.rfind(num).unwrap_or(0) as u64;
                    if first < min {
                        min = first;
                        start = i as u64 + 1;
                    }

                    if last > max {
                        max = last;
                        end = i as u64 + 1;
                    }
                }

                let res = start * 10 + end;
                res
            })
            .reduce(|a, b| a + b)
            .unwrap();
        println!("Day 01 P2: {}", result);
    }

    pub fn run() {
        let input = get_input("input/day01.txt");
        Self::p1(&input);
        Self::p2(&input);
    }
}
