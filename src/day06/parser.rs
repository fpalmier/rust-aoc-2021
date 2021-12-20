use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use crate::AdventOfCodeError;

fn num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

pub fn parse(input: &str) -> Result<Vec<usize>, AdventOfCodeError> {
    let (_, list) = separated_list1(tag(","), num)(input)
        .map_err(|_| AdventOfCodeError::Parsing)?;
    Ok(list)
}

#[test]
fn test_parser() -> Result<(), AdventOfCodeError> {
    let input = include_str!("test.txt");
    let input: String = input.to_string();

    let result = parse(&input)?;

    let expected: Vec<usize> = vec![3,4,3,1,2];
    assert_eq!(expected, result);

    Ok(())
}