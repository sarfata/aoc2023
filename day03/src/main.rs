use std::str::FromStr;
use std::{cmp::max, fs::read_to_string};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Part {
    pn: u32,
    x: u32,
    width: u32,
    y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Symbol {
    symbol: char,
    x: u32,
    y: u32,
}

struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
    width: u32,
    height: u32,
}
impl Part {
    fn is_adjacent_to(&self, s: &Symbol) -> bool {
        s.y.abs_diff(self.y) <= 1 && (s.x as i32 >= self.x as i32 - 1 && s.x <= self.x + self.width)
    }
    fn count_adjacent_symbols(&self, symbols: &[Symbol]) -> usize {
        symbols.iter().filter(|s| self.is_adjacent_to(s)).count()
    }
}

impl Symbol {
    fn filter_adjacent_parts<'a>(&'a self, parts: &'a [Part]) -> impl Iterator<Item = &'a Part> {
        parts.iter().filter(|part| part.is_adjacent_to(self))
    }
}

impl FromStr for Schematic {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = Vec::new();
        let mut symbols = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in s.lines().enumerate() {
            height = max(height, y + 1);

            let mut current_part: Option<Part> = None;
            for (x, c) in line.chars().enumerate() {
                width = max(width, x + 1);
                match c {
                    '0'..='9' => match current_part.as_mut() {
                        Some(part) => {
                            part.pn = part.pn * 10 + c.to_digit(10).unwrap();
                            part.width += 1;
                        }
                        None => {
                            current_part = Some(Part {
                                x: x as u32,
                                y: y as u32,
                                width: 1,
                                pn: c.to_digit(10).unwrap(),
                            })
                        }
                    },
                    '.' => match current_part {
                        Some(part) => {
                            parts.push(part);
                            current_part = None;
                        }
                        _ => {}
                    },
                    _ => {
                        if let Some(part) = current_part {
                            parts.push(part);
                            current_part = None;
                        }
                        symbols.push(Symbol {
                            symbol: c,
                            x: x as u32,
                            y: y as u32,
                        })
                    }
                }
            }

            if let Some(part) = current_part {
                parts.push(part);
            }
        }
        Ok(Schematic {
            parts,
            symbols,
            width: width as u32,
            height: height as u32,
        })
    }
}

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

/// Count the sum of all the part numbers that are adjacent to at least one symbol
fn part1(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let schema: Schematic = read_to_string(filename)?.parse()?;

    let total = schema
        .parts
        .iter()
        .filter(|part| part.count_adjacent_symbols(&schema.symbols) > 0)
        .fold(0, |acc, part| acc + part.pn);
    Ok(total)
    // 7342190 => too high... (was counting symbols one space off to the right)
    // 7339244 => too high... (I was not reseting current_part on symbols)
    // 532445
}

/// Count all 'gear parts' (parts touching a '*'), only when there are two
/// parts, multiply them together to get their 'gear ratio' and sum them all up
fn part2(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let schema: Schematic = read_to_string(filename)?.parse()?;

    Ok(schema
        .symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .filter_map(|s| {
            let gear_parts = s.filter_adjacent_parts(&schema.parts).collect::<Vec<_>>();
            if gear_parts.len() == 2 {
                Some(gear_parts[0].pn * gear_parts[1].pn)
            } else {
                None
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("...\n...", 3, 2, vec![], vec![])]
    #[case("1..\n...", 3, 2, vec![Part {pn: 1, x: 0, y: 0, width: 1}], vec![])]
    #[case("12.\n...", 3, 2, vec![Part {pn: 12, x: 0, y: 0, width: 2}], vec![])]
    #[case("123\n.$.", 3, 2, vec![Part {pn: 123, x: 0, y: 0, width: 3}], vec![Symbol{symbol: '$', x: 1, y: 1}])]
    #[case("123$456", 7, 1, vec![Part {pn: 123, x: 0, y: 0, width: 3}, Part {pn:456, x:4, width: 3, y:0}], vec![Symbol{symbol: '$', x: 3, y: 0}])]
    fn test_parser(
        #[case] schema: &str,
        #[case] width: u32,
        #[case] height: u32,
        #[case] parts: Vec<Part>,
        #[case] symbols: Vec<Symbol>,
    ) {
        let schema: Schematic = schema.parse().expect("parse error");

        assert_eq!(schema.width, width);
        assert_eq!(schema.height, height);
        assert_eq!(schema.parts, parts);
        assert_eq!(schema.symbols, symbols);
    }
    #[rstest]
    #[case(Part {x: 0, y: 0, width: 3, pn: 0}, &[], 0)]
    #[case(Part {x: 0, y: 0, width: 3, pn: 0}, &[Symbol {x: 3, y: 0, symbol: '$'}], 1)]
    #[case(Part {x: 0, y: 0, width: 3, pn: 0}, &[Symbol {x: 4, y: 0, symbol: '$'}], 0)]
    #[case(Part {x: 0, y: 0, width: 3, pn: 0}, &[Symbol {x: 5, y: 0, symbol: '$'}], 0)]
    #[case(Part {x: 0, y: 0, width: 3, pn: 0}, &[Symbol {x: 4, y: 3, symbol: '$'}], 0)]
    // Try to find edge case...
    #[case(Part {x: 2, y: 1, width: 3, pn: 0}, &[Symbol {x: 0, y: 0, symbol: '$'}, Symbol {x: 5, y: 0, symbol: '*'}], 1)]
    #[case(Part {x: 2, y: 1, width: 3, pn: 0}, &[Symbol {x: 0, y: 1, symbol: '$'}, Symbol {x: 5, y: 1, symbol: '*'}], 1)]
    #[case(Part {x: 2, y: 1, width: 3, pn: 0}, &[Symbol {x: 0, y: 2, symbol: '$'}, Symbol {x: 5, y: 2, symbol: '*'}], 1)]
    #[case(Part {x: 2, y: 1, width: 3, pn: 0}, &[Symbol {x: 0, y: 0, symbol: '$'}, Symbol {x: 6, y: 0, symbol: '*'}], 0)]
    #[case(Part {x: 2, y: 1, width: 3, pn: 0}, &[Symbol {x: 0, y: 1, symbol: '$'}, Symbol {x: 6, y: 1, symbol: '*'}], 0)]
    #[case(Part {x: 2, y: 1, width: 3, pn: 0}, &[Symbol {x: 0, y: 2, symbol: '$'}, Symbol {x: 6, y: 2, symbol: '*'}], 0)]
    fn test_adjacent(#[case] part: Part, #[case] symbols: &[Symbol], #[case] count: usize) {
        assert_eq!(part.count_adjacent_symbols(symbols), count);
    }
}
