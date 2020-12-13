use std::cmp;

use cmp::Ordering;

pub struct XmasEncryption {
    numbers: Vec<u64>,
}

impl XmasEncryption {
    pub fn new(numbers: Vec<u64>) -> Self {
        Self { numbers }
    }

    /// Finds the first number in `self.numbers` that is not the sum of two previous numbers (skipping the preamble)
    fn find_invalid_number(&self, preamble_len: usize, window_len: usize) -> Option<u64> {
        'n: for idx in preamble_len..self.numbers.len() {
            let &n = &self.numbers[idx];
            for first in idx - window_len..idx {
                for second in idx - window_len..idx {
                    if first == second {
                        continue;
                    } else if self.numbers[first] + self.numbers[second] == n {
                        continue 'n;
                    }
                }
            }
            return Some(n);
        }

        None
    }

    fn find_contiguous_sum(&self, target_sum: u64) -> Option<Vec<u64>> {
        for start in 0..self.numbers.len() {
            let mut sum = self.numbers[start];
            for next in start + 1..self.numbers.len() {
                sum += self.numbers[next];
                match sum.cmp(&target_sum) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        return Some(self.numbers[start..=next].iter().copied().collect());
                    }
                    Ordering::Greater => {
                        break;
                    }
                }
            }
        }

        None
    }
}

pub fn solve_part1_from_file(file: &str, preamble_len: usize, window_len: usize) -> u64 {
    let encr = get_xmas_encr_from_file(file);

    encr.find_invalid_number(preamble_len, window_len).unwrap()
}

pub fn solve_part2_from_file(file: &str, target_sum: u64) -> u64 {
    let encr = get_xmas_encr_from_file(file);

    let numbers = encr.find_contiguous_sum(target_sum).unwrap();

    let (smallest, largest) = (
        numbers.iter().copied().min().unwrap(),
        numbers.iter().copied().max().unwrap(),
    );

    smallest + largest
}

fn get_xmas_encr_from_file(file: &str) -> XmasEncryption {
    let contents = std::fs::read_to_string(file).unwrap();

    let numbers = contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().parse().unwrap())
        .collect();
    XmasEncryption::new(numbers)
}