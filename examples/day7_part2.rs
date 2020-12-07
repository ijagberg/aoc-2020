use aoc_2020;

fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    println!("{}", solve_part2_from_input_file(&input_file));
}

pub fn solve_part2_from_input_file(file: &str) -> u32 {
    let rules = aoc_2020::bags::get_rules_from_file(file);

    rules.count_bags_inside("shiny gold")
}
