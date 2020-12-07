use aoc_2020;

fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    println!("{}", aoc_2020::bags::solve_part1_from_input_file(&input_file));
}
