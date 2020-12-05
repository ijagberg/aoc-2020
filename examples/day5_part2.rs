use std::str::FromStr;

use aoc_2020::{self, seating::Seating};

fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    let lines: Vec<String> = aoc_2020::read_lines_from_file(&input_file)
        .unwrap()
        .map(|l| l.unwrap())
        .collect();

    println!(
        "{}",
        my_seat_id(
            lines
                .into_iter()
                .map(|p| Seating::from_str(&p).unwrap())
                .collect()
        )
    );
}

fn my_seat_id(passes: Vec<Seating>) -> u32 {
    let mut ids: Vec<_> = passes.into_iter().map(|s| s.id()).collect();

    ids.sort();

    for w in ids.windows(2) {
        let (before, after) = (w[0], w[1]);
        if after - before == 2 {
            return after - 1;
        }
    }

    panic!("unsolvable")
}
