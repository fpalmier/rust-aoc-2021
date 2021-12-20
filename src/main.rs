use crate::error::AdventOfCodeError;

mod error;
mod input;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() -> Result<(), AdventOfCodeError> {

    let input = input::fetch_input(1)?;
    let data = day01::parser::parse(&input)?;
    let one = day01::puzzle::first_part(&data)?;
    let two = day01::puzzle::second_part(&data)?;
    println!("Answer day 01 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    let input = input::fetch_input(2)?;
    let data = day02::parser::parse(&input)?;
    let one = day02::puzzle::first_part(&data)?;
    let two = day02::puzzle::second_part(&data)?;
    println!("Answer day 02 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    let input = input::fetch_input(3)?;
    let data = day03::parser::parse(&input)?;
    let one = day03::puzzle::first_part(12, &data)?;
    let two = day03::puzzle::second_part(12, &data)?;
    println!("Answer day 03 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    let input = input::fetch_input(4)?;
    let (numbers, grids) = day04::parser::parse(&input)?;
    let one = day04::puzzle::first_part(&numbers, &grids)?;
    let two = day04::puzzle::second_part(&numbers, &grids)?;
    println!("Answer day 04 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    let input = input::fetch_input(5)?;
    let data = day05::parser::parse(&input)?;
    let one = day05::puzzle::first_part(&data)?;
    let two = day05::puzzle::second_part(&data)?;
    println!("Answer day 05 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    let input = input::fetch_input(6)?;
    let data = day06::parser::parse(&input)?;
    let one = day06::puzzle::second_part(80, &data)?;
    let two = day06::puzzle::second_part(256, &data)?;
    println!("Answer day 06 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    let input = input::fetch_input(7)?;
    let data = day06::parser::parse(&input)?;
    let one = day07::puzzle::first_part(|x| x,&data)?;
    let two = day07::puzzle::first_part(|n| n * (n+1) / 2, &data)?;
    println!("Answer day 07 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    let input = input::fetch_input(8)?;
    let data = day08::parser::parse(&input)?;
    let one = day08::puzzle::first_part(&data)?;
    let two = day08::puzzle::second_part(&data)?;
    println!("Answer day 08 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    let input = input::fetch_input(9)?;
    let data = day09::parser::parse(&input)?;
    let one = day09::puzzle::first_part(&data)?;
    let two = day09::puzzle::second_part(&data)?;
    println!("Answer day 09 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    let input = input::fetch_input(10)?;
    let data = day10::parser::parse(&input)?;
    let one = day10::puzzle::first_part(&data)?;
    let two = day10::puzzle::second_part(&data)?;
    println!("Answer day 10 :");
    println!("  first part  : {}", one);
    println!("  second part : {}", two);

    Ok(())
}




