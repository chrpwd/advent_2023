extern crate nom;

use nom::bytes::complete::tag;
use nom::character::complete::{self, alpha1, digit1};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

use std::cmp;
use std::{
    collections::BTreeMap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};




struct Cube {
  color: String,
  amount: u32
}

struct Game {
  _id: String,
  rounds: Vec<Vec<Cube>>
}

impl Game {
    fn min_counts_sqr(&self) -> u32 {

        let mut color_counts = BTreeMap::new();

        self.rounds.iter().for_each(|round| {
            round.iter().for_each(|cube| {
                let entry = color_counts.entry(cube.color.as_str()).or_insert(0);
                *entry = cmp::max(*entry, cube.amount);
            });
        });

        return color_counts.iter().map(|(_, &count)| count).product();

    }
      
}

fn cube(input: &str) -> IResult<&str, Cube> {
  let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;

  Ok((input, Cube{ color: color.to_string(), amount }))
}

fn round(input: &str) -> IResult<&str, Vec<Cube>> {
  let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
  Ok((input, cubes))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
  let (input, id) = preceded(tag("Game "), digit1)(input)?;
  let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;

  Ok((input, Game{_id: id.to_string(), rounds}))
}

fn read_lines(filename: &str) -> io::Result<impl Iterator<Item = io::Result<String>>> {
  let file = File::open(filename)?;
  let reader = BufReader::new(file);

  Ok(reader.lines())
}

fn main() -> Result<(), Box<dyn Error>> {
    
    match read_lines("../input.txt") {
      Ok(lines) => {

        let input = lines.filter_map(|l| l.ok());

        let sum: u32 = input
            .map(|game_str| {
                let game = parse_game(game_str.as_str()).expect("game can be parsed");
                game.1.min_counts_sqr()
            })
            .sum();

        println!("{}", sum)
      }
      Err(err) => eprintln!("Error opening file: {}", err),
  }
    
  Ok(())
}
