use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut sum: u32 = 0;
    match read_lines("../input.txt") {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(l) => {
                        let mut it = l.chars().filter_map(|character| character.to_digit(10));

                        let first = it.next().expect("should be a num");

                        let num = match it.last() {
                            Some(num) => format!("{first}{num}"),
                            None => format!("{first}{first}"),
                        }
                        .parse::<u32>()
                        .expect("should be a valid number");

                    sum += num
                    
                    }

                    Err(err) => eprintln!("Error opening line of file: {}", err)
                }

            }
        }

        Err(err) => eprintln!("Error opening file: {}", err),

    }

    println!("{}", sum)
}

fn read_lines(filename: &str) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines())
}

