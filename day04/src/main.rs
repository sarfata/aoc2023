use std::{collections::HashMap, fs::read_to_string};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    multi::separated_list0,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

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

fn part1(filename: &str) -> Result<u32, &'static str> {
    let input = read_to_string(filename).map_err(|_e| "read error")?;
    if let Ok((rest, cards)) = parse_file(&input) {
        if rest.len() > 0 {
            println!("rest is {rest}");
        }
        Ok(cards.iter().map(|c| c.points()).sum())
    } else {
        Err("parse error")
    }
}

fn part2(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = read_to_string(filename).map_err(|_e| "read error")?;
    if let Ok((_, cards)) = parse_file(&input) {
        let mut g = Game::new(cards);
        g.play()?;
        Ok(g.count_cards())
    } else {
        Err("parse error".into())
    }
}

struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn count_match(&self) -> u32 {
        self.numbers
            .iter()
            .fold(0, |acc, n| match self.winning.contains(n) {
                true => acc + 1,
                false => acc,
            })
    }

    fn points(&self) -> u32 {
        match self.count_match() {
            0 => 0,
            c => 2u32.pow(c - 1),
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, Card> {
    let (input, _id) = preceded(preceded(tag("Card"), space1), digit1)(input)?;
    let (input, _) = preceded(tag(":"), space1)(input)?;
    let (input, num_lists) = separated_pair(
        separated_list0(space1, digit1),
        tuple((space0, tag("|"), space0)),
        separated_list0(space1, digit1),
    )(input)?;

    Ok((
        input,
        Card {
            winning: num_lists.0.iter().map(|s| s.parse().unwrap()).collect(),
            numbers: num_lists.1.iter().map(|s| s.parse().unwrap()).collect(),
        },
    ))
}

fn parse_file(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list0(tag("\n"), parse_line)(input)
}

struct Game {
    cards: HashMap<u32, u32>,
    // Note: I don't think I needed the queue because you are always modifying the counts below you so you can just walk the counts and make the changes.
    // scratch_queue: VecDeque<u32>,
    counts: Vec<u32>,
}

impl Game {
    fn new(cards: Vec<Card>) -> Self {
        let mut counts = Vec::with_capacity(cards.len());
        // let mut scratch_queue = VecDeque::with_capacity(cards.len());
        for _ in 0..cards.len() {
            counts.push(1);
            // scratch_queue.push_back(i as u32 + 1);
        }

        Game {
            cards: cards
                .iter()
                .enumerate()
                .map(|(i, c)| (i as u32 + 1, c.count_match()))
                .collect(),
            // scratch_queue,
            counts,
        }
    }

    /*
    first solution with a queue
    fn scratch(&mut self) -> Result<bool, &'static str> {
        let cardIndex = self.scratch_queue.pop_front().ok_or("nothing to scratch")?;
        let matching_numbers = self.cards.get(&cardIndex).ok_or("no such card")?;

        // Update the count of cards we have
        for i in cardIndex as usize + 1..=(cardIndex + matching_numbers) as usize {
            if i > self.counts.len() {
                panic!(
                    "reaching beyond end of the table card (index={cardIndex} i={i} len={})",
                    self.counts.len()
                );
            }
            self.counts[i - 1] = self.counts[i - 1] + 1;

            // Add these cards to our queue
            self.scratch_queue.push_back(i as u32);
        }

        Ok(!self.scratch_queue.is_empty())
    }

    fn play_slow(&mut self) -> Result<(), &'static str> {
        while self.scratch()? {}
        Ok(())
    }
     */

    fn play(&mut self) -> Result<(), &'static str> {
        for i in 0..self.counts.len() {
            println!("{i}: {:?}", self.counts);
            let matching_numbers = *self.cards.get(&(i as u32 + 1)).ok_or("no such card")?;

            for j in i + 1..=i + matching_numbers as usize {
                self.counts[j] = self.counts[j] + self.counts[i];
            }
        }
        Ok(())
    }

    fn count_cards(&self) -> u32 {
        self.counts.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn play_game() {
        let mut game = Game::new(
            parse_file(include_str!("../sample"))
                .expect("parse sample error")
                .1,
        );

        // game.scratch().expect("scratch error");
        // assert_eq!(game.counts, [1, 2, 2, 2, 2, 1]);
        // assert_eq!(game.scratch_queue, [2, 3, 4, 5, 6, 2, 3, 4, 5]);
        game.play().expect("play error");
        // assert_eq!(game.scratch_queue, []);
        assert_eq!(game.counts[0..6], [1, 2, 4, 8, 14, 1]);
        assert_eq!(game.count_cards(), 30);
    }

    #[rstest]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", &[31, 18, 13, 56, 72], &[74, 77, 10, 23, 35, 67, 36, 11])]
    #[case("Card 6:  1 18 13 56 72 | 74 77 10 23 35 67 36 11", &[1, 18, 13, 56, 72], &[74, 77, 10, 23, 35, 67, 36, 11])]
    #[case("Card   6:  1 18  3 56 72 | 74 77 10 23 35 67 36 11", &[1, 18,  3, 56, 72], &[74, 77, 10, 23, 35, 67, 36, 11])]
    #[case("Card   6:  1 18  3 56 72 |  4 77 10 23 35 67 36 11", &[1, 18,  3, 56, 72], &[4, 77, 10, 23, 35, 67, 36, 11])]
    fn test_parser(#[case] input: &str, #[case] winning: &[u32], #[case] numbers: &[u32]) {
        let card = parse_line(input).expect("parse error").1;

        assert_eq!(card.winning, winning);
        assert_eq!(card.numbers, numbers);
    }

    #[rstest]
    #[case(&[13, 32, 20, 16, 61], &[61, 30, 68, 82, 17, 32, 24, 19], 2)]
    #[case(&[31, 18, 13, 56, 72], &[74, 77, 10, 23, 35, 67, 36, 11], 0)]
    fn test_count_match(#[case] winning: &[u32], #[case] numbers: &[u32], #[case] matches: u32) {
        let card = Card {
            winning: winning.to_owned(),
            numbers: numbers.to_owned(),
        };
        assert_eq!(card.count_match(), matches)
    }

    #[rstest]
    #[case(&[41, 48, 83, 86, 17], &[83, 86, 6, 31, 17, 9, 48, 53], 8)]
    #[case(&[13, 32, 20, 16, 61], &[61, 30, 68, 82, 17, 32, 24, 19], 2)]
    #[case(&[31, 18, 13, 56, 72], &[74, 77, 10, 23, 35, 67, 36, 11], 0)]
    fn test_points(#[case] winning: &[u32], #[case] numbers: &[u32], #[case] matches: u32) {
        let card = Card {
            winning: winning.to_owned(),
            numbers: numbers.to_owned(),
        };
        assert_eq!(card.points(), matches)
    }
}
