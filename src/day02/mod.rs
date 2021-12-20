pub(crate) mod puzzle;
pub(crate) mod parser;

#[derive(Debug, PartialEq)]
pub enum Action {
    Forward,
    Up,
    Down,
}