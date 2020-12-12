use super::{ship::Ship, Instruction, Position};

#[derive(Debug)]
pub struct ShipWithWaypoint {
    ship: Ship,
    waypoint: Position,
}

impl ShipWithWaypoint {
    fn new() -> Self {
        Self {
            ship: Ship::new(),
            waypoint: Position::new(10, 1),
        }
    }

    fn handle_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::MoveNorth(v) => self.waypoint.y += v as isize,
            Instruction::MoveEast(v) => self.waypoint.x += v as isize,
            Instruction::MoveSouth(v) => self.waypoint.y -= v as isize,
            Instruction::MoveWest(v) => self.waypoint.x -= v as isize,
            Instruction::TurnLeft(v) => self.rotate_waypoint_left(v),
            Instruction::TurnRight(v) => self.rotate_waypoint_right(v),
            Instruction::MoveForward(v) => self.move_toward_waypoint(v),
        }
    }

    fn move_toward_waypoint(&mut self, steps: u32) {
        for _ in 0..steps {
            let new_x = self.ship.x_pos() + self.waypoint.x;
            let new_y = self.ship.y_pos() + self.waypoint.y;
            self.ship.set_pos((new_x, new_y));
        }
    }

    fn rotate_waypoint_left(&mut self, mut degrees: u32) {
        assert!(degrees % 90 == 0);

        degrees = degrees % 360;
        let steps = degrees / 90;

        for _ in 0..steps {
            let wp = self.waypoint;
            self.waypoint.x = -wp.y;
            self.waypoint.y = wp.x;
        }
    }

    fn rotate_waypoint_right(&mut self, mut degrees: u32) {
        assert!(degrees % 90 == 0);

        degrees = degrees % 360;
        let steps = degrees / 90;

        for _ in 0..steps {
            let wp = self.waypoint;
            self.waypoint.x = wp.y;
            self.waypoint.y = -wp.x;
        }
    }
}

pub fn solve_day12_part2_from_file(file: &str) -> isize {
    let instrs = super::read_instr_from_file(file);

    let mut ship_with_waypoint = ShipWithWaypoint::new();

    for instr in instrs {
        ship_with_waypoint.handle_instruction(instr);
    }

    let x_dist = ship_with_waypoint.ship.x_pos().abs();
    let y_dist = ship_with_waypoint.ship.y_pos().abs();

    x_dist + y_dist
}
