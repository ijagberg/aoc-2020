use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    hash::Hash,
};

use itertools::iproduct;
use simple_grid::Grid;

pub fn solve_day17_part1_from_file(file: &str) -> usize {
    let grid = parse_initial_state_from_file(file);

    let mut cubes = HashMap::new();
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            cubes.insert(Idx3::new(col as isize, row as isize, 0), grid[(col, row)]);
        }
    }

    let mut life = Life::new(cubes);

    for _ in 0..6 {
        life.step();
    }

    life.count_active()
}

pub fn solve_day17_part2_from_file(file: &str) -> usize {
    let grid = parse_initial_state_from_file(file);

    let mut cubes = HashMap::new();
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            cubes.insert(
                Idx4::new(col as isize, row as isize, 0, 0),
                grid[(col, row)],
            );
        }
    }

    let mut life = Life::new(cubes);
    for _ in 0..6 {
        life.step();
    }

    life.count_active()
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
struct Idx3(isize, isize, isize);

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
struct Idx4(isize, isize, isize, isize);

impl Idx3 {
    fn new(x: isize, y: isize, z: isize) -> Idx3 {
        Idx3(x, y, z)
    }
}

impl Idx4 {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Idx4 {
        Idx4(x, y, z, w)
    }
}

trait Neighbor: Copy {
    fn neighbors(&self) -> Vec<Self>;
}

impl Neighbor for Idx3 {
    fn neighbors(&self) -> Vec<Self> {
        const DIRS: [isize; 3] = [-1, 0, 1];

        let prod = iproduct!(&DIRS, &DIRS, &DIRS);
        let neigh: Vec<Idx3> = prod
            .filter_map(|(a, b, c)| {
                if a == &0 && b == &0 && c == &0 {
                    None
                } else {
                    Some(Idx3::new(self.0 + a, self.1 + b, self.2 + c))
                }
            })
            .collect();

        neigh
    }
}

impl Neighbor for Idx4 {
    fn neighbors(&self) -> Vec<Self> {
        const DIRS: [isize; 3] = [-1, 0, 1];

        let prod = iproduct!(&DIRS, &DIRS, &DIRS, &DIRS);
        let neigh: Vec<Idx4> = prod
            .filter_map(|(dx, dy, dz, dw)| {
                if dx == &0 && dy == &0 && dz == &0 && dw == &0 {
                    None
                } else {
                    Some(Idx4::new(
                        self.0 + dx,
                        self.1 + dy,
                        self.2 + dz,
                        self.3 + dw,
                    ))
                }
            })
            .collect();

        neigh
    }
}

#[derive(Clone, Debug)]
struct Life<TIndex> {
    cubes: HashMap<TIndex, Cube>,
}

impl<TIndex> Life<TIndex>
where
    TIndex: Neighbor + Hash + Eq,
{
    fn new(mut cubes: HashMap<TIndex, Cube>) -> Self {
        let indices_to_activate: HashSet<TIndex> = cubes
            .iter()
            .flat_map(|(i, _)| i.neighbors().into_iter())
            .collect();

        for idx in indices_to_activate {
            if let None = cubes.get(&idx) {
                cubes.insert(idx, Cube::Inactive);
            }
        }

        Self { cubes }
    }

    fn get(&self, idx: TIndex) -> Option<Cube> {
        self.cubes.get(&idx).copied()
    }

    fn activate_cube(&mut self, idx: TIndex) {
        self.set_cube(idx, Cube::Active);
    }

    fn deactivate_cube(&mut self, idx: TIndex) {
        self.set_cube(idx, Cube::Inactive);
    }

    fn set_cube(&mut self, idx: TIndex, cube: Cube) {
        for neighbor_idx in idx.neighbors() {
            if let None = self.get(neighbor_idx) {
                self.cubes.insert(neighbor_idx, Cube::Inactive);
            }
        }
        self.cubes.insert(idx, cube);
    }

    fn step(&mut self) {
        let mut next = self.clone();

        for (&idx, cube) in self.cubes.iter() {
            let active_neighbors_count = idx
                .neighbors()
                .into_iter()
                .filter(|&i| matches!(self.get(i), Some(Cube::Active)))
                .count();

            match cube {
                Cube::Active if active_neighbors_count == 2 || active_neighbors_count == 3 => {
                    // cube becomes active
                    next.activate_cube(idx);
                }
                Cube::Inactive if active_neighbors_count == 3 => {
                    // cube becomes active
                    next.activate_cube(idx);
                }
                _ => {
                    // cube becomes inactive
                    next.deactivate_cube(idx);
                }
            }
        }

        *self = next;
    }

    fn count_active(&self) -> usize {
        self.cubes
            .iter()
            .filter(|(_, c)| matches!(c, Cube::Active))
            .count()
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
enum Cube {
    Active,
    Inactive,
}

impl TryFrom<char> for Cube {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Cube::Inactive,
            '#' => Cube::Active,
            _ => return Err(()),
        })
    }
}

fn parse_initial_state_from_file(file: &str) -> Grid<Cube> {
    let content = std::fs::read_to_string(file).unwrap();

    let lines: Vec<_> = content
        .trim()
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();

    let height = lines.len();
    let width = lines[0].trim().len();

    let data: Vec<Cube> = lines
        .into_iter()
        .flat_map(|s| s.chars())
        .filter_map(|c| Cube::try_from(c).ok())
        .collect();

    let grid = Grid::new(width, height, data);
    grid
}
