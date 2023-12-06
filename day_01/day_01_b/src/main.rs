use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut sum: u32 = 0;
    match read_lines("../input.txt") {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(l) => {

                        let modified = parse_digit(&l);
                        let mut it = modified.chars().filter_map(|character| character.to_digit(10));
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

fn parse_digit(s: &str) -> String {
    let modified_line = s
        .replace("one", "o1e")
        .replace("two", "t2e")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    modified_line
}

fn read_lines(filename: &str) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines())
}
