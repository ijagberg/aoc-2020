use aoc_2020::{self, passport};

fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    let valid_passports = passport::read_valid_passports_from_file(&input_file);

    println!("{}", valid_passports.len());
}
