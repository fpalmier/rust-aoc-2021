use crate::AdventOfCodeError;
use crate::day10::{Kind::*, Kind, Side::*, Sign};

fn line(vec: &Vec<Sign>) -> usize {

    let mut stack: Vec<Kind> = match vec.first() {
        Some(Sign(Left, kind)) => {vec![*kind]},
        _ => return 0,
    };

    for s in vec.iter().skip(1) {
        match s.side() {
            Left => stack.push(s.kind()),
            Right => match (stack.last(), s.kind()) {
                (None, _) => return 1,
                (Some(&a), b) if a == b => {
                    stack.pop();
                },
                (Some(&a), b) if a != b => return match b {
                    Parenthesis => 3,
                    CurlyBracket => 1197,
                    SquareBracket => 57,
                    Chevron => 25137
                },
                _ => unreachable!()
            }
        };
    }

    0
}

pub(crate) fn first_part(data: &[Vec<Sign>]) -> Result<usize, AdventOfCodeError> {
    let r = data.iter()
        .map(line)
        .sum();
    Ok(r)
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {
    let data: Vec<Vec<Sign>> = vec![
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
    assert_eq!(26397, first_part(&data)?);
    Ok(())
}

fn line_2(vec: &Vec<Sign>) -> Option<usize> {

    let mut stack: Vec<Kind> = match vec.first() {
        Some(Sign(Left, kind)) => {vec![*kind]},
        _ => return None,
    };

    for s in vec.iter().skip(1) {
        match s.side() {
            Left => stack.push(s.kind()),
            Right => match (stack.last(), s.kind()) {
                (None, _) => return None,
                (Some(&a), b) if a == b => {
                    stack.pop();
                },
                (Some(&a), b) if a != b => return None,
                _ => unreachable!()
            }
        };
    }

    return match stack.len() {
        0 => None,
        _ => {
            stack.reverse();
            Some(stack.iter().fold(0, |mut acc, &cur| {
                acc *= 5;
                acc += match cur {
                    Parenthesis => 1,
                    CurlyBracket => 3,
                    SquareBracket => 2,
                    Chevron => 4,
                };

                acc
            }))
        }
    }
}

pub(crate) fn second_part(data: &Vec<Vec<Sign>>) -> Result<usize, AdventOfCodeError> {
    let mut r = data.iter()
        .filter_map(line_2)
        .collect::<Vec<_>>();
    r.sort_unstable();

    Ok(r[r.len() / 2])
}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {
    let data: Vec<Vec<Sign>> = vec![
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
    assert_eq!(288957, second_part(&data)?);
    Ok(())
}
