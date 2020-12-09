pub mod grid;
pub mod handheld;
pub mod password;
pub mod passport;
pub mod seating;
pub mod questions;
pub mod bags;
pub mod xmas;

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Returns an iterator over the lines of a given file
pub fn read_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
