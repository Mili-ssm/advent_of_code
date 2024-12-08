use std::{
    arch::x86_64,
    collections::{BTreeMap, BTreeSet},
};

use log::{debug, error, info, warn};

use crate::tools::txt_reader::read_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Signal(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Antenna {
    signal: Signal,
    position: Position,
}

#[derive(Debug)]
pub struct Map {
    bounds: Position,
    antennas: Vec<Antenna>,
}

#[inline]
pub fn loader() -> Map {
    let lines = if let Ok(lines) = read_lines("./Data/2024/day_08.txt") {
        lines
    } else {
        panic!("Could not read file");
    };

    let mut antennas = Vec::new();

    let mut bounds = Position { x: 0, y: 0 };
    for line in lines.into_iter() {
        bounds.y = 0;
        for signal in line.chars() {
            if signal != '.' {
                antennas.push(Antenna {
                    signal: Signal(signal as u16),
                    position: bounds,
                });
            }
            bounds.y += 1;
        }
        bounds.x += 1;
    }

    Map {
        bounds: bounds,
        antennas,
    }
}

#[inline]
pub fn part_1(map: &mut Map) -> u64 {
    map.antennas.sort_by(|a, b| a.signal.cmp(&b.signal));

    let mut anti_nodes: BTreeSet<Position> = BTreeSet::new();

    for i in 0..map.antennas.len() {
        let antenna_1 = &map.antennas[i];
        for j in i + 1..map.antennas.len() {
            let antenna_2 = &map.antennas[j];
            if antenna_1.signal != antenna_2.signal {
                break;
            }

            let x_distance = (antenna_1.position.x - antenna_2.position.x).abs();
            let y_distance = (antenna_1.position.y - antenna_2.position.y).abs();

            let (x1, x2) = if antenna_1.position.x >= antenna_2.position.x {
                (
                    antenna_1.position.x + x_distance,
                    antenna_2.position.x - x_distance,
                )
            } else {
                (
                    antenna_1.position.x - x_distance,
                    antenna_2.position.x + x_distance,
                )
            };

            let (y1, y2) = if antenna_1.position.y >= antenna_2.position.y {
                (
                    antenna_1.position.y + y_distance,
                    antenna_2.position.y - y_distance,
                )
            } else {
                (
                    antenna_1.position.y - y_distance,
                    antenna_2.position.y + y_distance,
                )
            };

            if 0 <= x1 && x1 < map.bounds.x && 0 <= y1 && y1 < map.bounds.y {
                anti_nodes.insert(Position { x: x1, y: y1 });
            }

            if 0 <= x2 && x2 < map.bounds.x && 0 <= y2 && y2 < map.bounds.y {
                anti_nodes.insert(Position { x: x2, y: y2 });
            }
        }
    }

    anti_nodes.len() as u64
}

pub fn part_2(map: &mut Map) -> u64 {
    map.antennas.sort_by(|a, b| a.signal.cmp(&b.signal));

    let mut anti_nodes: BTreeSet<Position> = BTreeSet::new();

    for i in 0..map.antennas.len() {
        let antenna_1 = &map.antennas[i];
        for j in i + 1..map.antennas.len() {
            let antenna_2 = &map.antennas[j];
            if antenna_1.signal != antenna_2.signal {
                break;
            }

            let x_distance = antenna_1.position.x - antenna_2.position.x;
            let y_distance = antenna_1.position.y - antenna_2.position.y;

            let mut anti_node = Position {
                x: antenna_2.position.x + x_distance,
                y: antenna_2.position.y + y_distance,
            };

            while 0 <= anti_node.x
                && anti_node.x < map.bounds.x
                && 0 <= anti_node.y
                && anti_node.y < map.bounds.y
            {
                anti_nodes.insert(anti_node);

                anti_node = Position {
                    x: anti_node.x + x_distance,
                    y: anti_node.y + y_distance,
                };
            }

            let x_distance = -1 * x_distance;
            let y_distance = -1 * y_distance;

            let mut anti_node = Position {
                x: antenna_1.position.x + x_distance,
                y: antenna_1.position.y + y_distance,
            };

            while 0 <= anti_node.x
                && anti_node.x < map.bounds.x
                && 0 <= anti_node.y
                && anti_node.y < map.bounds.y
            {
                anti_nodes.insert(anti_node);

                anti_node = Position {
                    x: anti_node.x + x_distance,
                    y: anti_node.y + y_distance,
                };
            }
        }
    }

    anti_nodes.len() as u64
}

#[test_log::test]
fn test_part_1() {
    let start_total = std::time::Instant::now();
    let mut map = loader();

    let start = std::time::Instant::now();
    let result = part_1(&mut map);

    let elapsed = start.elapsed();
    let elapsed_total = start_total.elapsed();
    info!(
        "\n - Part 1 [] : {}  \n\t| Time proces: {:?}\n\t| Time Total : {:?}",
        result, elapsed, elapsed_total
    );
}

#[test_log::test]
fn test_part_2() {
    let start_total = std::time::Instant::now();
    let mut map = loader();

    let start = std::time::Instant::now();
    let result = part_2(&mut map);

    let elapsed = start.elapsed();
    let elapsed_total = start_total.elapsed();
    info!(
        "\n - Part 2 [] : {}  \n\t| Time proces: {:?}\n\t| Time Total : {:?}",
        result, elapsed, elapsed_total
    );
}
