use crate::day02::Action;
use crate::error::AdventOfCodeError;

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

impl Position {
    fn new() -> Position {
        Position{
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
    fn take_command(&mut self, (action, value): &(Action, usize)) {
        match action {
            Action::Forward => {
                self.horizontal += value;
            }
            Action::Up => {
                self.depth -= value;
            }
            Action::Down => {
                self.depth += value;
            }
        }
    }

    fn take_command_v2(&mut self, (action, value): &(Action, usize)) {
        match action {
            Action::Forward => {
                self.horizontal += value;
                self.depth += self.aim * value;
            }
            Action::Up => {
                self.aim -= value;
            }
            Action::Down => {
                self.aim += value;
            }
        }
    }

    fn value(&self) -> usize {
        self.horizontal * self.depth
    }

}

pub fn first_part(data: &[(Action, usize)]) -> Result<usize, AdventOfCodeError> {
    let mut pos = Position::new();
    for command in data {
        pos.take_command(command);
    }
    Ok(pos.value())
}

#[test]
fn test_first_part() -> Result<(), AdventOfCodeError> {
    let data = vec![
        (Action::Forward, 5),
        (Action::Down, 5),
        (Action::Forward, 8),
        (Action::Up, 3),
        (Action::Down, 8),
        (Action::Forward, 2),
    ];
    let result = first_part(&data)?;
    assert_eq!(150, result);
    Ok(())
}

pub fn second_part(data: &[(Action, usize)]) -> Result<usize, AdventOfCodeError> {
    let mut pos = Position::new();
    for command in data {
        pos.take_command_v2(command);
    }
    Ok(pos.value())}

#[test]
fn test_second_part() -> Result<(), AdventOfCodeError> {
    let data = vec![
        (Action::Forward, 5),
        (Action::Down, 5),
        (Action::Forward, 8),
        (Action::Up, 3),
        (Action::Down, 8),
        (Action::Forward, 2),
    ];
    let result = second_part(&data)?;
    assert_eq!(900, result);
    Ok(())}