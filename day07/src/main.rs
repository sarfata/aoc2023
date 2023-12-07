use std::{collections::HashMap, fs::read_to_string};

// mod camel_cards;
mod camel_cards_joker;

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
    // let races = from_string(&read_to_string(&filename).map_err(|e| format!("io error {:?}", e))?)?;

    Ok(0)
}

fn part2(filename: &str) -> Result<u64, String> {
    // Almanac::from_file(filename)?.find_lowest_location2()
    Ok(0)
}
