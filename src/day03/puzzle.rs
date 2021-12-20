use std::cmp::Ordering;

use crate::AdventOfCodeError;

pub fn first_part(byte_size: usize, data: &[Vec<bool>]) -> Result<usize, AdventOfCodeError> {
    let acc = data.iter().fold(vec![0; byte_size as usize], |acc, cur| {
        let cur: Vec<i32> = cur.iter().map(|b| match b {
            true => 1,
            false => -1,
        }).collect();

        cur.iter()
            .zip(acc.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<i32>>()
    });

    let acc: Vec<bool> = acc.iter().map(|i| match i.cmp(&0) {
        Ordering::Less => true,
        Ordering::Greater => false,
        _ => todo!()
    }).collect();

    let gamma: usize = acc.iter().fold(0, |byte: usize, &b| match b {
        true => byte << 1 | 1,
        false => byte << 1,
    });

    let mut mask = 0;
    for _ in 0..byte_size {
        mask <<= 1;
        mask |= 1;
    }
    let epsilon = mask ^ gamma;

    Ok(gamma * epsilon)
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {
    let data = vec![
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

    assert_eq!(198, first_part(5, &data)?);

    Ok(())
}

pub fn second_part(byte_size: usize, data: &[Vec<bool>]) -> Result<usize, AdventOfCodeError> {
    let ox = compute(byte_size, true, data);
    let co2 = compute(byte_size, false, data);

    Ok(ox * co2)
}

fn compute(byte_size: usize, mode: bool, data: &[Vec<bool>]) -> usize {
    let mut data: Vec<&Vec<bool>> = data.iter().collect();
    for i in 0..byte_size {
        let sum: i32 = data.iter().map(|vec| match vec.get(i).unwrap() {
            true => 1,
            false => -1,
        }).sum();

        match sum.cmp(&0) {
            Ordering::Greater => {
                data = data.into_iter()
                    .filter(|&vec| vec.get(i).unwrap() == &mode)
                    .collect();
            }
            Ordering::Less => {
                data = data.into_iter()
                    .filter(|&vec| vec.get(i).unwrap() == &!mode)
                    .collect();
            }
            Ordering::Equal => {
                data = data.into_iter()
                    .filter(|&vec| vec.get(i).unwrap() == &mode)
                    .collect();
            }
        };
        
        if data.len() == 1 {
            return data.iter()
                .find(|&vec| vec.get(i).unwrap() == &mode)
                .unwrap()
                .iter().fold(0, |byte: usize, &b| match b {
                true => byte << 1 | 1,
                false => byte << 1,
            });
        }
    }
    panic!("")
}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {
    let data = vec![
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

    assert_eq!(230, second_part(5, &data)?);

    Ok(())
}