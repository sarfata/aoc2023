use std::{collections::HashMap, fs::read_to_string};

use almanac::Almanac;

mod almanac;
fn main() {
    println!(
        "Part1: Sample={:?} Input={:?}",
        part1("sample"),
        part1("input")
    );
    println!(
        "Part2: Sample={:?} Input={:?}",
        part2("sample"),
        part2("input")
    );
}

fn part1(filename: &str) -> Result<u64, String> {
    Almanac::from_file(filename)?.find_lowest_location()
}

fn part2(filename: &str) -> Result<u64, String> {
    Almanac::from_file(filename)?.find_lowest_location2()
}
