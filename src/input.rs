use crate::error::AdventOfCodeError;

const SESSION: &str = env!("SESSION");

pub fn fetch_input(day: u8) -> Result<String, AdventOfCodeError> {
    let client = reqwest::blocking::Client::new();

    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let cookie = format!("session={}", SESSION);

    let body = client.get(url)
        .header(reqwest::header::COOKIE, cookie)
        .send()?
        .text()?;

    Ok(body)
}