use std::collections::HashSet;

use aoc_2020::read_lines_from_file;

fn main() {
    let input_file = std::env::args().nth(1).unwrap();

    let lines: Vec<String> = read_lines_from_file(&input_file)
        .unwrap()
        .map(|l| l.unwrap())
        .collect();

    let nums = lines.into_iter().map(|l| l.parse().unwrap()).collect();

    println!("{}", solve(nums).unwrap());
}

fn solve(nums: Vec<u32>) -> Result<u32, ()> {
    const TARGET_SUM: u32 = 2020;
    let mut num_set = HashSet::new();

    for num in nums {
        let target_diff = TARGET_SUM - num;
        if num_set.contains(&target_diff) {
            return Ok(num * target_diff);
        } else {
            num_set.insert(num);
        }
    }

    Err(())
}
