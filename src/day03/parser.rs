use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use crate::AdventOfCodeError;

fn bool(input: &str) -> IResult<&str, bool> {
    let (input, action) = alt((tag("0"), tag("1")))(input)?;
    match action {
        "1" => Ok((input, true)),
        "0" => Ok((input, false)),
        _ => todo!(),
    }
}

pub fn parse(input: &str) -> Result<Vec<Vec<bool>>, AdventOfCodeError> {
    let (_, data) = separated_list1(line_ending, many1(bool))(input)
        .map_err(|_| AdventOfCodeError::Parsing)?;
    Ok(data)
}

#[test]
fn test_parser() -> Result<(), AdventOfCodeError> {
    let input = include_str!("test.txt");
    let input: String = input.to_string();

    let result = parse(&input)?;

    let expected = vec![
        vec!(false,false,true,false,false),
        vec!(true,true,true,true,false),
        vec!(true,false,true,true,false),
        vec!(true,false,true,true,true),
        vec!(true,false,true,false,true),
        vec!(false,true,true,true,true),
        vec!(false,false,true,true,true),
        vec!(true,true,true,false,false),
        vec!(true,false,false,false,false),
        vec!(true,true,false,false,true),
        vec!(false,false,false,true,false),
        vec!(false,true,false,true,false),
    ];
    assert_eq!(expected, result);
    
    Ok(())
}