use std::{fs::read_to_string, io};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(multispace1, map_res(digit1, |s: &str| s.parse::<u32>()))(input)
}

fn parse_data(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
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

    let result = time.into_iter().zip(distance).map(|(time, record_distance)| {
        (0..time).into_iter().filter_map(|speed| {
            let my_distance = (time - speed) * speed;
            (my_distance > record_distance).then_some(my_distance)
        }).count()
    }).product::<usize>();
    println!("RESULT: {}", result)
}
