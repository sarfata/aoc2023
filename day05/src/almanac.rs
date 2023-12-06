use std::num::ParseIntError;
use std::{error::Error, fs::read_to_string};

use nom::multi::many0;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space0, space1},
    combinator::{all_consuming, map_res},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    // Numbers with any number of spaces before/after
    preceded(
        tag("seeds:"),
        delimited(
            space0,
            // any number of spaces in between
            separated_list0(space1, parse_int),
            space0,
        ),
    )(input)
}

fn parse_int(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_conversion(input: &str) -> IResult<&str, Conversion> {
    let (input, values) = tuple((
        terminated(parse_int, space1),
        terminated(parse_int, space1),
        terminated(parse_int, space0),
    ))(input)?;

    Ok((input, values.into()))
}

fn parse_map(input: &str) -> IResult<&str, AMap> {
    let (input, source) = terminated(alpha1, tag("-"))(input)?;
    let (input, _) = tag("to-")(input)?;
    let (input, dest) = terminated(alpha1, tuple((space1, tag("map:"), space0, newline)))(input)?;
    let (input, conversions) = separated_list0(newline, preceded(space0, parse_conversion))(input)?;
    Ok((
        input,
        AMap {
            source: source.to_string(),
            dest: dest.to_string(),
            maps: conversions,
        },
    ))
}

fn parse_file(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = terminated(parse_seeds, many1(newline))(input)?;
    let (input, maps) =
        terminated(separated_list0(many1(newline), parse_map), many0(newline))(input)?;

    Ok((input, Almanac { seeds, maps }))
}

#[derive(Debug, PartialEq, Eq)]
struct AMap {
    source: String,
    dest: String,
    maps: Vec<Conversion>,
}

impl AMap {
    pub fn convert(&self, source: u64) -> u64 {
        if let Some(c) = self.maps.iter().find(|c| {
            source >= c.source_range_start && source < c.source_range_start + c.range_length
        }) {
            source - c.source_range_start + c.dest_range_start
        } else {
            source
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Conversion {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl From<(u64, u64, u64)> for Conversion {
    fn from(value: (u64, u64, u64)) -> Self {
        Conversion {
            dest_range_start: value.0,
            source_range_start: value.1,
            range_length: value.2,
        }
    }
}

pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AMap>,
}

impl Almanac {
    pub fn from_file(filename: &str) -> Result<Almanac, String> {
        let input = read_to_string(filename).map_err(|e| format!("read_to_string: {e}"))?;
        let result = all_consuming(parse_file)(&input)
            .map_err(|e| e.to_owned())
            .map_err(|e| format!("parse error: {e}"))?;
        Ok(result.1)
    }

    pub fn find_lowest_location(&self) -> Result<u64, String> {
        self.seeds
            .iter()
            .map(|s| self.find_location(*s))
            .collect::<Result<Vec<u64>, String>>()?
            .into_iter()
            .min()
            .ok_or("no seeds".to_owned())
    }
    pub fn find_lowest_location2(&self) -> Result<u64, String> {
        Ok(self
            .seeds
            .chunks(2)
            .map(|pair| {
                let start = pair[0];
                let end = pair[1];
                println!("For range {start} -> {end} => lowest = ...");
                let r = (start..(start + end))
                    .try_fold(u64::MAX, |acc, s| {
                        Result::<u64, String>::Ok(std::cmp::min(acc, self.find_location(s)?))
                    })
                    .unwrap();
                println!("For range {start} -> {end} => lowest = {}", r);
                r
            })
            .fold(u64::MAX, |acc, v| std::cmp::min(acc, v)))
    }

    pub fn find_location(&self, seed: u64) -> Result<u64, String> {
        self.find_path("seed", "location", seed)
    }

    pub fn find_path(&self, origin: &str, dest: &str, value: u64) -> Result<u64, String> {
        let mut current = origin;
        let mut current_value = value;

        while current != dest {
            if let Some(map) = self.maps.iter().find(|m| m.source == current) {
                current = &map.dest;
                current_value = map.convert(current_value);
            } else {
                return Err(format!(
                    "Stuck on type {current} with value {current_value}. No conversion available."
                ));
            }
        }
        Ok(current_value)
    }
}

#[cfg(test)]
mod tests {
    use nom::{combinator::all_consuming, IResult};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("seeds: 79 14 55 13", &[79, 14, 55, 13])]
    fn test_seeds(#[case] input: &str, #[case] result: &[u64]) {
        assert_eq!(parse_seeds(input).unwrap().1, result)
    }

    const T1: &str = "light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13";

    #[rstest]
    #[case(T1, "light", "temperature", vec![(45, 77, 23), (81,45,19), (68, 64, 13)])]
    fn test_map(
        #[case] input: &str,
        #[case] source: &str,
        #[case] dest: &str,
        #[case] maps: Vec<(u64, u64, u64)>,
    ) {
        let result = all_consuming(parse_map)(input).unwrap().1;
        assert_eq!(result.source, source);
        assert_eq!(result.dest, dest);

        // wtf can I not get this to work with blanket impls...???
        let conv = maps
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<Conversion>>();
        assert_eq!(result.maps, conv);
    }

    #[rstest]
    #[case("sample", 7)]
    #[case("input", 7)]
    fn test_parse_file(#[case] filename: &str, #[case] count_maps: u32) {
        let a = Almanac::from_file(filename).unwrap();
        assert_eq!(a.maps.len(), count_maps as usize)
    }

    const T2: &str = "seed-to-soil map:
    50 98 2
    52 50 48";

    #[rstest]
    #[case(T2, vec![(98, 50), (99, 51), (53, 55), (10, 10), (79,81), (14,14), (55, 57), (13,13)])]
    fn test_conversion(#[case] input: &str, #[case] tests: Vec<(u64, u64)>) {
        let map = all_consuming(parse_map)(input).unwrap().1;
        for t in tests {
            assert_eq!(map.convert(t.0), t.1);
        }
    }

    #[rstest]
    #[case("sample", vec![(79, 82), (14, 43), (55, 86), (13, 35)])]
    fn test_find_location(#[case] filename: &str, #[case] tests: Vec<(u64, u64)>) {
        let a = Almanac::from_file(filename).expect("parse error");

        for (seed, location) in tests {
            assert_eq!(a.find_location(seed).expect("find_loc error"), location);
        }
    }

    #[rstest]
    #[case("sample", 35)]
    fn test_find_lowest_location(#[case] filename: &str, #[case] lowest: u64) {
        let a = Almanac::from_file(filename).expect("parse error");

        assert_eq!(a.find_lowest_location().expect("find_loc error"), lowest);
    }

    #[rstest]
    #[case("sample", 46)]
    fn test_find_lowest_location2(#[case] filename: &str, #[case] lowest: u64) {
        let a = Almanac::from_file(filename).expect("parse error");

        assert_eq!(a.find_lowest_location2().expect("find_loc error"), lowest);
    }
}
