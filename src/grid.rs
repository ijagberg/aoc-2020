use std::convert::TryFrom;

/// A two dimensional grid
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    /// Constructs a new `Grid`
    ///
    /// # Panics
    /// If `width * height != cells.len()`
    pub fn new(width: usize, height: usize, cells: Vec<T>) -> Self {
        assert!(width * height == cells.len());
        Self {
            width,
            height,
            cells,
        }
    }

    /// Returns the width of the grid (number of columns)
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid (number of rows)
    pub fn height(&self) -> usize {
        self.height
    }

    /// Attempt to get the item in row `row`, column `col`
    ///
    /// Returns `None` if `row` or `col` is outside the grid (less than 0, larger than the height or width of the grid)
    pub fn get<Idx>(&self, row: Idx, col: Idx) -> Option<&T>
    where
        usize: TryFrom<Idx>,
    {
        let row = usize::try_from(row).ok()?;
        let col = usize::try_from(col).ok()?;

        let linear_idx = self.linear_idx(row, col)?;

        Some(&self.cells[linear_idx])
    }

    fn linear_idx(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.height() || col >= self.width() {
            None
        } else {
            Some(row * self.width() + col)
        }
    }
}

pub fn parse_toboggan_map(lines: &[String]) -> TobogganMap {
    let height = lines.len();

    if height == 0 {
        return TobogganMap::from_grid(Grid::new(0, 0, Vec::new()));
    }

    let width = lines[0].len();

    let mut cells = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        let chars_in_line: Vec<_> = line.chars().collect();
        let line_len = chars_in_line.len();
        if line_len != width {
            panic!(
                "invalid width of line {}, should be {}, was {}",
                row, width, line_len
            );
        }
        for c in chars_in_line {
            cells.push(
                Cell::try_from(c)
                    .unwrap_or_else(|_| panic!("could not create a Cell from char: '{}'", c)),
            )
        }
    }

    TobogganMap::from_grid(Grid::new(width, height, cells))
}

pub enum Cell {
    Empty,
    Tree,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => Cell::Tree,
            '.' => Cell::Empty,
            _ => return Err(()),
        })
    }
}

/// An _infinite_ two dimensional grid
///
/// This grid can be indexed on a column that is larger than the actual width of the grid.
/// It does this by pretending that the grid is repeated infinitely to the right.
/// The item at index `width+1` is the same as the item at index `1`
pub struct InfiniteGrid<T> {
    grid: Grid<T>,
}

impl<T> InfiniteGrid<T> {
    /// Construct a new infinite grid by repeating `grid`
    pub fn new(grid: Grid<T>) -> Self {
        Self { grid }
    }

    fn grid(&self) -> &Grid<T> {
        &self.grid
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let col = col % self.grid().width();

        self.grid().get(row, col)
    }

    pub fn iter_from(&self, row: usize, col: usize, right: usize, down: usize) -> SlopeIter<T> {
        SlopeIter::new((row, col), right, down, &self)
    }
}

/// A map of a forest a toboggan can travel through
pub struct TobogganMap {
    grid: InfiniteGrid<Cell>,
}

impl TobogganMap {
    pub fn new(grid: InfiniteGrid<Cell>) -> Self {
        Self { grid }
    }

    fn grid(&self) -> &InfiniteGrid<Cell> {
        &self.grid
    }

    pub fn from_grid(grid: Grid<Cell>) -> Self {
        let inf_grid = InfiniteGrid::new(grid);
        Self { grid: inf_grid }
    }

    /// Count the number of tree cells you would collide with if you were to travel from `(start_row, start_col)`
    /// with a slope `(right_step, down_step)`. I.e. repeatedly taking `right_step` steps horizontally, and `down_step` steps vertically,
    /// until you reach the bottom of the map.
    pub fn count_tree_collision(
        &self,
        start_row: usize,
        start_col: usize,
        right_step: usize,
        down_step: usize,
    ) -> usize {
        self.grid()
            .iter_from(start_row, start_col, right_step, down_step)
            .filter(|&(row, col)| matches!(self.grid().get(row, col), Some(Cell::Tree)))
            .count()
    }
}

pub struct SlopeIter<'a, T> {
    current_pos: (usize, usize),
    right: usize,
    down: usize,
    grid: &'a InfiniteGrid<T>,
}

impl<'a, T> SlopeIter<'a, T> {
    fn new(
        current_pos: (usize, usize),
        right: usize,
        down: usize,
        grid: &'a InfiniteGrid<T>,
    ) -> Self {
        Self {
            current_pos,
            right,
            down,
            grid,
        }
    }
}

impl<'a, T> Iterator for SlopeIter<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.current_pos;
        let next = (cur.0 + self.down, cur.1 + self.right);

        if self.grid.get(next.0, next.1).is_some() {
            self.current_pos = next;
            Some(next)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_data() -> TobogganMap {
        let chars: Vec<char> = "..##.......#...#...#...#....#..#...#.#...#.#.#...##..#...#.##......#.#.#....#.#........##.##...#...#...##....#.#..#...#.#".chars().collect();
        let lines: Vec<_> = chars
            .chunks(11)
            .map(|c| c.iter().collect::<String>())
            .collect();

        parse_toboggan_map(&lines)
    }

    #[test]
    fn single_slope_test() {
        let toboggan_map = example_data();

        assert_eq!(toboggan_map.count_tree_collision(0, 0, 3, 1), 7);
    }

    #[test]
    fn multiple_slopes_test() {
        let map = example_data();

        const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let trees: Vec<usize> = SLOPES
            .iter()
            .map(|&(right, down)| map.count_tree_collision(0, 0, right, down))
            .collect();

        assert_eq!(vec![2, 7, 3, 4, 2], trees);
    }
}
