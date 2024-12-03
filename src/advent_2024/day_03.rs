use std::num::ParseIntError;

use crate::tools::txt_reader::read_lines;

struct Token<T> {
    value: T,
    start: usize,
    end: usize,
}

type Number = Token<usize>;

trait ParserTrait {
    const SEPARATOR: char = ',';
    const OPEN_PARAMS: char = '(';
    const CLOSE_PARAMS: char = ')';

    const NUMBER_LENTH: usize = 5;

    fn get_number(chars: &[char], position: usize) -> Option<Number> {
        let mut pos = position;

        while chars[pos].is_digit(10) {
            pos += 1;
        }
        if position == pos {
            return None;
        }

        Some(Token {
            value: chars[position..pos]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
            start: position,
            end: pos,
        })
    }
}

struct Parser;

impl ParserTrait for Parser {}

#[inline]
pub fn loader() -> String {
    let lines = if let Ok(lines) = read_lines("./Data/2024/day_03.txt") {
        lines
    } else {
        panic!("Could not read file");
    };

    let mut list = Vec::new();
    for line in lines {
        list.push(line);
    }

    list.concat()
}

#[inline]
pub fn part_1(text: &str) -> usize {
    let mut result = 0;
    let chars = text.chars().collect::<Vec<char>>();

    let mut position = 0;
    while position < chars.len() {
        if chars[position] != 'm' {
            position += 1;
            continue;
        }

        let look_up = position + 3;
        if chars[position..look_up] != ['m', 'u', 'l'] {
            position += 1;
            continue;
        }

        position = look_up;
        if chars[position] != Parser::OPEN_PARAMS {
            continue;
        }

        let mut params: Vec<Number> = Vec::with_capacity(2);

        position += 1;
        while let Some(number) = Parser::get_number(&chars, position) {
            position = number.end;

            params.push(number);
            if chars[position] != Parser::SEPARATOR {
                break;
            }

            position += 1;
        }

        if chars[position] != Parser::CLOSE_PARAMS || params.len() != 2 {
            continue;
        }

        result += params[0].value * params[1].value;

        position += 1;
    }
    result
}
