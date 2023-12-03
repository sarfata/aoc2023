use std::{cmp::max, fs::read_to_string};

use game::parse_input;

mod game;

fn main() {
    println!(
        "Part1: Sample={:?} Input={:?}",
        part1("sample1.txt"),
        part1("input")
    );
    println!(
        "Part2: Sample={:?} Input={:?}",
        part2("sample1.txt"),
        part2("input")
    );
}

fn part1(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = read_to_string(filename)?;
    let games = parse_input(input.as_str()).unwrap().1;

    Ok(games.iter().fold(0, |acc, game| {
        if game
            .rounds
            .iter()
            .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
        {
            return acc + game.id;
        } else {
            return acc;
        }
    }))
}

fn part2(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = read_to_string(filename)?;
    let games = parse_input(input.as_str()).unwrap().1;

    Ok(games
        .iter()
        .map(|game| {
            game.rounds.iter().fold((0, 0, 0), |(r, g, b), round| {
                (max(r, round.red), max(g, round.green), max(b, round.blue))
            })
        })
        .map(|(r, g, b)| r * g * b)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1("sample1.txt").unwrap(), 8);
    }
}
