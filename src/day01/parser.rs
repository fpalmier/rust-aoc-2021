use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map_res};
use nom::IResult;
use nom::multi::separated_list1;
use crate::AdventOfCodeError;

fn num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

pub fn parse(input: &str) -> Result<Vec<usize>, AdventOfCodeError> {
    let (_, num) = separated_list1(line_ending, num)(input)
        .map_err(|_| AdventOfCodeError::Parsing)?;
    Ok(num)
}


#[test]
fn test_parser() -> Result<(), AdventOfCodeError> {
    let input = include_str!("test.txt");
    let input: String = input.to_string();

    let result = parse(&input)?;

    let expected = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(expected, result);

    Ok(())
}