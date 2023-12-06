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


    let mut total = 0;
    for num_list in numbers {
        let positions = [
            (1, 1),
            (1, 0),
            (1, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (0, 1),
        ];
        let part_positions: Vec<Coords> = num_list.iter().map(|(coords, _)| *coords).collect();
        let adjacent_checks: Vec<Coords> = num_list.iter().flat_map(|(pos, _)| {
            positions.iter().map(|adjacent_pos| {
                Coords {
                    x: adjacent_pos.0 + pos.x,
                    y: adjacent_pos.1 + pos.y
                }
            })
        })
        .unique()
        .filter(|num| !part_positions.contains(num))
        .collect();

        let is_part_number = adjacent_checks.iter().any(|pos| {
            let value = map.get(&pos);
            if let Some(Value::Symbol(_) )= value {
                true
            } else {
                false
            }
        });


        if is_part_number {

            let num = num_list
            .iter()
            .map(|(_, num)| num.to_string())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
            
            total += num
        }
    }

    println!("{}", total.to_string());
    
}
