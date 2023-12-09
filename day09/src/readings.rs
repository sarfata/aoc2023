use std::{fs::read_to_string, io, num::ParseIntError, str::FromStr, string::ParseError};

fn differences(v: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(v.len() - 1);

    for i in 1..v.len() {
        result.push(v[i] - v[i - 1]);
    }
    result
}

pub fn extrapolate_next(v: &[i64]) -> i64 {
    let d = differences(v);
    if d.iter().all(|x| *x == 0) {
        // All the numbers are the same
        v[0]
    } else {
        v[v.len() - 1] + extrapolate_next(&d)
    }
}
pub fn extrapolate_previous(v: &[i64]) -> i64 {
    let d = differences(v);
    if d.iter().all(|x| *x == 0) {
        // All the numbers are the same
        v[0]
    } else {
        v[0] - extrapolate_previous(&d)
    }
}

struct SensorMeasure {
    readings: Vec<i64>,
}

impl SensorMeasure {
    pub fn next_reading(&self) -> i64 {
        extrapolate_next(&self.readings)
    }
    pub fn previous_reading(&self) -> i64 {
        extrapolate_previous(&self.readings)
    }
    pub fn from_file(filename: &str) -> Result<Vec<Self>, std::io::Error> {
        let input = read_to_string(filename)?;
        Ok(input
            .split("\n")
            .map(|line| <SensorMeasure as std::str::FromStr>::from_str(line).expect("parse error"))
            .filter(|x| !x.readings.is_empty())
            .collect::<Vec<SensorMeasure>>())
    }
}

impl FromStr for SensorMeasure {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let readings = s
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<i64>, ParseIntError>>()
            .map_err(|e| format!("Parse {:?}", e))?;

        Ok(SensorMeasure { readings })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("sample", 3)]
    #[case("input", 200)]
    fn read(#[case] filename: &str, #[case] readings: usize) {
        let sensor_data = SensorMeasure::from_file(filename).expect("reading");
        assert_eq!(sensor_data.len(), readings);
    }

    #[rstest]
    #[case(vec![1,2,3,4,5], vec![1,1,1,1])]
    fn test_differences(#[case] i: Vec<i64>, #[case] out: Vec<i64>) {
        let r = differences(&i);
        assert_eq!(r.len(), out.len());
    }

    #[rstest]
    #[case(vec![1,1,1,1,1], 1)]
    #[case(vec![1,2,3,4,5], 6)]
    fn test_next(#[case] i: Vec<i64>, #[case] out: i64) {
        let r = extrapolate_next(&i);
        assert_eq!(r, out);
    }

    #[rstest]
    #[case("sample", 114)]
    #[case("input", 2038472161)]
    fn test_part1(#[case] filename: &str, #[case] result: i64) {
        let sensor_data = SensorMeasure::from_file(filename).expect("reading");
        let r = sensor_data
            .iter()
            .fold(0, |acc, sm| acc + sm.next_reading());
        assert_eq!(r, result);
    }

    #[rstest]
    #[case("sample", 2)]
    #[case("input", 1091)]
    fn test_part2(#[case] filename: &str, #[case] result: i64) {
        let sensor_data = SensorMeasure::from_file(filename).expect("reading");
        let r = sensor_data
            .iter()
            .fold(0, |acc, sm| acc + sm.previous_reading());
        assert_eq!(r, result);
    }
}
