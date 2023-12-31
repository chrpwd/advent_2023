use std::{io, fs::read_to_string};
use std::ops::Range;

use nom::character::complete;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{IResult, bytes::complete::{tag, take_until}, sequence::preceded, multi::{separated_list1, many1}, character::complete::{space1, digit1, line_ending}};


#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>
}

impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self.mappings.iter().find(|(source_range, _) | {
            source_range.contains(&source)
        } );
        let Some((source_range, dest_range)) = valid_mapping
        else {
            return source
        };

        let offset = source - source_range.start;
        dest_range.start + offset
    }
}

fn line (input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    // println!("INPUT {}", input);
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        preceded(tag(" "), complete::u64),
        preceded(tag(" "), complete::u64)
    ))(input)?;

    Ok((input, (
        source..(source+num),
        destination..(destination+num)
    )))
}
fn parse_seed_maps(input: &str) -> IResult<&str, Vec<SeedMap>> {
    let (input, m) = many0(preceded(take_until("map:"), preceded(tag("map:"), many1(preceded(line_ending, line)))))(input)?;

    let maps: Vec<SeedMap> = m.into_iter().map(|mappings| return SeedMap {mappings}).collect();
    return Ok((input, maps));
}
fn parse_seeds(input: &str) -> IResult<&str, (Vec<u64>, Vec<SeedMap>)> {

    let (input, s) = preceded(tag("seeds: "), separated_list1(space1, digit1))(input)?;

    let seeds: Vec<u64> = s.into_iter().map(|x| x.parse::<u64>().unwrap()).collect();

    let (_lf, maps) = parse_seed_maps(input)?;


    return Ok((input, (seeds, maps))) 
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
    let (_, (seeds, maps)) = parse_seeds(input).expect("to parse correctly");

    let locations = seeds.iter().map(|seed| {
        maps.iter().fold(*seed, |seed, map| map.translate(seed))
    }).collect::<Vec<u64>>();


    println!("LOCATION: {}", locations.iter().min().expect("should have minimum value").to_string());
}
