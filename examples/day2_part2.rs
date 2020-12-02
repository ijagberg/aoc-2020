use aoc_2020::{self, password};

fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    let lines: Vec<String> = aoc_2020::read_lines_from_file(&input_file)
        .unwrap()
        .map(|l| l.unwrap())
        .collect();

    println!("{}", count_valid_passwords(lines));
}

fn count_valid_passwords(lines: Vec<String>) -> usize {
    lines
        .into_iter()
        .map(|l| {
            let (policy, password) = password::parse_password_and_policy(&l).unwrap();
            policy.validate_toboggan_password(&password)
        })
        .filter(|&b| b == true)
        .count()
}
