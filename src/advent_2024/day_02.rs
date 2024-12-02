use std::collections::BTreeMap;

use crate::tools::txt_reader::read_lines;

#[inline]
pub fn loader() -> Vec<Vec<usize>> {
    let lines = if let Ok(lines) = read_lines("./Data/2024/day_02.txt") {
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
fn check_line(line: &[usize]) -> bool {
    let checking = if line[0] < line[1] {
        |a, b| a < b && a + 3 >= b
    } else {
        |a, b| a > b && b + 3 >= a
    };

    let mut prev_value = &line[0];
    for value in line.iter().skip(1) {
        if !checking(*prev_value, *value) {
            return false;
        }
        prev_value = value;
    }
    true
}

#[inline]
pub fn part_1(list: &[Vec<usize>]) -> u64 {
    let mut result: u64 = list.len() as u64;

    for line in list.iter() {
        if !check_line(line) {
            result -= 1;
        }
    }

    result
}

#[inline]
pub fn part_2(list: &[Vec<usize>]) -> u64 {
    let mut result: u64 = 0;

    for line in list.iter() {
        if check_line(line) {
            result += 1;
            continue;
        }

        for i in 0..line.len() {
            let mut new_line = line.to_vec();
            new_line.remove(i);
            if check_line(&new_line) {
                result += 1;
                break;
            }
        }
    }

    result
}

#[inline]
fn check_line_idx(line: &[usize]) -> Option<usize> {
    let checking = if line[0] < line[1] {
        |a, b| a < b && a + 3 >= b
    } else {
        |a, b| a > b && b + 3 >= a
    };

    let mut prev_value = &line[0];
    for (i, value) in line.iter().skip(1).enumerate() {
        if !checking(*prev_value, *value) {
            return Some(i);
        }
        prev_value = value;
    }
    None
}

//FIXME: This is not working

#[inline]
pub fn part_2_opt(list: &[Vec<usize>]) -> u64 {
    let mut result: u64 = 0;

    for line in list.iter() {
        if let Some(idx) = check_line_idx(line) {
            let mut new_line = line.to_vec();
            new_line.remove(idx);

            let mut new_line2 = line.to_vec();
            new_line2.remove(idx + 1);

            if check_line_idx(&new_line).is_some() && check_line_idx(&new_line2).is_some() {
                continue;
            }
        }

        result += 1;
    }

    result
}
