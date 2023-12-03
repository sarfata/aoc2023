use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::satisfy,
    combinator::peek,
    multi::{many0, separated_list1},
    sequence::pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");

    match input
        .lines()
        .try_fold(0, |acc, line| Ok::<u32, &str>(acc + process_line_1(line)?))
    {
        Ok(sum) => println!("Part 1: {}", sum),
        Err(e) => println!("Error: {}", e),
    }

    match parse_file_part2(input) {
        Ok((_, sum)) => println!("Part 2: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
    // part 1: 53974
    // part 2: 52840
}

fn process_line_1(line: &str) -> Result<u32, &str> {
    let digits = line.chars().filter(|c| c.is_digit(10));
    let first = digits
        .clone()
        .next()
        .ok_or("no digits")?
        .to_digit(10)
        .unwrap();
    let last = digits
        .clone()
        .last()
        .ok_or("no digits")?
        .to_digit(10)
        .unwrap();
    Ok(first * 10 + last)
}

fn parse_digit(input: &str) -> IResult<&str, Option<u8>> {
    let (rest, c) = satisfy(|c| c.is_digit(10))(input)?;
    Ok((rest, Some(c.to_digit(10).unwrap() as u8)))
}

fn parse_noise(input: &str) -> IResult<&str, Option<u8>> {
    let (rest, c) = take(1u8)(input)?;

    if c == "\n" {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )))
    } else {
        Ok((rest, None))
    }
}

fn parse_spelledout(input: &str) -> IResult<&str, Option<u8>> {
    // We leave the last letter in the input because it might be needed to form the next number
    let (rest, c) = alt((
        pair(tag("on"), peek(tag("e"))),
        pair(tag("tw"), peek(tag("o"))),
        pair(tag("thre"), peek(tag("e"))),
        pair(tag("fou"), peek(tag("r"))),
        pair(tag("fiv"), peek(tag("e"))),
        pair(tag("si"), peek(tag("x"))),
        pair(tag("seve"), peek(tag("n"))),
        pair(tag("eigh"), peek(tag("t"))),
        pair(tag("nin"), peek(tag("e"))),
    ))(input)?;
    Ok((
        rest, // + c.chars().last().unwrap(),
        Some(match c.0 {
            "on" => 1,
            "tw" => 2,
            "thre" => 3,
            "fou" => 4,
            "fiv" => 5,
            "si" => 6,
            "seve" => 7,
            "eigh" => 8,
            "nin" => 9,
            _ => unreachable!(),
        }),
    ))
}

fn parse_line_tokens_part2(input: &str) -> IResult<&str, Vec<u8>> {
    let (rest, result) = many0(alt((parse_digit, parse_spelledout, parse_noise)))(input)?;
    let valids = result.iter().filter_map(|c| *c).collect();
    Ok((rest, valids))
}

fn parse_line_part2(line: &str) -> IResult<&str, u32> {
    let (rest, tokens) = parse_line_tokens_part2(line)?;

    match (tokens.first(), tokens.last()) {
        (Some(first), Some(last)) => Ok((rest, (*first as u32) * 10 + (*last as u32))),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            line,
            nom::error::ErrorKind::Eof,
        ))),
    }
}

fn parse_file_part2(input: &str) -> IResult<&str, u32> {
    let (rest, values) = separated_list1(tag("\n"), parse_line_part2)(input)?;
    Ok((rest, values.iter().sum()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_digits() {
        assert_eq!(
            super::parse_line_tokens_part2("1234").unwrap().1,
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            super::parse_line_tokens_part2("912").unwrap().1,
            vec![9, 1, 2]
        );
        assert_eq!(
            super::parse_line_tokens_part2("a912").unwrap().1,
            vec![9, 1, 2]
        );
        assert_eq!(
            super::parse_line_tokens_part2("a9three12").unwrap().1,
            vec![9, 3, 1, 2]
        );
        assert_eq!(
            super::parse_line_tokens_part2("two13five6").unwrap().1,
            vec![2, 1, 3, 5, 6]
        );
    }

    #[test]
    fn parse_line() {
        assert_eq!(super::process_line_1("1234").unwrap(), 14);
        assert_eq!(super::parse_line_part2("three1234").unwrap().1, 34);
        assert_eq!(super::parse_line_part2("oneight").unwrap().1, 18);
    }
}
