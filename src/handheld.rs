use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub struct Handheld {
    instructions: Vec<Instruction>,
    curr: usize,
    accumulator: i32,
}

impl Handheld {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            curr: 0,
            accumulator: 0,
        }
    }

    fn is_terminated(&self) -> bool {
        self.curr() >= self.instructions().len()
    }

    fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    fn curr(&self) -> usize {
        self.curr
    }

    pub fn accumulator(&self) -> i32 {
        self.accumulator
    }

    fn add_to_acc(&mut self, v: i32) {
        self.accumulator += v;
    }

    /// Executes the next instruction (starting at 0)
    pub fn step(&mut self) {
        let current_instruction = &self.instructions[self.curr];
        match (&current_instruction.op, current_instruction.arg) {
            (Operation::Acc, arg) => {
                self.add_to_acc(arg);
                self.curr += 1;
            }
            (Operation::Jmp, arg) => {
                if arg < 0 {
                    self.curr -= arg.abs() as usize;
                } else {
                    self.curr += arg as usize;
                }
            }
            (Operation::Nop, _arg) => {
                self.curr += 1;
            }
        }
    }

    /// Step forward through instructions until termination (next instruction does not exist)
    /// 
    /// # Returns
    /// * The value in the `accumulator` if termination was successful
    /// * An empty error if any instruction is executed twice (infinite loop)
    pub fn step_until_termination(&mut self) -> Result<i32, ()> {
        let mut already_run = HashSet::new();

        while !self.is_terminated() {
            if already_run.contains(&self.curr()) {
                return Err(());
            }
            already_run.insert(self.curr());
            self.step();
        }

        Ok(self.accumulator())
    }

    fn fix_inf_loop(&mut self) -> Result<(), ()> {
        // replace jmp with nop:
        for idx in 0..self.instructions().len() {
            let instr = &self.instructions()[idx];
            if instr.is_jmp() {
                let mut clone = self.clone();
                let new_instr = Instruction::nop(instr.arg());
                clone.set_instruction(idx, new_instr);

                if let Ok(_) = clone.step_until_termination() {
                    self.set_instruction(idx, new_instr);
                    return Ok(());
                }
            }
        }

        // replace nop with jmp:
        for idx in 0..self.instructions().len() {
            let instr = &self.instructions()[idx];
            if instr.is_nop() {
                let mut clone = self.clone();
                let new_instr = Instruction::jmp(instr.arg());

                clone.set_instruction(idx, new_instr);

                if let Ok(_) = clone.step_until_termination() {
                    self.set_instruction(idx, new_instr);
                    return Ok(());
                }
            }
        }

        Err(())
    }

    fn set_instruction(&mut self, idx: usize, instr: Instruction) {
        self.instructions[idx] = instr;
    }
}

pub fn parse_handheld_from_input_file(file: &str) -> Handheld {
    let content = std::fs::read_to_string(file).unwrap();

    let instructions: Vec<Instruction> = content
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|p| p.trim().parse().unwrap())
        .collect();

    Handheld::new(instructions)
}

pub fn fix_inf_loop(mut handheld: Handheld) -> i32 {
    handheld.fix_inf_loop().unwrap();
    handheld.step_until_termination().unwrap()
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Instruction {
    op: Operation,
    arg: i32,
}

impl Instruction {
    fn new(op: Operation, arg: i32) -> Self {
        Self { op, arg }
    }

    fn op(&self) -> Operation {
        self.op
    }

    fn arg(&self) -> i32 {
        self.arg
    }

    fn jmp(arg: i32) -> Instruction {
        Instruction::new(Operation::Jmp, arg)
    }

    fn acc(arg: i32) -> Instruction {
        Instruction::new(Operation::Acc, arg)
    }

    fn nop(arg: i32) -> Instruction {
        Instruction::new(Operation::Nop, arg)
    }

    fn is_jmp(&self) -> bool {
        matches!(self.op(), Operation::Jmp)
    }

    fn is_acc(&self) -> bool {
        matches!(self.op(), Operation::Acc)
    }

    fn is_nop(&self) -> bool {
        matches!(self.op(), Operation::Nop)
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // input like
        // "nop +4"

        let parts: Vec<&str> = s.split(" ").collect();

        if parts.len() != 2 {
            return Err(ParseInstructionError::InvalidFormat);
        }

        // parts[0]: "nop"
        // parts[1]: "+4"

        let op: Operation = parts[0]
            .parse()
            .map_err(|_| ParseInstructionError::InvalidOperation)?;
        let arg: i32 = parts[1]
            .parse()
            .map_err(|_| ParseInstructionError::InvalidArgument)?;

        Ok(Instruction::new(op, arg))
    }
}

#[derive(Debug)]
enum ParseInstructionError {
    InvalidFormat,
    InvalidOperation,
    InvalidArgument,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Operation {
    /// Add a value to the global accumulator
    Acc,
    /// Jump a given number of instructions relative to the current
    Jmp,
    /// Do nothing, go to next instruction
    Nop,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "nop" => Operation::Nop,
            "jmp" => Operation::Jmp,
            "acc" => Operation::Acc,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        assert_eq!(
            Instruction::from_str("nop +4").unwrap(),
            Instruction::new(Operation::Nop, 4)
        );

        assert_eq!(
            Instruction::from_str("acc -25").unwrap(),
            Instruction::new(Operation::Acc, -25)
        );
    }

    #[test]
    fn parse_handheld_from_input_file_test() {
        use Operation::*;

        assert_eq!(
            day8_example(),
            Handheld::new(vec![
                Instruction::new(Nop, 0),
                Instruction::new(Acc, 1),
                Instruction::new(Jmp, 4),
                Instruction::new(Acc, 3),
                Instruction::new(Jmp, -3),
                Instruction::new(Acc, -99),
                Instruction::new(Acc, 1),
                Instruction::new(Jmp, -4),
                Instruction::new(Acc, 6),
            ])
        );
    }

    #[test]
    fn find_inf_loop_test() {
        let mut handheld = day8_example();

        handheld.step_until_termination().unwrap_err(); // steps until terminated or inf loop is found

        assert_eq!(handheld.accumulator(), 5);
    }

    #[test]
    fn fix_inf_loop_test() {
        let handheld = day8_example();

        assert_eq!(fix_inf_loop(handheld), 8);
    }

    fn day8_example() -> Handheld {
        parse_handheld_from_input_file("inputs/day8_example.txt")
    }
}
