use std::{fs::read_to_string, io, cmp};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use roots::{find_roots_quadratic, Roots};


fn parse_line(input: &str) -> IResult<&str, u64> {
    let (input, list) = separated_list1(multispace1, map_res(digit1, |s: &str| s.parse::<String>()))(input)?;
    let num = list.join("").parse::<u64>().expect("can be parsed to num");

    Ok((input, num))
}



fn parse_data(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, _) = preceded(tag("Time:"), multispace0)(input)?;
    let (input, time) = parse_line(input)?;

    let (input, _) = newline(input)?;

    let (input, _) = preceded(tag("Distance:"), multispace1)(input)?;
    let (input, distance) = parse_line(input)?;
    Ok((input, (time, distance)))
}

fn read_file_content(file_path: &str) -> io::Result<String> {
    // Read the file into a String
    let file_content: String = read_to_string(file_path)?;

    // Return the &str reference
    Ok(file_content)
}

fn main() {
    let s = read_file_content("input.txt").expect("read file contents");
    let input = s.as_str();
    let (_, (time, distance)) = parse_data(input).expect("input can be parsed correctly");

    println!("{} - {}", time , distance);
    let roots = find_roots_quadratic(1 as f64, time as f64, distance as f64);

    match roots {
        Roots::Two(roots) => {
            let num = (roots[1].floor() - roots[0].ceil())+ (1 as f64);
            println!("RESULT: {}", num)
        }

        _ => {
            println!("DID NOT HAVE 2 ROOTS")

        }
    }
}
