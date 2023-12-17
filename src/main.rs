mod solutions {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
}

use crate::solutions::day01::Day01;
use crate::solutions::day02::Day02;
use crate::solutions::day03::Day03;
use crate::solutions::day04::Day04;


mod get_input;

fn main() {
    Day01::run();
    Day02::run();
    Day03::run();
    Day04::run();
}
