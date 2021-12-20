use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use crate::AdventOfCodeError;

fn digit(input: &str) -> IResult<&str, u8> {
    let tags = (tag("0"), tag("1"), tag("2"), tag("3"), tag("4"), tag("5"), tag("6"), tag("7"), tag("8"), tag("9"));
    let (input, action) = alt(tags)(input)?;
    match action {
        "0" => Ok((input, 0)),
        "1" => Ok((input, 1)),
        "2" => Ok((input, 2)),
        "3" => Ok((input, 3)),
        "4" => Ok((input, 4)),
        "5" => Ok((input, 5)),
        "6" => Ok((input, 6)),
        "7" => Ok((input, 7)),
        "8" => Ok((input, 8)),
        "9" => Ok((input, 9)),
        _ => todo!(),
    }
}

pub fn parse(input: &str) -> Result<Vec<Vec<u8>>, AdventOfCodeError> {
    let (_, data) = separated_list1(line_ending, many1(digit))(input)
        .map_err(|_| AdventOfCodeError::Parsing)?;
    Ok(data)
}

#[test]
fn test_parser() -> Result<(), AdventOfCodeError> {
    let input = include_str!("test.txt");
    let input: String = input.to_string();
    let result = parse(&input)?;

    let expected: Vec<Vec<u8>> = vec![
        vec![2,1,9,9,9,4,3,2,1,0],
        vec![3,9,8,7,8,9,4,9,2,1],
        vec![9,8,5,6,7,8,9,8,9,2],
        vec![8,7,6,7,8,9,6,7,8,9],
        vec![9,8,9,9,9,6,5,6,7,8],
    ];

    assert_eq!(expected, result);
    Ok(())
}