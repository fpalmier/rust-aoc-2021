
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdventOfCodeError {
    #[error("data store disconnected")]
    Network(#[from] reqwest::Error),
    #[error("can't parse")]
    Parsing,
    #[error("unknown error")]
    Unknown,
 }