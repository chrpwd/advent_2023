extern crate nom;

use nom::bytes::complete::tag;
use nom::character::complete::{self, alpha1, digit1};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

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
  id: String,
  rounds: Vec<Vec<Cube>>
}

impl Game {
  fn is_valid_id(&self, cmp: &BTreeMap<&str, u32>) -> bool {
      self.rounds.iter().all(|round| {
          round.iter().all(|cube| {
              let color = cube.color.to_lowercase();
              cube.amount <= *cmp.get(color.as_str()).expect("a valid cube")
          })
      })
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

  Ok((input, Game{id: id.to_string(), rounds}))
}

fn read_lines(filename: &str) -> io::Result<impl Iterator<Item = io::Result<String>>> {
  let file = File::open(filename)?;
  let reader = BufReader::new(file);

  Ok(reader.lines())
}

fn main() -> Result<(), Box<dyn Error>> {

    let map = BTreeMap::from([
      ("red", 12),
      ("green", 13),
      ("blue", 14)
    ]);
    
    match read_lines("../input.txt") {
      Ok(lines) => {

        let input = lines.filter_map(|l| l.ok());

        let sum: u32 = input
            .filter_map(|game_str| {
                let game = parse_game(game_str.as_str()).expect("game can be parsed");
                game.1.is_valid_id(&map).then_some(game.1.id.parse::<u32>().expect("can be parsed to num"))
            })
            .sum();

        println!("{}", sum)
      }
      Err(err) => eprintln!("Error opening file: {}", err),
  }
    
  Ok(())
}
