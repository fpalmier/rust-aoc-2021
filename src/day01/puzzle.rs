use crate::error::AdventOfCodeError;



pub fn first_part(data: &[usize]) -> Result<usize, AdventOfCodeError> {

    let count = data.windows(2)
        .into_iter()
        .filter(|&arr| arr[0] < arr[1])
        .count();

    Ok(count)
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {
    let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let result = first_part(&data)?;
    assert_eq!(7, result);
    Ok(())
}

pub fn second_part(data: &[usize]) -> Result<usize, AdventOfCodeError> {
    let count = data.windows(3)
        .collect::<Vec<_>>()
        .windows(2)
        .into_iter()
        .filter(|&arr| arr[0].iter().sum::<usize>() < arr[1].iter().sum::<usize>())
        .count();
    Ok(count)
}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {
    let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let result = second_part(&data)?;
    assert_eq!(5, result);
    Ok(())
}