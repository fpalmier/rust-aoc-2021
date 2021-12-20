use nom::{
    IResult,
    combinator::{
        map_res,
        recognize,
    },
    multi::{
        many0,
        separated_list1,
    },
    bytes::complete::tag,
    character::complete::{
        digit1,
        line_ending,
    },
};
use crate::AdventOfCodeError;
use crate::AdventOfCodeError::Parsing;

fn num(input: &str) -> IResult<&str, u8> {
    let (input, _) = many0(tag(" "))(input)?;
    map_res(recognize(digit1), str::parse)(input)
}

pub fn order(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, list) = separated_list1(tag(","), num)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, list))
}

pub fn jump(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(line_ending)(input)?;
    Ok((input, ()))
}

fn row(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, list) = separated_list1(tag(" "), num)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, list))
}

#[test]
fn test_row1() {
    let input = "88 88 88 66 66\n";
    let result = row(input);
    assert_eq!(result, Ok(("", vec![88, 88, 88, 66, 66])));
}

#[test]
fn test_row2() {
    let input = "88 88  8 66 66\n";
    let result = row(input);
    assert_eq!(result, Ok(("", vec![88, 88, 8, 66, 66])));
}

#[test]
fn test_row3() {
    let input = " 8 88 88 66 66\n";
    let result = row(input);
    assert_eq!(result, Ok(("", vec![8, 88, 88, 66, 66])));
}

fn tab(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, first) = row(input)?;
    let (input, second) = row(input)?;
    let (input, third) = row(input)?;
    let (input, fourth) = row(input)?;
    let (input, fifth) = row(input)?;

    let t = [first, second, third, fourth, fifth].concat();

    Ok((input, t))
}

fn tabs(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    let (input, tab) = separated_list1(line_ending, tab)(input)?;
    Ok((input, tab))
}

pub fn parse(input: &str) -> Result<(Vec<u8>, Vec<Vec<u8>>), AdventOfCodeError> {
    let (input, order) = order(input)
        .map_err(|_| Parsing)?;

    let (input, _) = jump(input)
        .map_err(|_| Parsing)?;

    let (_, tabs) = tabs(input)
        .map_err(|_| Parsing)?;

    Ok((order, tabs))
}