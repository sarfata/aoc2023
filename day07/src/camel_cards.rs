use std::fs::read_to_string;
use std::{str::FromStr, string::ParseError};

use nom::character::complete::alphanumeric1;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{
    character::complete::{alpha1, digit1, newline, space1},
    combinator::{all_consuming, map_res},
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
};
use strum::EnumString;

#[derive(Debug, Clone, Copy, EnumString, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Clone, Eq, PartialEq, Debug, PartialOrd, Ord)]
enum Hand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPairs(Vec<Card>),
    ThreeOfKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfKind(Vec<Card>),
    FiveOfKind(Vec<Card>),
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() != 5 {
            return Err(format!("Wrong input string length: {}", input.len()));
        }

        let mut cards = input
            .chars()
            .map(|c| Card::from_str(&c.to_string()).map_err(|e| format!("parse error {e:?}")))
            .collect::<Result<Vec<Card>, String>>()?;

        let hand_cards = cards.clone();

        cards.sort();
        cards.reverse();
        let mut groups = cards
            .into_iter()
            .fold(vec![] as Vec<(Card, u8)>, |mut acc, next| {
                if let Some(previous) = acc.last_mut() {
                    if previous.0 == next {
                        previous.1 = previous.1 + 1;
                        return acc;
                    }
                }
                acc.push((next, 1));
                acc
            });
        groups.sort_by(|a, b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        Ok(match groups.len() {
            1 => Hand::FiveOfKind(hand_cards),
            2 => {
                if groups[0].1 == 4 {
                    Hand::FourOfKind(hand_cards)
                } else if groups[0].1 == 3 {
                    Hand::FullHouse(hand_cards)
                } else {
                    unreachable!("2 groups - len[0]={} groups: {:?}", groups[0].1, groups)
                }
            }
            3 => {
                if groups[0].1 == 3 {
                    Hand::ThreeOfKind(hand_cards)
                } else if groups[0].1 == 2 {
                    Hand::TwoPairs(hand_cards)
                } else {
                    unreachable!("3 groups")
                }
            }
            4 => {
                if groups[0].1 == 2 {
                    Hand::OnePair(hand_cards)
                } else {
                    unreachable!("4 groups")
                }
            }
            5 => Hand::HighCard(hand_cards),
            _ => unreachable!("groups.len"),
        })
    }
}

pub struct HandList(Vec<(Hand, u32)>);

impl HandList {
    fn parse(input: &str) -> Result<Self, String> {
        let (_, hands) = all_consuming(terminated(
            separated_list1(
                newline::<&str, Error<&str>>,
                separated_pair(
                    map_res(alphanumeric1, |cards| Hand::from_str(cards)),
                    space1,
                    map_res(digit1, |s| u32::from_str_radix(s, 10)),
                ),
            ),
            many0(newline),
        ))(input)
        .map_err(|e| format!("parse error: {e:?}"))?;

        Ok(HandList(hands))
    }

    fn winnings(&self) -> u64 {
        let mut ranked = self.0.clone();
        ranked.sort_by(|a, b| a.0.cmp(&b.0));
        println!("ranked hands: {:?}", ranked);
        ranked
            .iter()
            .enumerate()
            .fold(0, |acc, (rank, (_hand, bid))| {
                acc + (rank as u64 + 1) * *bid as u64
            })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("sample", 6440)]
    // #[case("input", 0)] // 252673619 => too low - that was when I was applying poker rules, instead of comparing cards in order
    #[case("input", 253205868)] //  => too low
    fn sample(#[case] filename: &str, #[case] result: u64) {
        let l = HandList::parse(&read_to_string(filename).unwrap()).unwrap();
        assert_eq!(l.winnings(), result)
    }

    #[rstest]
    #[case("AAAAA", Hand::FiveOfKind(vec![Card::A, Card::A, Card::A, Card::A, Card::A]))]
    #[case("A2377", Hand::OnePair(vec![Card::A, Card::Two, Card::Three, Card::Seven, Card::Seven]))]
    /*
    #[case("AAAAA", Hand::FiveOfKind(Card::A))]
    #[case("77777", Hand::FiveOfKind(Card::Seven))]
    #[case("7777J", Hand::FourOfKind(Card::Seven, Card::J))]
    #[case("777JJ", Hand::FullHouse(Card::Seven, Card::J))]
    #[case("777J2", Hand::ThreeOfKind(Card::Seven, Card::J, Card::Two))]
    #[case("778JJ", Hand::TwoPairs(Card::J, Card::Seven, Card::Eight))]
    #[case("7789J", Hand::OnePair(Card::Seven, Card::J, Card::Nine, Card::Eight))]
    #[case(
        "3789J",
        Hand::HighCard(Card::J, Card::Nine, Card::Eight, Card::Seven, Card::Three)
    )]
    */
    fn test_in(#[case] input: &str, #[case] hand: Hand) {
        assert_eq!(Hand::from_str(input).unwrap(), hand);
    }

    #[rstest]
    #[case(Card::J, Card::A)]
    #[case(Card::Seven, Card::A)]
    fn compare_cards(#[case] lower: Card, #[case] higher: Card) {
        assert!(lower < higher);
    }

    #[rstest]
    #[case("AAAA2", "AAAAA")]
    #[case("2AAAA", "33332")]
    #[case("T55J5", "QQQJA")]
    /*  WRONG. WE compare the cards given when they are otherwise of same strength.
    #[case("AAAA2", "AAAAA")]
    #[case("23456", "23457")]
    #[case("22337", "22338")]
    #[case("22337", "22448")]
    */
    fn compare_hands(#[case] lower: &str, #[case] higher: &str) {
        assert!(Hand::from_str(lower).expect("lower") < Hand::from_str(higher).expect("higher"));
    }
}
