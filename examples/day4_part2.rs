use aoc_2020::{self, passport::Passport};

fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    let passports = aoc_2020::passport::read_passports_from_file(&input_file);

    println!("{}", validate_passports(passports));
}

fn validate_passports(passports: Vec<Passport>) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}
