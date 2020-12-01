use std::collections::HashMap;

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
    let mut num_map = HashMap::new();

    for i in 0..nums.len() {
        let num_i = nums[i];
        for j in i + 1..nums.len() {
            let num_j = nums[j];
            num_map.insert(num_i + num_j, (num_i, num_j));
        }
    }

    for num in nums {
        let target_diff = TARGET_SUM - num;
        if let Some(&(i, j)) = num_map.get(&target_diff) {
            return Ok(i * j * num);
        }
    }

    Err(())
}
