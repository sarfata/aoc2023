use std::{collections::HashMap, fs::read_to_string};

use num_bigint::{BigInt, ToBigInt};
use races::*;

mod races;
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

    // // sample
    // let T = 71530.to_bigint().unwrap();
    // let D = 940200.to_bigint().unwrap();
    // part2_bigint(T, D);

    // // actual
    // let T = 59707878.to_bigint().unwrap();
    // let D = 430121812131276i64.to_bigint().unwrap();
    // part2_bigint(T, D);
    // // 42948148 => too low
}

fn part1(filename: &str) -> Result<u64, String> {
    let races = from_string(&read_to_string(&filename).map_err(|e| format!("io error {:?}", e))?)?;
    // Almanac::from_file(filename)?.find_lowest_location()

    Ok(races
        .iter()
        .fold(1, |acc, race| acc * race.count_ways_to_win()))
}

fn part2(filename: &str) -> Result<u64, String> {
    // Almanac::from_file(filename)?.find_lowest_location2()
    Ok(0)
}

fn part2_float(T: f64, D: f64) -> u64 {
    let x1 = 0.5 * (T - (T.powi(2) - 4.0 * D).sqrt());
    let x2 = 0.5 * (T + (T.powi(2) - 4.0 * D).sqrt());

    let solution = x2.floor() - x1.ceil() + 1.0;
    println!("x1={x1} x2={x2} x2-x1={solution}");
    solution as u64
}

fn part2_bigint(T: BigInt, D: BigInt) -> Result<BigInt, String> {
    let x1: BigInt = (&T - (T.pow(2) - 4.to_bigint().unwrap() * &D).sqrt()) / 2;
    let x2: BigInt = (&T + (T.pow(2) - 4.to_bigint().unwrap() * D).sqrt()) / 2;

    let solution = &x2 - &x1;
    println!("x1={x1} x2={x2} x2-x1={solution}");
    Ok(solution)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{part2_float, races::Race};

    #[rstest]
    #[case("71530", "940200", 71503)]
    #[case("59707878", "430121812131276", 42948149)]
    fn test_part2(#[case] t_digits: &str, #[case] d_digits: &str, #[case] result: u64) {
        assert_eq!(
            part2_float(
                t_digits.parse::<f64>().unwrap(),
                d_digits.parse::<f64>().unwrap()
            ),
            result
        );
    }

    #[rstest]
    #[case(71530, 940200, 71503)]
    #[case(59707878, 430121812131276, 42948149)]
    fn test_part2_simple(#[case] time: u64, #[case] distance: u64, #[case] result: u64) {
        assert_eq!(
            Race {
                time,
                record_distance: distance
            }
            .count_ways_to_win(),
            result
        );
    }
}
