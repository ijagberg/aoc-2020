use std::convert::TryFrom;

use grid::Grid;

pub struct FerrySeating {
    grid: Grid<Seat>,
}

impl FerrySeating {
    fn new(grid: Grid<Seat>) -> Self {
        Self { grid }
    }
}

fn parse_ferry_seating_from_input_file(file: &str) -> FerrySeating {
    let contents = std::fs::read_to_string(file).unwrap();

    let lines: Vec<_> = contents.split('\n').filter(|l| !l.is_empty()).map(|l| l.trim()).collect();

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

enum Seat {
    Empty,
    Occupied,
    Floor,
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
