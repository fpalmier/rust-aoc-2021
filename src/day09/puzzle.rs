use std::collections::HashMap;
use crate::AdventOfCodeError;

fn get(x: isize, y:isize, data: &Vec<Vec<u8>>) -> Option<&u8> {
    match (x, y) {
        (x, y) if x < 0 || y < 0 => None,
        (x, y) => data.get(x as usize).and_then(|r| r.get(y as usize)),
    }
}

pub(crate) fn first_part(data: &Vec<Vec<u8>>) -> Result<usize, AdventOfCodeError> {
    let height = data.len() as isize;
    let width = data.first().unwrap().len() as isize;

    let mut risk = 0;
    for i in 0..height {
        for j in 0..width {
            let h = get(i, j, data).unwrap();
            let next = vec![(i + 1, j), (i - 1 ,j), (i, j + 1), (i, j - 1)];
            let under = next.into_iter()
                .filter_map(|(x, y)| get(x, y, data))
                .all(|x| x > h);
            match under {
                true => {
                    risk += *h as usize + 1
                },
                false => {}
            };
        }
    }

    Ok(risk)
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {
    let data: Vec<Vec<u8>> = vec![
        vec![2,1,9,9,9,4,3,2,1,0],
        vec![3,9,8,7,8,9,4,9,2,1],
        vec![9,8,5,6,7,8,9,8,9,2],
        vec![8,7,6,7,8,9,6,7,8,9],
        vec![9,8,9,9,9,6,5,6,7,8],
    ];
    assert_eq!(15, first_part(&data)?);
    Ok(())
}

fn explore(data: &Vec<Vec<u8>>, map: &mut HashMap<(isize, isize), bool>, (i, j): (isize, isize)) -> usize {
    match get(i, j, data) {
        Some(9) => return 0,
        None => return 0,
        _ => {}
    };
    if map.contains_key(&(i, j)) { return 0 }
    map.insert((i, j), true);

    let mut score = 1;
    score += explore(data, map, (i - 1, j));
    score += explore(data, map, (i + 1, j));
    score += explore(data, map, (i, j - 1));
    score += explore(data, map, (i, j + 1));

    score
}

pub(crate) fn second_part(data: &Vec<Vec<u8>>) -> Result<usize, AdventOfCodeError> {
    let mut map: HashMap<(isize, isize), bool> = HashMap::new();
    let mut score: Vec<usize> = vec![];
    for (i, vec) in data.iter().enumerate() {
        let i = i as isize;
        for (j, _) in vec.iter().enumerate() {
            let j = j as isize;
            let exp = explore(data, &mut map, (i, j));
            if exp > 0 {
                score.push(exp);
            }
        }
    }
    score.sort_by(|a, b| b.cmp(a));
    return Ok(score.iter().take(3).product())
}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {
    let data: Vec<Vec<u8>> = vec![
        vec![2,1,9,9,9,4,3,2,1,0],
        vec![3,9,8,7,8,9,4,9,2,1],
        vec![9,8,5,6,7,8,9,8,9,2],
        vec![8,7,6,7,8,9,6,7,8,9],
        vec![9,8,9,9,9,6,5,6,7,8],
    ];
    assert_eq!(1134, second_part(&data)?);
    Ok(())
}
