use crate::AdventOfCodeError;

pub(crate) fn _first_part(nb_day: usize, fish: &Vec<usize>) -> Result<usize, AdventOfCodeError> {
    let mut fish = fish.to_owned();
    for _ in 0..nb_day {
        let mut born = 0;
        for v in fish.iter_mut() {
            match &v {
                0 => {
                    born += 1;
                    *v = 6
                },
                _ => *v -= 1,
            }
        }
        fish.extend(vec![8;born]);
    }

    let count = fish.len();

    Ok(count)
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {

    let data: Vec<usize> = vec![3,4,3,1,2];

    assert_eq!(26, _first_part(18, &data)?);
    assert_eq!(5934, _first_part(80, &data)?);

    Ok(())
}

pub(crate) fn second_part(nb_day: usize, fish0: &Vec<usize>) -> Result<usize, AdventOfCodeError> {
    let mut counter = [0; 9];
    for &f in fish0 {
        counter[f % 9] += 1;
    }
    for d in 0..nb_day {
        let n = counter[d % 9];
        counter[(d + 7) % 9] += n;
    }
    Ok(counter.into_iter().sum())
}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {

    let data: Vec<usize> = vec![3,4,3,1,2];

    assert_eq!(26, second_part(18, &data)?);
    assert_eq!(5934, second_part(80, &data)?);
    assert_eq!(26984457539, second_part(256, &data)?);

    Ok(())
}