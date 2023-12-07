use std::collections::HashSet;
use std::{fs::read_to_string, io};

use nom::bytes::complete::take_while;
use nom::character::complete::multispace1;
use nom::sequence::preceded;
use nom::{
    bytes::complete::tag,
    character::complete::{
        digit1, line_ending, space1,
    },
    multi::{separated_list1},
    sequence::{
        separated_pair, tuple,
    },
    IResult, Parser,
};
#[derive(Debug)]
struct Card {
    _id: String,
    winning_nums: HashSet<u32>,
    card_nums: HashSet<u32>,
}

fn card_sections(input: &str) -> IResult<&str, HashSet<u32>> {
    separated_list1(multispace1, digit1)
        .map(|set: Vec<&str>| HashSet::from_iter(set.iter().map(|&e| e.parse::<u32>().expect("Failed to parse")))).parse(input)
}

fn game(input: &str) -> IResult<&str, Card> {

    let (input, id) = preceded(take_while(|c: char| c.is_whitespace() || c.is_alphabetic()), digit1)(input)?;
    preceded(tuple((tag(":"), space1)), separated_pair(card_sections, take_while(|c: char| c.is_whitespace() || c == '|'), card_sections).map(|(winners, card_nums)| Card {
        winning_nums: winners,
        card_nums,
        _id: id.to_string()
    })).parse(input)
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

       let intersection: Vec<&u32> = card.winning_nums.intersection(&card.card_nums).into_iter().collect();
       if intersection.len() > 0 {
        return intersection.iter().skip(1).enumerate().map(|(i, _)| 1 << i).sum::<usize>() + 1
       } else {
        return 0
       }

    }).sum();
    println!("POINTS: {}", points);
}
