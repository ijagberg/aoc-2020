pub mod ship;
pub mod waypoint;

use std::str::FromStr;

#[derive(Default, Copy, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    MoveNorth(u32),
    MoveEast(u32),
    MoveSouth(u32),
    MoveWest(u32),
    TurnLeft(u32),
    TurnRight(u32),
    MoveForward(u32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(());
        }

        let value: u32 = s[1..].parse().map_err(|_| ())?;

        let action = &s[..1];

        Ok(match action {
            "N" => Instruction::MoveNorth(value),
            "E" => Instruction::MoveEast(value),
            "S" => Instruction::MoveSouth(value),
            "W" => Instruction::MoveWest(value),
            "L" => Instruction::TurnLeft(value),
            "R" => Instruction::TurnRight(value),
            "F" => Instruction::MoveForward(value),
            _ => return Err(()),
        })
    }
}

fn read_instr_from_file(file: &str) -> Vec<Instruction> {
    let content = std::fs::read_to_string(file).unwrap();

    content
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction_test() {
        assert_eq!(
            Instruction::from_str("N3").unwrap(),
            Instruction::MoveNorth(3)
        );
    }
}
