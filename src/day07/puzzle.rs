use std::cmp::Ordering;
use crate::AdventOfCodeError;

pub(crate) fn first_part(cost: fn(usize) -> usize, data: &[usize]) -> Result<usize, AdventOfCodeError> {
    let min = data.iter().min().unwrap_or(&0);
    let max = data.iter().max().unwrap_or(&0);

    let r = (*min..*max)
        .map(|x| data.iter().map(|&i| match i.cmp(&x) {
            Ordering::Less => cost(x - i),
            Ordering::Equal => 0,
            Ordering::Greater => cost(i - x),
        }).sum()).min().unwrap_or(0);

    Ok(r)
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {
    let data: Vec<usize> = vec![16,1,2,0,4,2,7,1,2,14];
    assert_eq!(37, first_part(|x| x, &data)?);
    Ok(())
}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {
    let data: Vec<usize> = vec![16,1,2,0,4,2,7,1,2,14];
    assert_eq!(168, first_part(|n| n * (n+1) / 2, &data)?);
    Ok(())
}
