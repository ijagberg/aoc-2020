fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    println!(
        "{}",
        aoc_2020::xmas::solve_part1_from_file(&input_file, 25, 25)
    );
}
