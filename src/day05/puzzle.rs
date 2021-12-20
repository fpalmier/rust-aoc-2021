use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::ops::Range;
use crate::AdventOfCodeError;

struct Map {
    points: HashMap<(usize, usize), usize>,
}

fn magic_range(a: usize, b:usize) -> Range<usize> {
    match b.cmp(&a) {
        Ordering::Less => b..a+1,
        _ => a..b+1,
    }
}

#[test]
fn test_magic_range() {
    assert_eq!(4..9, magic_range(4, 8));
    assert_eq!(4..9, magic_range(8, 4));
}

impl Map {
    fn new() -> Map {
        Map {
            points: HashMap::new(),
        }
    }
    fn draw(&mut self, line: &((usize, usize),(usize, usize))) {
        let points = match line {
            ((x1, y1), (x2, y2)) if x1 == x2 && y1 == y2 => {
                vec![(*x1, *y1)]
            }
            ((x1, y1), (x2, y2)) if x1 == x2 => {
                magic_range(*y1, *y2).map(|y| (*x1, y)).collect()
            }
            ((x1, y1), (x2, y2)) if y1 == y2 => {
                magic_range(*x1, *x2).map(|x| (x, *y1)).collect()
            }
            _ => {
                vec![]
            }
        };
        for p in points {
            match self.points.get(&p) {
                Some(&x) => self.points.insert(p, x+1),
                None => self.points.insert(p, 1),
            };
        }
    }
    fn draw_v2(&mut self, line: &((usize, usize),(usize, usize))) {
        let points = match line {
            ((x1, y1), (x2, y2)) if x1 == x2 && y1 == y2 => {
                vec![(*x1, *y1)]
            }
            ((x1, y1), (x2, y2)) if x1 == x2 => {
                magic_range(*y1, *y2).map(|y| (*x1, y)).collect()
            }
            ((x1, y1), (x2, y2)) if y1 == y2 => {
                magic_range(*x1, *x2).map(|x| (x, *y1)).collect()
            }
            ((x1, y1), (x2, y2)) if max(x1 ,x2) - min(x1, x2) == max(y1 ,y2) - min(y1, y2) => {
                match (x1.cmp(x2), y1.cmp(y2)) {
                    (a, b) if a == b => magic_range(*x1, *x2)
                        .zip(magic_range(*y1, *y2))
                        .map(|(x, y)| (x, y))
                        .collect(),
                    _ => magic_range(*x1, *x2)
                        .rev()
                        .zip(magic_range(*y1, *y2))
                        .map(|(x, y)| (x, y))
                        .collect()
                }
            }
            _ => {
                vec![]
            }
        };
        for p in points {
            match self.points.get(&p) {
                Some(&x) => self.points.insert(p, x+1),
                None => self.points.insert(p, 1),
            };
        }
    }
    fn count(&self) -> usize {
        self.points.values().filter_map(|&x| match x.cmp(&1) {
            Ordering::Greater => Some(x),
            _ => None,
        }).count()
    }
}

pub(crate) fn first_part(lines: &[((usize, usize), (usize, usize))]) -> Result<usize, AdventOfCodeError> {
    let mut map = Map::new();
    for line  in lines {
        map.draw(line);
    }
    Ok(map.count())
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {
    let data: Vec<((usize, usize),(usize, usize))> = vec![
        ((0,9),(5,9)),
        ((8,0),(0,8)),
        ((9,4),(3,4)),
        ((2,2),(2,1)),
        ((7,0),(7,4)),
        ((6,4),(2,0)),
        ((0,9),(2,9)),
        ((3,4),(1,4)),
        ((0,0),(8,8)),
        ((5,5),(8,2)),
    ];
    assert_eq!(5, first_part(&data)?);
    Ok(())
}

pub(crate) fn second_part(lines: &[((usize, usize), (usize, usize))]) -> Result<usize, AdventOfCodeError> {
    let mut map = Map::new();
    for line  in lines {
        map.draw_v2(line);
    }
    Ok(map.count())
}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {
    let data: Vec<((usize, usize),(usize, usize))> = vec![
        ((0,9),(5,9)),
        ((8,0),(0,8)),
        ((9,4),(3,4)),
        ((2,2),(2,1)),
        ((7,0),(7,4)),
        ((6,4),(2,0)),
        ((0,9),(2,9)),
        ((3,4),(1,4)),
        ((0,0),(8,8)),
        ((5,5),(8,2)),
    ];

    assert_eq!(12, second_part(&data)?);

    Ok(())
}