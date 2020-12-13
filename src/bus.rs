pub fn solve_day13_part1_from_file(file: &str) -> i64 {
    let (dep, buses) = parse_departures_from_file(file);

    let (mut best_bus, mut best_wait_time) = (None, None);
    for bus in buses.into_iter().filter_map(|i| i) {
        if dep % bus == 0 {
            return bus;
        } else {
            let wait_time = (((dep / bus) + 1) * bus) - dep;

            if wait_time < best_wait_time.unwrap_or(i64::MAX) {
                best_wait_time = Some(wait_time);
                best_bus = Some(bus);
            }
        }
    }

    best_bus.unwrap() * best_wait_time.unwrap()
}

pub fn solve_day13_part2_from_file(file: &str) -> i64 {
    let (_dep, buses) = parse_departures_from_file(file);
    let mods = buses
        .iter()
        .enumerate()
        .filter_map(|(_idx, b)| match b {
            Some(b) => Some(*b),
            None => None,
        })
        .collect::<Vec<_>>();
    let res = buses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Some(b) => Some(b - i as i64),
            None => None,
        })
        .collect::<Vec<_>>();
    crt::chinese_remainder(&res, &mods).unwrap()
}

fn parse_departures_from_file(file: &str) -> (i64, Vec<Option<i64>>) {
    let lines: Vec<_> = std::fs::read_to_string(file)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.to_owned())
        .collect();
    let dep = lines[0].parse().unwrap();
    let buses = lines[1]
        .split(',')
        .map(|l| {
            if l == "x" {
                None
            } else {
                Some(l.parse().unwrap())
            }
        })
        .collect();

    (dep, buses)
}

/// Chinese remainder theorem, in Rust
///
/// Source: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
mod crt {
    fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = egcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }

    fn mod_inv(x: i64, n: i64) -> Option<i64> {
        let (g, x, _) = egcd(x, n);
        if g == 1 {
            Some((x % n + n) % n)
        } else {
            None
        }
    }

    pub fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
        let prod = modulii.iter().product::<i64>();
        let mut sum = 0;
        for (&residue, &modulus) in residues.iter().zip(modulii) {
            let p = prod / modulus;
            sum += residue * mod_inv(p, modulus)? * p
        }
        Some(sum % prod)
    }
}
