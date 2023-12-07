use std::{fs::read_to_string, io, collections::BTreeMap};
use itertools::Itertools;


#[derive(Debug)]
enum Value {
    Number(u32),
    Dot,
    Symbol(char)
}

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd, Clone, Hash, Copy)]
struct Coords {
    y: i32,
    x: i32
}


fn read_file_content(file_path: &str) -> io::Result<String> {
    // Read the file into a String
    let file_content: String = read_to_string(file_path)?;

    // Return the &str reference
    Ok(file_content)
}

fn main() {
    let s = read_file_content("input.txt").expect("read file contents");
    let schema = s.as_str();

    let map: BTreeMap<Coords, Value> = schema
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, ch)| {
                (
                    Coords{
                        x: x as i32,
                        y: y as i32
                    },
                    match ch {
                        '.' => Value::Dot,
                        c if c.is_ascii_digit() => {
                            Value::Number(c.to_digit(10).expect("char to be parsed as num"))
                        }
                        c => Value::Symbol(c),
                    },
                )
            })
        })
        .collect();
    

    //nums
    let mut numbers: Vec<Vec<(Coords, u32)>> = vec![];

    for(coords, value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some((last_coords, _)) => {
                            if last_coords.x + 1 == coords.x {
                                let last = numbers.iter_mut().last().expect("should exist");
                                last.push((*coords, *num))
                            } else {
                                numbers.push(vec![(*coords, *num)]);
                            }
                        }
                        None => unimplemented!("shouldnt happen")
                    }
                },
                None => {
                    numbers.push(vec![(*coords, *num)])
                }
            }
        }
    }

    let position_diffs = [
        (1, 1),
        (1, 0),
        (1, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (0, 1),
    ];

    let mut total = 0;
    for symbol in map.iter().filter(|(_, value)| matches!(value, Value::Symbol('*'), )) {
        let adjacent_checks: Vec<Coords> = 
            position_diffs.iter().map(|adjacent_pos| {
                Coords {
                    x: symbol.0.x + adjacent_pos.0,
                    y: symbol.0.y + adjacent_pos.1
                }
            })
        .collect();

        let mut indexes_of_numbers = vec![];

        for pos in adjacent_checks {
            for (i, num_list) in numbers.iter().enumerate()
            {
                if num_list
                    .iter()
                    .find(|(num_pos, _)| num_pos == &pos)
                    .is_some()
                {
                    indexes_of_numbers.push(i);
                }
            }
        }

        let is_gear = indexes_of_numbers.iter().unique().count() == 2;
        
        if is_gear {
            total += indexes_of_numbers
                .iter()
                .unique()
                .map(|index| {
                    numbers[*index]
                    .iter()
                    .map(|(_, num)| num.to_string())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
                }).product::<usize>()
        }
    }

    println!("{}", total.to_string());
    
}
