use std::collections::HashMap;

use crate::get_input::get_input;

pub struct Day03 {}
// 0 1 2
// 3 4 5
// 6 7 8

impl Day03 {
    fn p1(input: &str) {
        let input_matrix: Vec<Vec<char>> =
            input.lines().map(|line| line.chars().collect()).collect();

        let num_rows = input_matrix.len();
        let num_cols = input_matrix[0].len();

        let mut total = 0;

        for y in 0..input_matrix.len() {
            let mut num_str = String::new();
            let mut is_part = false;

            for x in 0..input_matrix[y].len() {
                let char = input_matrix[y][x];
                if char.is_numeric() {
                    num_str.push(char.clone());
                    let moves = [
                        (y, x.saturating_sub(1)),
                        (y, x.saturating_add(1)),
                        (y.saturating_add(1), x),
                        (y.saturating_sub(1), x),
                        (y.saturating_add(1), x.saturating_sub(1)),
                        (y.saturating_add(1), x.saturating_add(1)),
                        (y.saturating_sub(1), x.saturating_sub(1)),
                        (y.saturating_sub(1), x.saturating_add(1)),
                    ];

                    for m in moves {
                        if m.1 >= num_cols || m.0 >= num_rows {
                            continue;
                        }
                        let c = input_matrix[m.0][m.1];
                        if !c.is_numeric() && c != '.' {
                            is_part = true;
                        }
                    }
                } else {
                    if is_part && num_str.len() > 0 {
                        let num: u128 = num_str.parse().expect("cannot parse num");
                        total += num;
                    }
                    is_part = false;
                    num_str = String::new();
                }
            }

            if is_part && num_str.len() > 0 {
                let num: u128 = num_str.parse().expect("cannot parse num");
                total += num;
            }
        }

        println!("Day 03 P1: {:?}", total);
    }

    fn p2(input: &str) {
        let input_matrix: Vec<Vec<char>> =
            input.lines().map(|line| line.chars().collect()).collect();

        let num_rows = input_matrix.len();
        let num_cols = input_matrix[0].len();

        let mut total = 0;
        let mut gears: HashMap<(usize, usize), Vec<u128>> = HashMap::new();

        let mut star_location = (0, 0);
        for y in 0..input_matrix.len() {
            let mut num_str = String::new();
            let mut has_star = false;

            for x in 0..input_matrix[y].len() {
                let char = input_matrix[y][x];
                if char.is_numeric() {
                    num_str.push(char.clone());
                    let moves = [
                        (y, x.saturating_sub(1)),
                        (y, x.saturating_add(1)),
                        (y.saturating_add(1), x),
                        (y.saturating_sub(1), x),
                        (y.saturating_add(1), x.saturating_sub(1)),
                        (y.saturating_add(1), x.saturating_add(1)),
                        (y.saturating_sub(1), x.saturating_sub(1)),
                        (y.saturating_sub(1), x.saturating_add(1)),
                    ];

                    for m in moves {
                        if m.1 >= num_cols || m.0 >= num_rows {
                            continue;
                        }
                        let c = input_matrix[m.0][m.1];
                        if c == '*' {
                            has_star = true;
                            star_location = (m.0, m.1);
                        }
                    }
                } else {
                    if has_star && num_str.len() > 0 {
                        let num: u128 = num_str.parse().expect("cannot parse num");
                        match gears.get_mut(&star_location) {
                            Some(gear) => {
                                gear.push(num);
                            }
                            None => {
                                gears.insert(star_location, vec![num]);
                            }
                        }
                    }
                    star_location = (0, 0);
                    has_star = false;
                    num_str = String::new();
                }
            }

            if has_star && num_str.len() > 0 {
                let num: u128 = num_str.parse().expect("cannot parse num");
                match gears.get_mut(&star_location) {
                    Some(gear) => {
                        gear.push(num);
                    }
                    None => {
                        gears.insert(star_location, vec![num]);
                    }
                }
            }
            star_location = (0, 0);
        }

        for (_, nums) in gears {
            if nums.len() == 2 {
                total += nums[0] * nums[1]
            }
        }

        println!("Day 03 P2: {:?}", total);
    }

    pub fn run() {
        let input = get_input("input/day03.txt");
        Self::p1(&input);
        Self::p2(&input);
    }
}
