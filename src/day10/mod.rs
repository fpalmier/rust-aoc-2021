use crate::day10::Side::*;
use crate::day10::Kind::*;

pub mod parser;
pub mod puzzle;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind {
    Parenthesis,
    CurlyBracket,
    SquareBracket,
    Chevron,
}

#[derive(Debug, PartialEq)]
pub struct Sign(Side, Kind);

impl Sign {
    fn side(&self) -> Side { self.0 }
    fn kind(&self) -> Kind { self.1 }
}

impl From<char> for Sign {
    fn from(char: char) -> Self {
        let side = match char {
            '(' | '{' | '[' | '<' => Left,
            ')' | '}' | ']' | '>' => Right,
            _ => panic!("")
        };
        let kind = match char {
            '(' | ')' => Parenthesis,
            '{' | '}' => CurlyBracket,
            '[' | ']' => SquareBracket,
            '<' | '>' => Chevron,
            _ => panic!("")
        };
        Sign(side, kind)
    }
}