use crate::get_input::get_input;
use std::cmp::max;

pub struct Day02 {}

impl Day02 {
    pub fn p1(input: &str) {
        let mut total = 0;

        for (i, line) in input.lines().enumerate() {
            let game_id = i + 1;
            let game_str = line.split_once(": ").unwrap().1;
            let sets: Vec<&str> = game_str.split("; ").collect();
            let mut possible = true;

            for set in sets {
                let draws: Vec<&str> = set.split(", ").collect();
                let mut red_cubes: i32 = 12;
                let mut green_cubes: i32 = 13;
                let mut blue_cubes: i32 = 14;

                for draw in draws {
                    let (num, cube_colour) = draw.split_once(" ").unwrap();
                    let num: i32 = num.parse().unwrap();

                    match cube_colour {
                        "red" => red_cubes -= num,
                        "blue" => blue_cubes -= num,
                        "green" => green_cubes -= num,
                        _ => (),
                    };
                }

                if red_cubes < 0 || blue_cubes < 0 || green_cubes < 0 {
                    possible = false;
                }
            }

            if possible {
                total += game_id;
            }
        }

        println!("Day 02 P1: {}", total);
    }

    pub fn p2(input: &str) {
        let mut total = 0;

        for line in input.lines() {
            let game_str = line.split_once(": ").unwrap().1;
            let sets: Vec<&str> = game_str.split("; ").collect();

            let mut red_cubes: i32 = 0;
            let mut green_cubes: i32 = 0;
            let mut blue_cubes: i32 = 0;

            for set in sets {
                let draws: Vec<&str> = set.split(", ").collect();

                for draw in draws {
                    let (num, cube_colour) = draw.split_once(" ").unwrap();
                    let num: i32 = num.parse().unwrap();

                    match cube_colour {
                        "red" => red_cubes = max(red_cubes, num),
                        "blue" => blue_cubes = max(blue_cubes, num),
                        "green" => green_cubes = max(green_cubes, num),
                        _ => (),
                    };
                }
            }

            let power = green_cubes * red_cubes * blue_cubes;
            total += power;
        }

        println!("Day 02 P2: {}", total);
    }

    pub fn run() {
        let input = get_input("input/day02.txt");
        Self::p1(&input);
        Self::p2(&input);
    }
}
