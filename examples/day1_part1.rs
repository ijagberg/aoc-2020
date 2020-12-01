use std::collections::HashSet;

use aoc_2020::read_lines_from_file;

fn main() {
    let input_file = std::env::args().nth(1).unwrap();
    let target = std::env::args()
        .nth(2)
        .map(|s| s.parse().unwrap())
        .unwrap_or(2020);

    let lines: Vec<String> = read_lines_from_file(&input_file)
        .unwrap()
        .map(|l| l.unwrap())
        .collect();

    let nums = lines.into_iter().map(|l| l.parse().unwrap()).collect();

    println!("{}", solve(nums, target).unwrap());
}

/// Iterate over `nums`, finding the first pair of numbers that sum up to a given `target`
/// # Returns
/// The product of the pair of numbers, or () on failure (no pair in `nums` sum up to `target`)
fn solve(nums: Vec<u32>, target: u32) -> Result<u32, ()> {
    let mut num_set = HashSet::new();

    for num in nums {
        let target_diff = target - num;
        if num_set.contains(&target_diff) {
            return Ok(num * target_diff);
        } else {
            num_set.insert(num);
        }
    }

    Err(())
}
