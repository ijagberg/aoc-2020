use std::collections::{HashMap, HashSet};

pub struct Adapters {
    bag: Vec<JoltsRating>,
    set: HashSet<JoltsRating>,
    outlet_jolts: JoltsRating,
}

impl Adapters {
    fn new(mut bag: Vec<JoltsRating>) -> Self {
        assert!(!bag.is_empty());

        bag.sort();

        let outlet_jolts = bag.iter().max().unwrap().rating() + 3; // safe to unwrap because we asserted bag is not empty earlier

        let outlet_jolts = JoltsRating::new(outlet_jolts);

        let set = bag.iter().copied().collect();

        Self {
            bag,
            outlet_jolts,
            set,
        }
    }

    fn bag(&self) -> &Vec<JoltsRating> {
        &self.bag
    }

    pub fn outlet_jolts(&self) -> u64 {
        self.outlet_jolts.rating()
    }

    fn set(&self) -> &HashSet<JoltsRating> {
        &self.set
    }

    fn arrangements_from(
        &self,
        start: JoltsRating,
        memo: &mut HashMap<JoltsRating, Option<u64>>,
    ) -> Option<u64> {
        let possible_next: Vec<JoltsRating> = (start.rating() + 1..=start.rating() + 3)
            .map(JoltsRating::new)
            .filter(|j| self.set().contains(&j))
            .collect();

        if possible_next.is_empty() {
            if self.outlet_jolts() - start.rating() <= 3 {
                Some(1)
            } else {
                None
            }
        } else {
            let mut a_out = 0;
            for n in possible_next {
                if let Some(memoized) = memo.get(&n) {
                    a_out += memoized.unwrap_or(0);
                } else if let Some(a) = self.arrangements_from(n, memo) {
                    memo.insert(n, Some(a));
                    a_out += a;
                } else {
                    memo.insert(n, None);
                }
            }

            Some(a_out)
        }
    }
}

pub fn solve_part1_from_file(file: &str) -> u32 {
    let adapters = get_adapters_from_file(file);

    let mut diff_of_1 = 1; // start to first
    let mut diff_of_3 = 1; // last to end

    for w in adapters.bag().windows(2) {
        let (from, to) = (w[0], w[1]);

        let diff = to.rating() - from.rating();
        if diff == 1 {
            diff_of_1 += 1;
        } else if diff == 3 {
            diff_of_3 += 1;
        }
    }

    diff_of_1 * diff_of_3
}

pub fn solve_part2_from_file(file: &str) -> u64 {
    let adapters = get_adapters_from_file(file);

    let mut memo = HashMap::new();
    adapters
        .arrangements_from(JoltsRating::new(0), &mut memo)
        .unwrap()
}

fn get_adapters_from_file(file: &str) -> Adapters {
    let contents = std::fs::read_to_string(file).unwrap();
    let ratings: Vec<JoltsRating> = contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| JoltsRating::new(l.parse().unwrap()))
        .collect();

    Adapters::new(ratings)
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct JoltsRating(u64);

impl JoltsRating {
    fn new(v: u64) -> Self {
        JoltsRating(v)
    }

    fn rating(&self) -> u64 {
        self.0
    }
}
