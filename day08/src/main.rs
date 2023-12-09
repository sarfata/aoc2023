use std::str::FromStr;

use map::{count_parallel_steps, Map};

mod map;

fn main() {
    println!(
        "Part1: Sample={:?} Input={:?}",
        part1("sample"),
        part1("input")
    );
    println!(
        "Part2: Sample={:?} Input={:?}",
        part2("sample2"),
        part2("input")
    );
}

fn part1(_filename: &str) -> Result<u64, String> {
    // let races = from_string(&read_to_string(&filename).map_err(|e| format!("io error {:?}", e))?)?;

    Ok(0)
}

fn part2(filename: &str) -> Result<usize, String> {
    let input = std::fs::read_to_string(filename).expect("read error");
    let map = Map::from_str(&input).expect("parse error");
    Ok(count_parallel_steps(&map))

    // 11206957317755125787748971 => too high
}
