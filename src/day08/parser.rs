use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use crate::AdventOfCodeError;

fn line(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, (a,_,b)) = tuple((separated_list1(tag(" "), alpha1), tag(" | "), separated_list1(tag(" "), alpha1)))(input)?;
    Ok((input, (a, b)))
}

pub fn parse(input: &str) -> Result<Vec<(Vec<&str>, Vec<&str>)>, AdventOfCodeError> {
    let (_, data) = separated_list1(line_ending, line)(input)
        .map_err(|_| AdventOfCodeError::Parsing)?;
    Ok(data)
}

#[test]
fn test_parser() -> Result<(), AdventOfCodeError> {
    let input = include_str!("test.txt");
    let input: String = input.to_string();

    let expected = vec![
        (vec!["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb"], vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]),
        (vec!["edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde", "gfcbed", "gfec"], vec!["fcgedb", "cgb", "dgebacf", "gc"]),
        (vec!["fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb", "cdgabef"], vec!["cg", "cg", "fdcagb", "cbg"]),
        (vec!["fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab", "fgdeca", "fcdbega"], vec!["efabcd", "cedba", "gadfec", "cb"]),
        (vec!["aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab", "fcbdga"], vec!["gecf", "egdcabf", "bgf", "bfgea"]),
        (vec!["fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg", "bafgc", "acf"], vec!["gebdcfa", "ecba", "ca", "fadegcb"]),
        (vec!["dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb", "gdcebf", "gf"], vec!["cefg", "dcbef", "fcge", "gbcadfe"]),
        (vec!["bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced", "adcbefg", "gebcd"], vec!["ed", "bcgafe", "cdgba", "cbgef"]),
        (vec!["egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb", "bfceg"], vec!["gbdfcae", "bgc", "cg", "cgb"]),
        (vec!["gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac", "fegbdc"], vec!["fgae", "cfgab", "fg", "bagce"]),
    ];

    let result = parse(&input)?;
    assert_eq!(expected, result);
    Ok(())
}