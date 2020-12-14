use std::{collections::HashMap, str::FromStr};

use bitvec::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Bit {
    One,
    Zero,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    SetMask(Vec<(usize, Option<Bit>)>),
    SetMem(usize, usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" = ").collect();
        if parts.len() != 2 {
            return Err(());
        }

        match &parts[0][..3] {
            "mem" => {
                let lbrack_idx = parts[0].find("[").expect("no left bracket");
                let rbrack_idx = parts[0].find("]").expect("no right bracket");
                let addr = parts[0][lbrack_idx + 1..rbrack_idx]
                    .parse()
                    .expect("element between left and right bracket is not an integer");
                let val = parts[1]
                    .parse()
                    .expect("element on right side of equal sign is not an integer");
                Ok(Instruction::SetMem(addr, val))
            }
            "mas" => Ok(Instruction::SetMask(parse_mask(parts[1]))),
            _ => Err(()),
        }
    }
}

pub fn solve_day14_part1_from_file(file: &str) -> usize {
    let contents = std::fs::read_to_string(file).unwrap();

    let mut mask = Vec::new();
    let mut memory = HashMap::new();
    for line in contents.split('\n').filter(|l| !l.is_empty()) {
        let instr = Instruction::from_str(line).expect("could not parse instruction");
        match instr {
            Instruction::SetMask(new_mask) => mask = new_mask,
            Instruction::SetMem(addr, val) => {
                let masked_val = apply_mask_v1(val, &mask);
                memory.insert(addr, masked_val);
            }
        }
    }

    memory.values().sum()
}

pub fn solve_day14_part2_from_file(file: &str) -> usize {
    let contents = std::fs::read_to_string(file).unwrap();

    let mut mask = Vec::new();
    let mut memory = HashMap::new();
    for line in contents.split('\n').filter(|l| !l.is_empty()) {
        let instr = Instruction::from_str(line).expect("could not parse instruction");
        match instr {
            Instruction::SetMask(new_mask) => mask = new_mask,
            Instruction::SetMem(addr, val) => {
                let addrs = apply_mask_v2(addr, &mask);
                for addr in addrs {
                    memory.insert(addr, val);
                }
            }
        }
    }

    memory.values().sum()
}

fn apply_mask_v1(mut val: usize, mask: &[(usize, Option<Bit>)]) -> usize {
    let bits = val.view_bits_mut::<Msb0>();

    for &(idx, bit) in mask {
        if let Some(bit) = bit {
            bits.set(
                idx,
                match bit {
                    Bit::One => true,
                    Bit::Zero => false,
                },
            );
        }
    }

    bits_to_usize(&bits)
}

fn bits_to_usize(bits: &BitSlice<Msb0>) -> usize {
    bits.iter()
        .rev()
        .fold((Some(1_usize), 0), |(pow, sum), &b| {
            if let Some(pow) = pow {
                if b {
                    (pow.checked_mul(2_usize), sum + pow)
                } else {
                    (pow.checked_mul(2_usize), sum)
                }
            } else {
                // pow * 2 would overflow, because we are at the final index of bits
                (pow, sum)
            }
        })
        .1
}

fn apply_mask_v2(addr: usize, mask: &[(usize, Option<Bit>)]) -> Vec<usize> {
    let mut addrs = Vec::new();

    let bits = addr.view_bits::<Msb0>();

    let floating_idxs = mask.iter().filter(|(_idx, b)| b.is_none()).count();

    let combinations: Vec<_> = combinations(floating_idxs);

    for comb in combinations {
        let mut float_count = 0;
        let mut bits_replaced = bits.clone().to_bitvec();
        for (idx, replacement) in mask {
            if let Some(repl) = replacement {
                if let Bit::One = repl {
                    bits_replaced.set(*idx, true);
                }
            } else {
                bits_replaced.set(*idx, comb[float_count]);
                float_count += 1;
            }
        }
        addrs.push(bits_to_usize(bits_replaced.as_bitslice()));
    }

    addrs
}

fn combinations(v: usize) -> Vec<BitVec> {
    (0..2_usize.pow(v as u32))
        .map(|n| n.view_bits::<Lsb0>().to_bitvec())
        .collect()
}

fn parse_mask(s: &str) -> Vec<(usize, Option<Bit>)> {
    s.chars()
        .enumerate()
        .map(|(idx, c)| match c {
            '0' => (idx + (64 - 36), Some(Bit::Zero)),
            '1' => (idx + (64 - 36), Some(Bit::One)),
            _ => (idx + (64 - 36), None),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Bit::*;

    #[test]
    fn apply_mask_test() {
        assert_eq!(
            apply_mask_v1(11, &vec![(57, Some(One)), (62, Some(Zero))]),
            73
        );
    }
}
