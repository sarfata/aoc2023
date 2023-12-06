use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space0, space1},
    combinator::{all_consuming, map_res},
    multi::{many1, separated_list0},
    sequence::{preceded, terminated},
    IResult,
};

pub struct Race {
    pub time: u64,
    pub record_distance: u64,
}

impl Race {
    pub fn count_ways_to_win(&self) -> u64 {
        (0..self.time)
            .map(|button_down_time| {
                let speed = button_down_time;
                let remaining_time = self.time - button_down_time;

                let distance = speed * remaining_time;
                match distance > self.record_distance {
                    true => 1,
                    false => 0,
                }
            })
            .sum()
    }
}

pub fn from_string(input: &str) -> Result<Vec<Race>, String> {
    Ok(all_consuming(parse_input)(input)
        .map_err(|e| format!("Parse error: {:?}", e))?
        .1)
}

fn parse_int(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = terminated(
        preceded(
            tag("Time:"),
            preceded(space0, separated_list0(space1, parse_int)),
        ),
        newline,
    )(input)?;
    let (input, distances) = terminated(
        preceded(
            tag("Distance:"),
            preceded(space0, separated_list0(space1, parse_int)),
        ),
        many1(newline),
    )(input)?;

    Ok((
        input,
        times
            .iter()
            .zip(distances.iter())
            .map(|(t, d)| Race {
                time: *t,
                record_distance: *d,
            })
            .collect::<Vec<Race>>(),
    ))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::from_string;

    #[rstest]
    fn test_parser() {
        let sample = "Time:      7  15   30
Distance:  9  40  200
";

        let races = from_string(sample).unwrap();
        assert_eq!(races[0].count_ways_to_win(), 4);
        assert_eq!(races[1].count_ways_to_win(), 8);
        assert_eq!(races[2].count_ways_to_win(), 9);
    }
}
