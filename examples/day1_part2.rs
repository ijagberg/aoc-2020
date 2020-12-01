use std::collections::HashMap;

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

    println!("{}", solve(nums, target).expect("unsolvable"));
}

/// Iterate over `nums`, finding the first 3-tuple of numbers that sum up to a given `target`
/// # Returns
/// The product of the tuple of numbers, or () on failure (no 3-tuple in `nums` sum up to `target`)
fn solve(nums: Vec<u32>, target: u32) -> Result<u32, ()> {
    // create and populate a map of (i+j) -> (i,j)
    // to allow for easier lookup
    let mut num_map = HashMap::new();
    for i in 0..nums.len() {
        let num_i = nums[i];
        for j in i + 1..nums.len() {
            let num_j = nums[j];
            num_map.insert(num_i + num_j, (num_i, num_j));
        }
    }

    // for each number, check if the target diff (target - num) has been found already in the map
    // if it has been found, the product is num*i*j
    for num in nums {
        let target_diff = target - num;
        if let Some(&(i, j)) = num_map.get(&target_diff) {
            return Ok(i * j * num);
        }
    }

    Err(())
}
