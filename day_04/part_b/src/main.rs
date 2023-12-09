use std::{cmp, iter};
use std::collections::HashSet;
use std::{fs::read_to_string, io};
use std::collections::VecDeque;

use nom::{
    bytes::complete::{take_while, tag},
    character::complete::{
        digit1, line_ending, space1
    },
    multi::separated_list1,
    sequence::{
        separated_pair, tuple, preceded
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
    separated_list1(space1, digit1)
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
    
    let data: Vec<_> = cards.iter().map(|card| (

       card.winning_nums.intersection(&card.card_nums).into_iter().collect::<Vec<&u32>>().len()

    )).collect();

    let mut dq: VecDeque<usize> = VecDeque::from(vec![1; data.len()]); //init to original copies

    let result = data.iter().fold(0, |mut acc, score| {
        let copies = dq.pop_front().unwrap();
        let gained = cmp::min(*score, dq.len());

        let added_copies =vec![copies; gained];

        dq = dq.iter()
            .zip(added_copies.iter().chain(iter::repeat(&0)))
            .map(|(&a, &b)| a + b)
            .collect();

        acc += copies;
        acc
    });
    println!("Cards: {}", result);

}
