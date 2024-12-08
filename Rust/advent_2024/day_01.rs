use std::{collections::BTreeMap, env};

use crate::tools::txt_reader::read_lines;

#[cfg(test)]
use log::{debug, info};

#[inline]
pub fn loader() -> (Vec<usize>, Vec<usize>) {
    let lines = if let Ok(lines) = read_lines("./Data/2024/day_01.txt") {
        lines
    } else {
        panic!("Could not read file");
    };

    let mut list_a: Vec<usize> = Vec::new();
    let mut list_b: Vec<usize> = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        list_a.push(parts[0].to_string().parse().unwrap());
        list_b.push(parts[1].to_string().parse().unwrap());
    }
    (list_a, list_b)
}

#[inline]
pub fn part_1(list_a: &[usize], list_b: &[usize]) -> u64 {
    let mut sorted_a = list_a.iter().copied().collect::<Vec<usize>>();
    sorted_a.sort();
    let mut sorted_b = list_b.iter().copied().collect::<Vec<usize>>();
    sorted_b.sort();

    let mut result: u64 = 0;

    for (a, b) in sorted_a.iter().zip(sorted_b.iter()) {
        if a > b {
            result += (a - b) as u64;
        } else {
            result += (b - a) as u64;
        }
    }

    result
}

#[test_log::test]
fn test_part_1() {
    let (list_a, list_b) = loader();
    info!("Part 1 [] : {}", part_1(&list_a, &list_b));
}

#[inline]
pub fn part_2_dict(list_a: &[usize], list_b: &[usize]) -> u64 {
    let mut dict_b = BTreeMap::new();
    for b in list_b.iter() {
        let value = dict_b.entry(b).or_insert(0);
        *value += 1;
    }

    let mut result: u64 = 0;
    for a in list_a.iter() {
        result += (dict_b.get(a).unwrap_or(&0) * a) as u64;
    }
    result
}

#[test_log::test]
fn test_part_2_dict() {
    let (list_a, list_b) = loader();
    info!("Part 2 [DICT] : {}", part_2_dict(&list_a, &list_b));
}

#[inline]
pub fn part_2_vec(list_a: &[usize], list_b: &[usize]) -> u64 {
    let max_id = list_b.iter().max().unwrap();
    let mut dict = vec![0; *max_id + 1];

    for b in list_b.iter() {
        dict[*b] += 1;
    }

    let mut result: u64 = 0;

    for a in list_a.iter() {
        if a > max_id {
            continue;
        }
        result += (dict[*a] * a) as u64;
    }

    result
}

#[test_log::test]
fn test_part_2_vec() {
    let (list_a, list_b) = loader();
    info!("Part 2 [VEC] : {}", part_2_vec(&list_a, &list_b));
}
