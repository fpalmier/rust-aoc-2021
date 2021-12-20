use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use crate::AdventOfCodeError;
use crate::day10::Sign;

fn sign(input: &str) -> IResult<&str, Sign> {
    let tags = (tag("("), tag(")"),tag("{"), tag("}"),tag("["), tag("]"),tag("<"), tag(">"));
    let (input, sign) = alt(tags)(input)?;
    let sign = sign.chars().next().unwrap().into();
    Ok((input, sign))
}

pub fn parse(input: &str) -> Result<Vec<Vec<Sign>>, AdventOfCodeError> {
    let (_, data) = separated_list1(line_ending, many1(sign))(input)
        .map_err(|_| AdventOfCodeError::Parsing)?;
    Ok(data)
}

#[test]
fn test_parser() -> Result<(), AdventOfCodeError> {
    let input = include_str!("test.txt");
    let input: String = input.to_string();
    let result = parse(&input)?;

    let expected: Vec<Vec<Sign>> = vec![
        "[({(<(())[]>[[{[]{<()<>>".chars().map(|x|x.into()).collect(),
        "[(()[<>])]({[<{<<[]>>(".chars().map(|x|x.into()).collect(),
        "{([(<{}[<>[]}>{[]{[(<()>".chars().map(|x|x.into()).collect(),
        "(((({<>}<{<{<>}{[]{[]{}".chars().map(|x|x.into()).collect(),
        "[[<[([]))<([[{}[[()]]]".chars().map(|x|x.into()).collect(),
        "[{[{({}]{}}([{[{{{}}([]".chars().map(|x|x.into()).collect(),
        "{<[[]]>}<{[{[{[]{()[[[]".chars().map(|x|x.into()).collect(),
        "[<(<(<(<{}))><([]([]()".chars().map(|x|x.into()).collect(),
        "<{([([[(<>()){}]>(<<{{".chars().map(|x|x.into()).collect(),
        "<{([{{}}[<[[[<>{}]]]>[]]".chars().map(|x|x.into()).collect(),
    ];

    assert_eq!(expected, result);
    Ok(())
}