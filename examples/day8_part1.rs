use aoc_2020::{self, handheld::parse_handheld_from_input_file};

fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    println!("{}", solve_part1_from_input_file(&input_file));
}

pub fn solve_part1_from_input_file(file: &str) -> i32 {
    let mut handheld = parse_handheld_from_input_file(file);

    handheld.step_until_termination().unwrap_err();

    handheld.accumulator()
}
