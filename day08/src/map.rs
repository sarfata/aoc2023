use std::{collections::HashMap, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, newline},
    combinator::{all_consuming, map_res},
    error::Error,
    multi::{fold_many0, many0, many1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use strum::EnumString;

#[derive(EnumString, Clone, Copy)]
pub enum Move {
    L,
    R,
}

pub type NodeLabel = String;

pub struct Nomad<'a, I: Iterator<Item = Move>> {
    starting_position: &'a str,
    pub map: &'a Map,
    pub current_label: &'a str,
    pub moves: I,
}

impl<'a, I: Iterator<Item = Move>> Nomad<'a, I> {
    fn step_m(&mut self, m: Move) {
        self.current_label = Self::step(self.map, self.current_label, m);
    }

    fn step(map: &'a Map, current_label: &'a str, m: Move) -> &'a str {
        if let Some(current_node) = map.nodes.iter().find(|n| n.0 == current_label) {
            match m {
                Move::L => &current_node.1 .0,
                Move::R => &current_node.1 .1,
            }
        } else {
            panic!("On a node which does not exist: {}", current_label)
        }
    }

    fn count_steps(&'a mut self) -> usize {
        let mut count = 0;
        let mut moves = self.map.moves.clone().into_iter();

        loop {
            let m = match moves.next() {
                Some(m) => m,
                None => {
                    moves = self.map.moves.clone().into_iter();
                    moves.next().unwrap()
                }
            };
            match Self::step(self.map, self.current_label, m) {
                x if x == "ZZZ" => {
                    count += 1;
                    break;
                }
                x => {
                    count += 1;
                    self.current_label = x
                }
            }
        }
        count
    }

    fn ghost_complete(&self) -> bool {
        self.current_label.ends_with("Z")
        // if self.current_label == &(self.starting_position[0..2].to_string() + "Z") {
        //     true
        // } else {
        //     false
        // }
    }
}

fn find_divisors(n: u64) -> Vec<u64> {
    let mut primes = vec![];
    for i in (2..n) {
        if n % i == 0 {
            primes.push(i);
        }
    }
    primes
}

pub fn count_parallel_steps(map: &Map) -> usize {
    let mut count: usize = 0;
    let mut moves = map.moves.clone().into_iter();

    let mut ghosts: Vec<_> = vec![];

    for l in map.nodes.keys().into_iter() {
        if l.ends_with("A") {
            ghosts.push(map.spawn_nomad(l));
        }
    }

    let mut periods = ghosts.iter().map(|g| 0).collect::<Vec<_>>();
    loop {
        if ghosts.iter().any(|g| g.ghost_complete()) {
            for (i, g) in ghosts.iter().enumerate() {
                if g.ghost_complete() {
                    if periods[i] == 0 {
                        periods[i] = count;
                    } else {
                        if count % periods[i] != 0 {
                            println!(
                                "Invalid period count={} period={} i={}",
                                count, periods[i], i
                            );
                        }
                    }
                }
            }
            println!(
                "{count}: {} - Periods={:?}",
                ghosts
                    .iter()
                    .map(|g| if g.ghost_complete() { "X" } else { "_" })
                    .collect::<Vec<&str>>()
                    .join(""),
                periods
            );
            if periods.iter().all(|p| *p != 0) {
                let all_divisors = periods.iter().flat_map(|p| find_divisors(*p as u64)).fold(
                    vec![],
                    |mut acc, n| {
                        if !acc.contains(&n) {
                            acc.push(n);
                        }
                        acc
                    },
                );
                let lcm = all_divisors.iter().fold(1, |acc, d| acc * d);
                println!("Periods Divisors={:?} LCM={}", all_divisors, lcm);
            }
        }
        // println!(
        //     "Ghosts positions: {:?}",
        //     ghosts
        //         .iter()
        //         .map(|g| g.current_label)
        //         .collect::<Vec<&str>>()
        // );
        if ghosts.iter().all(|g| g.ghost_complete()) {
            break;
        }

        let m = match moves.next() {
            Some(m) => m,
            None => {
                moves = map.moves.clone().into_iter();
                moves.next().unwrap()
            }
        };
        for g in ghosts.iter_mut() {
            g.step_m(m);
        }
        count += 1;
    }
    count
}

pub struct Map {
    moves: Vec<Move>,
    nodes: HashMap<NodeLabel, (NodeLabel, NodeLabel)>,
}

impl Map {
    fn spawn_nomad<'a>(&'a self, start: &'a str) -> Nomad<'a, impl Iterator<Item = Move>> {
        Nomad {
            map: &self,
            current_label: start,
            moves: self.moves.clone().into_iter(),
            starting_position: start,
        }
    }
}

fn parse_nodelabel(input: &str) -> IResult<&str, NodeLabel> {
    let (rest, label) = alphanumeric1(input)?;

    Ok((rest, label.to_string()))
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (moves, nodes)) = all_consuming(separated_pair(
            many1(map_res(
                alt((tag::<&str, &str, Error<&str>>("L"), tag("R"))),
                |letter| Move::from_str(letter),
            )),
            many0(newline),
            fold_many0(
                terminated(
                    separated_pair(
                        parse_nodelabel,
                        tag(" = "),
                        preceded(
                            tag("("),
                            terminated(
                                separated_pair(parse_nodelabel, tag(", "), parse_nodelabel),
                                tag(")"),
                            ),
                        ),
                    ),
                    many1(newline),
                ),
                HashMap::new,
                |mut acc, (k, (left, right))| {
                    acc.insert(k, (left, right));
                    acc
                },
            ),
        ))(s)
        .map_err(|e| format!("Map parse error: {e:?}"))?;

        Ok(Map {
            moves: moves,
            nodes: HashMap::from(nodes),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("sample", 2, 7)]
    #[case("input", 263, 702)]
    fn load(#[case] filename: &str, #[case] moves: usize, #[case] nodes: usize) {
        let input = std::fs::read_to_string(filename).expect("read error");
        let map = Map::from_str(&input).expect("parse error");
        assert_eq!(map.moves.len(), moves);
        assert_eq!(map.nodes.len(), nodes);
    }

    #[rstest]
    #[case("sample", 2)]
    #[case("input", 13939)]
    fn walk(#[case] filename: &str, #[case] steps: usize) {
        let input = std::fs::read_to_string(filename).expect("read error");
        let map = Map::from_str(&input).expect("parse error");
        assert_eq!(map.spawn_nomad("AAA").count_steps(), steps);
    }

    #[rstest]
    #[case("sample2", 6)]
    // #[case("input", 13939)]
    fn walk_ghost(#[case] filename: &str, #[case] steps: usize) {
        let input = std::fs::read_to_string(filename).expect("read error");
        let map = Map::from_str(&input).expect("parse error");
        assert_eq!(count_parallel_steps(&map), steps);
    }
}
