use std::collections::HashSet;
use std::{fs::read_to_string, io};

use nom::bytes::complete::take_while;
use nom::character::complete::{line_ending, multispace1};
use nom::{character::complete::digit1, multi::separated_list1, sequence::preceded, IResult};

#[derive(Debug)]
struct Card {
    _id: String,
    winning_nums: HashSet<i32>,
    card_nums: HashSet<i32>,
}

fn card_sections(input: &str) -> IResult<&str, HashSet<i32>> {
    let (input, nums) = separated_list1(multispace1, digit1)(input)?;

    let nums_set: HashSet<i32> = nums.iter().map(|n| n.parse::<i32>().expect("is number")).collect();

    Ok((input, nums_set))
}

fn game(input: &str) -> IResult<&str, Card> {
    let (input, id) = preceded(take_while(|c: char| c.is_whitespace() || c.is_alphabetic()), digit1)(input)?;
    let (input, sets) = preceded(
        take_while(|c: char| c == ':' || c.is_whitespace()),
        separated_list1(take_while(|c: char| c == '|' || c.is_whitespace()), card_sections),
    )(input)?;

    // Clone the vectors before constructing the Card struct
    let winning_nums = sets[0].clone();
    let card_nums = sets[1].clone();

    Ok((
        input,
        Card {
            _id: id.trim().to_string(),
            winning_nums,
            card_nums,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
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
    let (_lf, cards) = parse_cards(input).expect("to parse correctly");
    
    let points: usize = cards.iter().map(|card| {

       let intersection = card.winning_nums.intersection(&card.card_nums).into_iter().collect::<Vec<&i32>>();
       if intersection.len() > 0 {
        return intersection.iter().skip(1).enumerate().map(|(i, _)| 1 << i).sum::<usize>() + 1
       } else {
        return 0
       }

    }).sum();
    println!("POINTS: {}", points);
}
