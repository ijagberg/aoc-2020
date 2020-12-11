use std::{convert::TryFrom, fmt::Display};

use grid::{Grid, GridIndex};

const STEPS: [(isize, isize); 8] = [
    (0, 1),   // up
    (1, 1),   // up-right
    (1, 0),   // right
    (1, -1),  // down-right
    (0, -1),  // down
    (-1, -1), // down-left
    (-1, 0),  // left
    (-1, 1),  // up-left
];

#[derive(Clone, PartialEq, Eq)]
pub struct FerrySeating {
    grid: Grid<Seat>,
}

impl FerrySeating {
    fn new(grid: Grid<Seat>) -> Self {
        Self { grid }
    }

    pub fn count_occupied_seats(&self) -> usize {
        self.grid
            .cell_iter()
            .filter(|c| matches!(c, Seat::Occupied))
            .count()
    }

    fn neighbors(&self, idx: GridIndex) -> Vec<(GridIndex, &Seat)> {
        let neighbors = STEPS
            .iter()
            .filter_map(|&(col_offset, row_offset)| self.add_to_idx(idx, (col_offset, row_offset)))
            .map(|idx| (idx, &self.grid[idx]))
            .collect();

        neighbors
    }

    fn step_until_seat(
        &self,
        mut start: GridIndex,
        (col_step, row_step): (isize, isize),
    ) -> Option<(GridIndex, &Seat)> {
        loop {
            if let Some(to_idx) = self.add_to_idx(start, (col_step, row_step)) {
                match self.seat_at(to_idx).unwrap() {
                    e @ Seat::Empty => return Some((to_idx, &e)),
                    o @ Seat::Occupied => return Some((to_idx, &o)),
                    Seat::Floor => {}
                }
                start = to_idx;
            } else {
                return None;
            }
        }
    }

    fn seats_visible_from(&self, idx: GridIndex) -> Vec<(GridIndex, &Seat)> {
        let mut visible_seats = Vec::new();
        for &(col_step, row_step) in &STEPS {
            if let Some(visible_seat) = self.step_until_seat(idx, (col_step, row_step)) {
                visible_seats.push(visible_seat)
            }
        }
        visible_seats
    }

    fn seat_at(&self, idx: GridIndex) -> Option<&Seat> {
        self.grid.get(idx)
    }

    fn step_once_neighbor(&self) -> FerrySeating {
        let mut next = self.grid.clone();

        for row in 0..self.grid.height() {
            for col in 0..self.grid.width() {
                let idx = GridIndex::new(row, col);
                let neighbors = self.neighbors(idx);
                match self.seat_at(idx).unwrap() {
                    Seat::Empty => {
                        if neighbors
                            .iter()
                            .all(|(_idx, seat)| matches!(seat, Seat::Empty | Seat::Floor))
                        {
                            next[idx] = Seat::Occupied;
                        }
                    }
                    Seat::Occupied => {
                        if neighbors
                            .iter()
                            .filter(|(_idx, seat)| matches!(seat, Seat::Occupied))
                            .count()
                            >= 4
                        {
                            next[idx] = Seat::Empty;
                        }
                    }
                    Seat::Floor => {}
                }
            }
        }

        FerrySeating::new(next)
    }

    fn step_once_vision(&self) -> FerrySeating {
        let mut next = self.grid.clone();

        for row in 0..self.grid.height() {
            for col in 0..self.grid.width() {
                let idx = GridIndex::new(row, col);
                let visible_seats = self.seats_visible_from(idx);
                match self.seat_at(idx).unwrap() {
                    Seat::Empty => {
                        if visible_seats
                            .iter()
                            .all(|(_idx, seat)| matches!(seat, Seat::Empty | Seat::Floor))
                        {
                            next[idx] = Seat::Occupied;
                        }
                    }
                    Seat::Occupied => {
                        if visible_seats
                            .iter()
                            .filter(|(_idx, seat)| matches!(seat, Seat::Occupied))
                            .count()
                            >= 5
                        {
                            next[idx] = Seat::Empty;
                        }
                    }
                    Seat::Floor => {}
                }
            }
        }

        FerrySeating::new(next)
    }

    pub fn step_until_stable_neighbor(&self) -> FerrySeating {
        let mut curr = self.clone();
        for _iteration in 1.. {
            let next = curr.step_once_neighbor();
            if curr == next {
                return next;
            } else {
                curr = next;
            }
        }

        unreachable!()
    }

    pub fn step_until_stable_vision(&self) -> FerrySeating {
        let mut curr = self.clone();
        for _iteration in 1.. {
            let next = curr.step_once_vision();
            if curr == next {
                return next;
            } else {
                curr = next;
            }
        }

        unreachable!()
    }

    fn add_to_idx(
        &self,
        idx: GridIndex,
        (col_offset, row_offset): (isize, isize),
    ) -> Option<GridIndex> {
        let eval_col = if col_offset < 0 {
            if idx.column() > 0 {
                Some(idx.column() - col_offset.abs() as usize)
            } else {
                None
            }
        } else if col_offset > 0 {
            if idx.column() < self.grid.width() - 1 {
                Some(idx.column() + col_offset as usize)
            } else {
                None
            }
        } else {
            Some(idx.column())
        };
        let eval_row = if row_offset < 0 {
            if idx.row() > 0 {
                Some(idx.row() - row_offset.abs() as usize)
            } else {
                None
            }
        } else if row_offset > 0 {
            if idx.row() < self.grid.height() - 1 {
                Some(idx.row() + row_offset as usize)
            } else {
                None
            }
        } else {
            Some(idx.row())
        };

        if let (Some(col), Some(row)) = (eval_col, eval_row) {
            Some(GridIndex::new(row, col))
        } else {
            None
        }
    }
}

pub fn parse_ferry_seating_from_input_file(file: &str) -> FerrySeating {
    let contents = std::fs::read_to_string(file).unwrap();

    let lines: Vec<_> = contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut grid_data = Vec::new();
    for line in lines {
        let mut seats: Vec<Seat> = line.chars().map(|c| Seat::try_from(c).unwrap()).collect();
        grid_data.append(&mut seats);
    }

    let grid = Grid::new(width, height, grid_data);

    FerrySeating::new(grid)
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Seat::Empty => "L",
            Seat::Occupied => "#",
            Seat::Floor => ".",
        };

        write!(f, "{}", output)
    }
}

impl TryFrom<char> for Seat {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'L' => Seat::Empty,
            '.' => Seat::Floor,
            '#' => Seat::Occupied,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ferry_seating_from_input_file_test() {
        let ferry = parse_ferry_seating_from_input_file("inputs/day11_example.txt");

        assert_eq!(ferry.grid.width(), 10);
        assert_eq!(ferry.grid.height(), 10);
    }
}
