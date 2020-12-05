use std::{convert::TryFrom, convert::TryInto, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub struct Seating {
    row: [FrontBack; 7],
    column: [LeftRight; 3],
    row_value: u32,
    column_value: u32,
    id: u32,
}

impl Seating {
    fn new(row: [FrontBack; 7], column: [LeftRight; 3]) -> Self {
        let row_value = Seating::evaluate_row(&row);
        let column_value = Seating::evaluate_column(&column);
        let id = Seating::evaluate_id(row_value, column_value);
        Self {
            row,
            column,
            row_value,
            column_value,
            id,
        }
    }

    pub fn row(&self) -> u32 {
        self.row_value
    }

    pub fn column(&self) -> u32 {
        self.column_value
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    fn evaluate_id(row_value: u32, column_value: u32) -> u32 {
        8 * row_value + column_value
    }

    fn evaluate_row(fbs: &[FrontBack; 7]) -> u32 {
        let (mut low, mut high) = (0, 127);

        for fb in fbs {
            let mid = (low + high) / 2;

            match fb {
                FrontBack::Front => {
                    high = mid;
                }
                FrontBack::Back => {
                    low = mid + 1;
                }
            }
        }

        debug_assert_eq!(low, high);

        low
    }

    fn evaluate_column(lrs: &[LeftRight; 3]) -> u32 {
        let (mut low, mut high) = (0, 7);

        for fb in lrs {
            let mid = (low + high) / 2;

            match fb {
                LeftRight::Left => {
                    high = mid;
                }
                LeftRight::Right => {
                    low = mid + 1;
                }
            }
        }

        debug_assert_eq!(low, high);

        low
    }
}

#[derive(Debug, Clone, PartialEq)]
enum FrontBack {
    Front,
    Back,
}

impl TryFrom<char> for FrontBack {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'F' => FrontBack::Front,
            'B' => FrontBack::Back,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum LeftRight {
    Left,
    Right,
}

impl TryFrom<char> for LeftRight {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'L' => LeftRight::Left,
            'R' => LeftRight::Right,
            _ => return Err(()),
        })
    }
}

impl FromStr for Seating {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(());
        }

        let row = &s[..7];
        let column = &s[7..];

        let row = row
            .chars()
            .map(FrontBack::try_from)
            .collect::<Result<Vec<FrontBack>, _>>()?;

        let column = column
            .chars()
            .map(LeftRight::try_from)
            .collect::<Result<Vec<LeftRight>, _>>()?;

        Ok(Seating::new(
            row.try_into().map_err(|_| ())?,
            column.try_into().map_err(|_| ())?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        check_example("FBFBBFFRLR", 44, 5, 357);
        check_example("BFFFBBFRRR", 70, 7, 567);
        check_example("FFFBBBFRRR", 14, 7, 119);
        check_example("BBFFBBFRLL", 102, 4, 820);
    }

    fn check_example(input: &str, row: u32, column: u32, id: u32) {
        let seating = Seating::from_str(input).unwrap();

        assert_eq!(seating.row(), row);
        assert_eq!(seating.column(), column);
        assert_eq!(seating.id(), id);
    }
}
