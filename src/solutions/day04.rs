use std::collections::{HashMap, HashSet};

use crate::get_input::get_input;

pub struct Day04 {}

impl Day04 {
    fn p1(input: &str) {
        let mut result = 0;

        for (_, line) in input.lines().enumerate() {
            let mut set = HashSet::new();
            let mut count = 0;
            let (_, card) = line.split_once(": ").unwrap();
            let (winning_nums, nums) = card.split_once(" | ").unwrap();
            let winning_nums: Vec<u32> = winning_nums
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let nums: Vec<u32> = nums
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            for num in nums {
                set.insert(num);
            }

            for winning_num in winning_nums {
                if set.contains(&winning_num) {
                    count += 1;
                }
            }

            if count > 0 {
                let game_result = usize::pow(2, count - 1);
                result += game_result;
            }
        }

        println!("Day 04 P1: {}", result);
    }

    fn p2(input: &str) {
        let mut result = 0;
        let lines: Vec<&str> = input.lines().collect();
        let num_cards = lines.len();
        let mut copies = HashMap::new();

        for (i, line) in lines.iter().enumerate() {
            let mut set = HashSet::new();
            let mut count = 0;
            let (_, card) = line.split_once(": ").unwrap();
            let (winning_nums, nums) = card.split_once(" | ").unwrap();
            let winning_nums: Vec<u32> = winning_nums
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let nums: Vec<u32> = nums
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            for num in nums {
                set.insert(num);
            }

            for winning_num in winning_nums {
                if set.contains(&winning_num) {
                    count += 1;
                }
            }

            let current_card_id = i + 1;
            let curr_copy = copies.entry(current_card_id).or_insert(0);
            *curr_copy += 1;
            let curr_copy: i32 = *curr_copy;
            for num in 0..count {
                let card_num = current_card_id + num + 1;
                if card_num > num_cards {
                    continue;
                }

                let copy = copies.entry(card_num).or_insert(0);
                *copy += 1*curr_copy;
            }
        }

        let mut result = 0;
        for (_, num) in copies {
            result += num;
        }

        println!("Day 04 P2: {}", result);
    }

    pub fn run() {
        let input = get_input("input/day04.txt");

        Self::p1(&input);
        Self::p2(&input);
    }
}
