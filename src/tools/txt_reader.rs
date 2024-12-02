use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Flatten;
use std::path::Path;

type FileLines = Flatten<std::io::Lines<io::BufReader<File>>>;

pub fn read_lines<P>(filename: P) -> io::Result<FileLines>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().flatten())
}
