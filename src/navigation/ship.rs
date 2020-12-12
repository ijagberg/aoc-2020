use super::*;

#[derive(Debug)]
pub struct Ship {
    facing_direction: Direction,
    position: Position,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            facing_direction: Direction::East,
            position: Position::default(),
        }
    }

    pub fn set_pos(&mut self, (x, y): (isize, isize)) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn x_pos(&self) -> isize {
        self.position.x
    }

    pub fn y_pos(&self) -> isize {
        self.position.y
    }

    fn handle_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::MoveNorth(v) => self.position.y += v as isize,
            Instruction::MoveEast(v) => self.position.x += v as isize,
            Instruction::MoveSouth(v) => self.position.y -= v as isize,
            Instruction::MoveWest(v) => self.position.x -= v as isize,
            Instruction::TurnLeft(v) => self.turn_left(v),
            Instruction::TurnRight(v) => self.turn_right(v),
            Instruction::MoveForward(v) => self.move_forward(v),
        }
    }

    fn move_forward(&mut self, v: u32) {
        match self.facing_direction {
            Direction::North => self.position.y += v as isize,
            Direction::East => self.position.x += v as isize,
            Direction::South => self.position.y -= v as isize,
            Direction::West => self.position.x -= v as isize,
        }
    }

    fn turn_left(&mut self, mut degrees: u32) {
        assert!(degrees % 90 == 0);

        degrees %= 360;
        let steps = degrees / 90;

        for _ in 0..steps {
            match self.facing_direction {
                Direction::North => self.facing_direction = Direction::West,
                Direction::East => self.facing_direction = Direction::North,
                Direction::South => self.facing_direction = Direction::East,
                Direction::West => self.facing_direction = Direction::South,
            }
        }
    }

    fn turn_right(&mut self, mut degrees: u32) {
        assert!(degrees % 90 == 0);

        degrees %= 360;
        let steps = degrees / 90;

        for _ in 0..steps {
            match self.facing_direction {
                Direction::North => self.facing_direction = Direction::East,
                Direction::East => self.facing_direction = Direction::South,
                Direction::South => self.facing_direction = Direction::West,
                Direction::West => self.facing_direction = Direction::North,
            }
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self::new()
    }
}

pub fn solve_day12_part1_from_file(file: &str) -> isize {
    let instrs = read_instr_from_file(file);

    let mut ship = Ship::new();

    for instr in instrs {
        ship.handle_instruction(instr);
    }

    let x_dist = ship.position.x.abs();
    let y_dist = ship.position.y.abs();

    x_dist + y_dist
}
