use std::collections::HashSet;
use crate::AdventOfCodeError;

pub(crate) fn first_part(data: &[(Vec<&str>, Vec<&str>)]) -> Result<usize, AdventOfCodeError> {

    let r = data.iter()
        .map(|(_, v)| v.iter().filter(|&&str| matches!(str.len(), 2 | 3 | 4 | 7)).count())
        .sum();

    Ok(r)
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {
    let data = vec![
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
    assert_eq!(26, first_part(&data)?);
    Ok(())
}

fn read_line((input, output): &(Vec<&str>, Vec<&str>)) -> usize {

    let mut input: Vec<HashSet<char>> = input
        .iter()
        .map(|str| str.chars().collect())
        .collect();

    input.sort_by_key(|set| set.len());

    let mut number: [Option<&HashSet<char>>; 10] = [None; 10];

    for set in input.iter() {
        match set.len() {
            2 => number[1] = Some(set),
            4 => number[4] = Some(set),
            3 => number[7] = Some(set),
            7 => number[8] = Some(set),
            5 => {
                let index = match set {
                    set if set.intersection(number[4].unwrap()).count() == 2 => 2,
                    set if set.intersection(number[1].unwrap()).count() == 2 => 3,
                    _ => 5,
                };
                number[index] = Some(set);
            }
            6 => {
                let index = match set {
                    set if set.intersection(number[1].unwrap()).count() == 1 => 6,
                    set if set.intersection(number[4].unwrap()).count() == 4 => 9,
                    _ => 0,
                };
                number[index] = Some(set);
            }
            _ => {}
        }
    }

    output.clone().iter()
        .filter_map(|&digit| {
            let digit: HashSet<_> = digit.chars().collect();
            number.iter().enumerate()
                .filter_map(|(n, &option)| option.map(|v|(n,v)))
                .filter_map(|(n, set)| match set == &digit {
                    true => Some(n.to_string()),
                    false => None,
                }).next()
        })
        .collect::<Vec<_>>()
        .concat()
        .parse()
        .unwrap()

}

pub(crate) fn second_part(data: &Vec<(Vec<&str>, Vec<&str>)>) -> Result<usize, AdventOfCodeError> {
    let r = data.iter()
        .map(read_line)
        .sum();
    Ok(r)
}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {
    let data = vec![
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
    assert_eq!(61229, second_part(&data)?);
    Ok(())
}
