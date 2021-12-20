use crate::error::AdventOfCodeError;

#[derive(Debug)]
struct Tab {
    numbers: Vec<u8>,
    peeked: Vec<bool>,
}

impl Tab {

    fn init_from_vec(vec: Vec<u8>) -> Result<Tab, AdventOfCodeError> {
        if vec.len() != 25 {
            return Err(AdventOfCodeError::Parsing)
        }
        let tab = Tab {
            numbers: vec,
            peeked: vec![false; 25],
        };
        Ok(tab)
    }

    pub fn pick(&mut self, i: u8) {
        let index = self.numbers.iter().position(|&j| i == j);
        if let Some(index) = index {
            self.peeked[index] = true;
        }
    }

    pub fn bingo(&self) -> bool {
        let b = &self.peeked;
        let c = (0..5).into_iter()
            .flat_map(|i| b.iter().skip(i).step_by(5))
            .copied()
            .collect::<Vec<bool>>()
            .chunks(5)
            .into_iter()
            .any(|c| c.iter().all(|&b| b));

        let r = self.peeked
            .iter()
            .zip(self.numbers.iter())
            .collect::<Vec<(&bool, &u8)>>()
            .chunks(5)
            .into_iter()
            .any(|c| {
                let r = c.iter().all(|(b, _)| **b);
                r
            });

        r || c
    }

    pub fn score(&self, last: usize) -> usize {
        let sum: usize = self.numbers.iter().zip(self.peeked.iter())
            .filter_map(|(&i, b)| match b {
                false => Some(i as usize),
                true => None,
            }).sum();
        sum * last
    }

}

fn resolve_all(numbers: &[u8], grids: &[Vec<u8>]) -> Result<Vec<usize>, AdventOfCodeError> {
    let mut tabs = grids.iter()
        .map(|vec| Tab::init_from_vec(vec.clone()))
        .collect::<Result<Vec<Tab>,_>>()?;

    let mut all = Vec::new();

    for &i in numbers {

        for tab in &mut tabs {
            tab.pick(i)
        }

        for winner in tabs.iter()
            .filter(|&t| t.bingo()) {
            let s = winner.score(i as usize);
            all.push(s);
        }
        tabs = tabs.into_iter().filter(|t| !t.bingo()).collect();
    }

    Ok(all)
}

pub(crate) fn first_part(numbers: &[u8], grids: &[Vec<u8>]) -> Result<usize, AdventOfCodeError> {
    match resolve_all(numbers, grids)?.first() {
        Some(&x) => Ok(x),
        _ => Err(AdventOfCodeError::Unknown)
    }
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {
    let numbers: Vec<u8> = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1,];

    let one = vec![
        22, 13, 17, 11, 0,
        8, 2, 23, 4, 24,
        21, 9, 14, 16, 7,
        6, 10, 3, 18, 5,
        1, 12, 20, 15, 19,
    ];

    let two = vec![
        3, 15, 0, 2, 22,
        9, 18, 13, 17, 5,
        19, 8, 7, 25, 23,
        20, 11, 10, 24, 4,
        14, 21, 16, 12, 6,
    ];
    let three = vec![
        14, 21, 17, 24,  4,
        10, 16, 15,  9, 19,
        18,  8, 23, 26, 20,
        22, 11, 13, 6,  5,
        2,  0, 12 , 3 , 7,
    ];

    let result = first_part(&numbers, &vec![one, two, three])?;

    assert_eq!(4512, result);

    Ok(())
}

pub(crate) fn second_part(numbers: &[u8], grids: &[Vec<u8>]) -> Result<usize, AdventOfCodeError> {
    match resolve_all(numbers, grids)?.last() {
        Some(&x) => Ok(x),
        _ => Err(AdventOfCodeError::Unknown)
    }}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {
    let numbers: Vec<u8> = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1,];

    let one = vec![
        22, 13, 17, 11, 0,
        8, 2, 23, 4, 24,
        21, 9, 14, 16, 7,
        6, 10, 3, 18, 5,
        1, 12, 20, 15, 19,
    ];

    let two = vec![
        3, 15, 0, 2, 22,
        9, 18, 13, 17, 5,
        19, 8, 7, 25, 23,
        20, 11, 10, 24, 4,
        14, 21, 16, 12, 6,
    ];
    let three = vec![
        14, 21, 17, 24,  4,
        10, 16, 15,  9, 19,
        18,  8, 23, 26, 20,
        22, 11, 13, 6,  5,
        2,  0, 12 , 3 , 7,
    ];

    let result = second_part(&numbers, &vec![one, two, three])?;

    assert_eq!(1924, result);

    Ok(())
}
