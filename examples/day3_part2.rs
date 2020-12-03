use aoc_2020;

fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    let lines: Vec<String> = aoc_2020::read_lines_from_file(&input_file)
        .unwrap()
        .map(|l| l.unwrap())
        .collect();

    println!("{}", count_tree_collisions(lines));
}

fn count_tree_collisions(lines: Vec<String>) -> usize {
    const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let map = aoc_2020::grid::parse_toboggan_map(&lines);

    SLOPES
        .iter()
        .map(|&(right, down)| map.count_tree_collision(0, 0, right, down))
        .product()
}
