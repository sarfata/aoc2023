use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

pub struct Round {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

fn parse_round(mut input: &str) -> IResult<&str, Round> {
    let mut round = Round {
        blue: 0,
        red: 0,
        green: 0,
    };

    loop {
        let (rest, (value, _, color)) = tuple((
            digit1,
            tag(" "),
            alt((tag("red"), tag("green"), tag("blue"))),
        ))(input)?;
        match color {
            "red" => round.red = value.parse().unwrap(),
            "green" => round.green = value.parse().unwrap(),
            "blue" => round.blue = value.parse().unwrap(),
            _ => panic!("unexpected color"),
        }
        let (rest, separator) = opt(tag(", "))(rest)?;
        input = rest;
        if separator.is_none() {
            break;
        }
    }

    Ok((input, round))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list0(tag("; "), parse_round)(input)?;

    Ok((
        input,
        Game {
            id: id.parse().unwrap(),
            rounds,
        },
    ))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(tag("\n"), parse_game)(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("8 green, 6 blue, 20 red", (20, 8, 6))]
    #[case("5 blue, 4 red, 13 green", (4, 13, 5))]
    #[case("5 green, 1 red", (1, 5, 0))]
    #[case("3 green", (0, 3, 0))]
    fn test_parse_round(#[case] input: &str, #[case] rgb: (u32, u32, u32)) {
        let round: Round = parse_round(input).expect("parse failed").1;

        assert_eq!(round.red, rgb.0);
        assert_eq!(round.green, rgb.1);
        assert_eq!(round.blue, rgb.2);
    }

    #[rstest]
    #[case("Game 1: \n", 1, &[])]
    #[case("Game 1: 3 green\n", 1, &[(0, 3, 0)])]
    #[case("Game 1: 3 green", 1, &[(0, 3, 0)])]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
        3
        ,
        &[(20, 8, 6), (4, 13, 5), (1, 5, 0)]
    )]
    fn test_parse_game0(#[case] input: &str, #[case] id: u32, #[case] rounds: &[(u32, u32, u32)]) {
        let game: Game = parse_game(input).expect("parse failed").1;

        assert_eq!(game.id, id);
        assert_eq!(game.rounds.len(), rounds.len());
        for (i, r) in rounds.iter().enumerate() {
            assert_eq!(game.rounds[i].red, r.0);
            assert_eq!(game.rounds[i].green, r.1);
            assert_eq!(game.rounds[i].blue, r.2);
        }
    }
}
