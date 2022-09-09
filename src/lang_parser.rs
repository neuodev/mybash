use std::str::FromStr;
use thiserror::Error;
pub struct LangParser;

#[derive(Debug, Error)]
pub enum ParseErr {}

impl FromStr for LangParser {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LangParser)
    }
}
