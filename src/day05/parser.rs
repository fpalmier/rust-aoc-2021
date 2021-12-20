use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use crate::AdventOfCodeError;

fn num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn coordinate(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(num, tag(","), num)(input)
}

pub fn parse(input: &str) -> Result<Vec<((usize, usize),(usize, usize))>, AdventOfCodeError> {
    let line = separated_pair(coordinate, tag(" -> "), coordinate);
    let (_, data) = separated_list1(line_ending, line)(input)
        .map_err(|_| AdventOfCodeError::Parsing)?;
    Ok(data)
}

#[test]
fn test_parser() -> Result<(), AdventOfCodeError> {
    let input = include_str!("test.txt");
    let input: String = input.to_string();

    let result = parse(&input)?;

    let expected: Vec<((usize, usize),(usize, usize))> = vec![
        ((0,9),(5,9)),
        ((8,0),(0,8)),
        ((9,4),(3,4)),
        ((2,2),(2,1)),
        ((7,0),(7,4)),
        ((6,4),(2,0)),
        ((0,9),(2,9)),
        ((3,4),(1,4)),
        ((0,0),(8,8)),
        ((5,5),(8,2)),
    ];
    assert_eq!(expected, result);

    Ok(())
}