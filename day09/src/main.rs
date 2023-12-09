use std::str::FromStr;

mod readings;

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
    Ok(0)
}
