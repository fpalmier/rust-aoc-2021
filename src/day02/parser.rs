use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use crate::AdventOfCodeError;
use crate::day02::Action;

fn num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn action(input: &str) -> IResult<&str, Action> {
    let (input, action) = alt((tag("forward"), tag("up"), tag("down")))(input)?;
    match action {
        "forward" => Ok((input, Action::Forward)),
        "up" => Ok((input, Action::Up)),
        "down" => Ok((input, Action::Down)),
        _ => todo!()
    }
}

#[test]
fn test_parse_action() {
    assert_eq!(Ok(("", Action::Forward)), action("forward"));
    assert_eq!(Ok(("", Action::Up)), action("up"));
    assert_eq!(Ok(("", Action::Down)), action("down"));
}


pub fn parse(input: &str) -> Result<Vec<(Action, usize)>, AdventOfCodeError> {
    let command = separated_pair(action, tag(" "), num);
    let (_, data) = separated_list1(line_ending, command)(input)
        .map_err(|_| AdventOfCodeError::Parsing)?;
    Ok(data)
}


#[test]
fn test_parser() -> Result<(), AdventOfCodeError> {
    let input = include_str!("test.txt");
    let input: String = input.to_string();

    let result = parse(&input)?;

    let expected = vec![
        (Action::Forward, 5),
        (Action::Down, 5),
        (Action::Forward, 8),
        (Action::Up, 3),
        (Action::Down, 8),
        (Action::Forward, 2),
    ];
    assert_eq!(expected, result);

    Ok(())
}