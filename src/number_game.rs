use std::collections::{hash_map::Entry, HashMap};

pub struct NumberGame {
    spoken_numbers: Vec<u32>,
    spoken_counts: HashMap<u32, (Vec<usize>, u32)>,
}

impl NumberGame {
    fn new(starting_numbers: Vec<u32>) -> Self {
        let spoken_numbers = Vec::new();
        let spoken_counts = HashMap::new();

        let mut s = NumberGame {
            spoken_counts,
            spoken_numbers,
        };

        for num in starting_numbers {
            s.speak_number(num);
        }

        s
    }

    fn play_for_n_turns(mut self, n: usize) -> Vec<u32> {
        let actual_turns = n - self.spoken_numbers.len();
        for _turn in 0..actual_turns {
            let last_spoken_number = self.last_spoken_number();

            let (idx, count) = self.get_idx_and_count(last_spoken_number);

            if *count == 1 {
                self.speak_number(0);
            } else {
                let diff = idx[idx.len() - 1] - idx[idx.len() - 2];
                self.speak_number(diff as u32);
            }
        }

        self.spoken_numbers
    }

    fn get_idx_and_count(&self, number: u32) -> &(Vec<usize>, u32) {
        self.spoken_counts.get(&number).unwrap()
    }

    fn last_spoken_number(&self) -> u32 {
        *self.spoken_numbers.last().unwrap()
    }

    fn speak_number(&mut self, number: u32) {
        self.spoken_numbers.push(number);
        match self.spoken_counts.entry(number) {
            Entry::Occupied(mut entry) => {
                let (indices, prev_count) = entry.get_mut();

                indices.push(self.spoken_numbers.len());
                *prev_count += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert((vec![self.spoken_numbers.len()], 1));
            }
        }
    }
}

pub fn solve_day15_part1_from_file(file: &str, turns: usize) -> u32 {
    let game = parse_number_game_from_file(file);

    let numbers = game.play_for_n_turns(turns);

    *numbers.last().unwrap()
}

fn parse_number_game_from_file(file: &str) -> NumberGame {
    let content = std::fs::read_to_string(file).unwrap();
    NumberGame::new(
        content
            .split(',')
            .filter(|l| !l.is_empty())
            .map(|l| l.trim().parse().unwrap())
            .collect(),
    )
}
