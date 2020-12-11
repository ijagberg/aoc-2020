pub mod bags;
pub mod grid;
pub mod handheld;
pub mod jolts;
pub mod passport;
pub mod password;
pub mod questions;
pub mod seating;
pub mod xmas;

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Returns an iterator over the lines of a given file
pub fn read_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    fn read_lines_from_file(file: &str) -> Vec<String> {
        let file = File::open(file).unwrap();
        let lines: Vec<String> = io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect();

        lines
    }

    mod day1 {
        use super::*;

        fn solve_day1_part1_from_file(file: &str, target: u32) -> Option<u32> {
            let nums: Vec<u32> = read_lines_from_file(file)
                .into_iter()
                .map(|l| l.parse().unwrap())
                .collect();

            let mut num_set = HashSet::new();

            for num in nums {
                let target_diff = target - num;
                if num_set.contains(&target_diff) {
                    return Some(num * target_diff);
                } else {
                    num_set.insert(num);
                }
            }
            None
        }

        fn solve_day1_part2_from_file(file: &str, target: u32) -> Option<u32> {
            let nums: Vec<u32> = read_lines_from_file(&file)
                .into_iter()
                .map(|l| l.parse().unwrap())
                .collect();

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
                    return Some(i * j * num);
                }
            }

            None
        }

        #[test]
        fn day1_part1() {
            assert_eq!(
                solve_day1_part1_from_file("inputs/day1_example.txt", 2020).unwrap(),
                514579
            );
            assert_eq!(
                solve_day1_part1_from_file("inputs/day1.txt", 2020).unwrap(),
                712075
            );
        }

        #[test]
        fn day1_part2() {
            assert_eq!(
                solve_day1_part2_from_file("inputs/day1_example.txt", 2020).unwrap(),
                241861950
            );
            assert_eq!(
                solve_day1_part2_from_file("inputs/day1.txt", 2020).unwrap(),
                145245270
            );
        }
    }

    mod day2 {
        use crate::password::Policy;

        use super::*;

        fn solve_day2_part1_from_file(file: &str) -> usize {
            read_lines_from_file(file)
                .into_iter()
                .map(|l| {
                    let (policy, password) = password::parse_sled_policy_and_password(&l).unwrap();
                    policy.validate(&password)
                })
                .filter(|&b| b == true)
                .count()
        }

        fn solve_day2_part2_from_file(file: &str) -> usize {
            read_lines_from_file(file)
                .into_iter()
                .map(|l| {
                    let (policy, password) =
                        password::parse_toboggan_policy_and_password(&l).unwrap();
                    println!("{:?}: {:?}", policy, password);
                    policy.validate(&password)
                })
                .filter(|&b| b == true)
                .count()
        }

        #[test]
        fn day2_part1() {
            assert_eq!(solve_day2_part1_from_file("inputs/day2_example.txt"), 2);
            assert_eq!(solve_day2_part1_from_file("inputs/day2.txt"), 640);
        }

        #[test]
        fn day2_part2() {
            assert_eq!(solve_day2_part2_from_file("inputs/day2_example.txt"), 1);
            assert_eq!(solve_day2_part2_from_file("inputs/day2.txt"), 472);
        }
    }

    mod day3 {
        use super::*;

        fn solve_day3_part1_from_file(file: &str) -> usize {
            let lines = read_lines_from_file(&file);
            let map = grid::parse_toboggan_map(&lines);
            map.count_tree_collision(0, 0, 3, 1)
        }

        fn solve_day3_part2_from_file(file: &str) -> usize {
            let lines = read_lines_from_file(&file);
            const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
            let map = grid::parse_toboggan_map(&lines);

            SLOPES
                .iter()
                .map(|&(right, down)| map.count_tree_collision(0, 0, right, down))
                .product()
        }

        #[test]
        fn day3_part1() {
            assert_eq!(solve_day3_part1_from_file("inputs/day3_example.txt"), 7);
            assert_eq!(solve_day3_part1_from_file("inputs/day3.txt"), 259);
        }

        #[test]
        fn day3_part2() {
            assert_eq!(solve_day3_part2_from_file("inputs/day3_example.txt"), 336);
            assert_eq!(solve_day3_part2_from_file("inputs/day3.txt"), 2224913600);
        }
    }

    mod day4 {
        use super::*;

        fn solve_day4_part1_from_file(file: &str) -> usize {
            passport::read_valid_passports_from_file(file).len()
        }

        fn solve_day4_part2_from_file(file: &str) -> usize {
            passport::read_valid_passports_from_file(file).len()
        }

        #[test]
        fn day4_part1() {
            assert_eq!(solve_day4_part1_from_file("inputs/day4_example.txt"), 2);
            // assert_eq!(solve_day4_part1_from_file("inputs/day4.txt"), 233); // TODO
        }

        #[test]
        fn day4_part2() {
            assert_eq!(solve_day4_part2_from_file("inputs/day4_example.txt"), 2);
            assert_eq!(solve_day4_part2_from_file("inputs/day4.txt"), 111);
        }
    }

    mod day5 {
        use std::str::FromStr;

        use crate::seating::airplane::Seating;

        use super::*;

        fn solve_day5_part1_from_file(file: &str) -> u32 {
            read_lines_from_file(&file)
                .into_iter()
                .map(|p| Seating::from_str(&p).unwrap())
                .map(|s| s.id())
                .max()
                .unwrap()
        }

        fn solve_day5_part2_from_file(file: &str) -> u32 {
            let mut ids: Vec<_> = read_lines_from_file(&file)
                .into_iter()
                .map(|p| Seating::from_str(&p).unwrap())
                .map(|s| s.id())
                .collect();

            ids.sort();

            for w in ids.windows(2) {
                let (before, after) = (w[0], w[1]);
                if after - before == 2 {
                    return after - 1;
                }
            }

            unreachable!()
        }

        #[test]
        fn day5_part1() {
            assert_eq!(solve_day5_part1_from_file("inputs/day5.txt"), 933);
        }

        #[test]
        fn day5_part2() {
            assert_eq!(solve_day5_part2_from_file("inputs/day5.txt"), 711);
        }
    }

    mod day6 {
        use super::*;

        fn solve_day6_part1_from_file(file: &str) -> usize {
            let lines = read_lines_from_file(&file);
            questions::count_union_yes(lines)
        }

        fn solve_day6_part2_from_file(file: &str) -> usize {
            let lines = read_lines_from_file(&file);
            questions::count_intersection_yes(lines)
        }

        #[test]
        fn day6_part1() {
            assert_eq!(solve_day6_part1_from_file("inputs/day6_example.txt"), 11);
            assert_eq!(solve_day6_part1_from_file("inputs/day6.txt"), 6551);
        }

        #[test]
        fn day6_part2() {
            assert_eq!(solve_day6_part2_from_file("inputs/day6_example.txt"), 6);
            assert_eq!(solve_day6_part2_from_file("inputs/day6.txt"), 3358);
        }
    }

    mod day7 {
        use super::*;

        fn solve_day7_part1_from_file(file: &str) -> u32 {
            let rules = bags::get_rules_from_file(file);

            rules.how_many_can_contain("shiny gold")
        }

        fn solve_day7_part2_from_file(file: &str) -> u32 {
            let rules = bags::get_rules_from_file(file);

            rules.count_bags_inside("shiny gold")
        }

        #[test]
        fn day7_part1() {
            assert_eq!(solve_day7_part1_from_file("inputs/day7_example.txt"), 4);
            assert_eq!(solve_day7_part1_from_file("inputs/day7.txt"), 335);
        }

        #[test]
        fn day7_part2() {
            assert_eq!(solve_day7_part2_from_file("inputs/day7_example.txt"), 32);
            assert_eq!(solve_day7_part2_from_file("inputs/day7.txt"), 2431);
        }
    }

    mod day8 {
        use super::*;

        fn solve_day8_part1_from_file(file: &str) -> i32 {
            let mut handheld = handheld::parse_handheld_from_input_file(file);

            handheld.step_until_termination().unwrap_err();

            handheld.accumulator()
        }

        fn solve_day8_part2_from_file(file: &str) -> i32 {
            handheld::fix_inf_loop(handheld::parse_handheld_from_input_file(file))
        }

        #[test]
        fn day8_part1() {
            assert_eq!(solve_day8_part1_from_file("inputs/day8_example.txt"), 5);
            assert_eq!(solve_day8_part1_from_file("inputs/day8.txt"), 1489);
        }

        #[test]
        fn day8_part2() {
            assert_eq!(solve_day8_part2_from_file("inputs/day8_example.txt"), 8);
            assert_eq!(solve_day8_part2_from_file("inputs/day8.txt"), 1539);
        }
    }

    mod day9 {
        use super::*;

        fn solve_day9_part1_from_file(file: &str, preamble_len: usize, window_len: usize) -> u64 {
            xmas::solve_part1_from_file(file, preamble_len, window_len)
        }

        fn solve_day9_part2_from_file(file: &str, target_sum: u64) -> u64 {
            xmas::solve_part2_from_file(file, target_sum)
        }

        #[test]
        fn day9_part1() {
            assert_eq!(
                solve_day9_part1_from_file("inputs/day9_example.txt", 5, 5),
                127
            );
            assert_eq!(
                solve_day9_part1_from_file("inputs/day9.txt", 25, 25),
                466456641
            );
        }

        #[test]
        fn day9_part2() {
            assert_eq!(
                solve_day9_part2_from_file("inputs/day9_example.txt", 127),
                62
            );
            assert_eq!(
                solve_day9_part2_from_file("inputs/day9.txt", 466456641),
                55732936
            );
        }
    }

    mod day10 {
        use super::*;

        fn solve_day10_part1_from_file(file: &str) -> u32 {
            jolts::solve_part1_from_file(file)
        }

        fn solve_day10_part2_from_file(file: &str) -> u64 {
            jolts::solve_part2_from_file(file)
        }

        #[test]
        fn day10_part1() {
            assert_eq!(
                solve_day10_part1_from_file("inputs/day10_example.txt"),
                22 * 10
            );
            assert_eq!(solve_day10_part1_from_file("inputs/day10.txt"), 2201);
        }

        #[test]
        fn day10_part2() {
            assert_eq!(
                solve_day10_part2_from_file("inputs/day10_example.txt"),
                19208
            );
            assert_eq!(
                solve_day10_part2_from_file("inputs/day10.txt"),
                169255295254528
            );
        }
    }
}
