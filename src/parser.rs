use std::str::FromStr;
use thiserror::Error;
pub struct Parser;

#[derive(Debug, Error)]
pub enum ParserErr {}

impl FromStr for Parser {
    type Err = ParserErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Parser)
    }
}
