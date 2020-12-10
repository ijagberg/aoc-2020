fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    println!("{}", aoc_2020::jolts::solve_part2_from_file(&input_file));
}
