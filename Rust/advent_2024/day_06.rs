use log::{debug, error, info, warn};

use crate::tools::txt_reader::read_lines;

#[inline]
pub fn loader() -> Vec<Vec<usize>> {
    let lines = if let Ok(lines) = read_lines("./Data/2024/day_06.txt") {
        lines
    } else {
        panic!("Could not read file");
    };

    let mut list: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        list.push(
            parts
                .iter()
                .map(|s| s.to_string().parse().unwrap())
                .collect(),
        );
    }
    list
}

#[inline]
fn find_error(line: &[usize]) -> Option<usize> {
    let checking = if line[0] < line[1] {
        |a, b| a < b && a + 3 >= b
    } else {
        |a, b| a > b && b + 3 >= a
    };

    for i in 0..line.len() - 1 {
        if !checking(line[i], line[i + 1]) {
            return Some(i);
        }
    }
    None
}

#[inline]
pub fn part_1(list: &[Vec<usize>]) -> u64 {
    let mut result: u64 = list.len() as u64;

    for line in list.iter() {
        if find_error(line).is_some() {
            result -= 1;
        }
    }

    result
}

#[test_log::test]
fn test_part_1() {
    let list = loader();
    info!(" Part 1 [] : {}", part_1(&list));
}
