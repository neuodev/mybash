use crate::{
    lang_parser::{Expression, LangParser, ParseErr},
    regex::RE_IF_ELSE,
};
use regex::Regex;
use std::str::FromStr;
use thiserror::Error;

/// A representation of `if else` statments
///
/// Example
/// ```
/// if <condition>
/// do <expr1>
/// else
/// do <expr2>
/// endif
/// ```
/// Or
/// ```
/// if <condition>
/// do <expr>
/// endif
/// ```
pub struct Condition {
    condition: String,
    if_expr: Expression,
    else_expr: Option<Expression>,
}

#[derive(Debug, Error)]
pub enum ConditionErr {
    #[error("`0` is not a valid if else statment")]
    InvalidIfElse(String),
    #[error("Parsing error: `{0}`")]
    ParseErr(#[from] ParseErr),
    #[error("Invalid experssion: {0}")]
    InvalidExperssion(String),
}

impl FromStr for Condition {
    type Err = ConditionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(RE_IF_ELSE).unwrap();
        if let Some(caps) = re.captures(s) {
            let condition = caps["con"].to_string();

            let raw_if_expr = caps["if_expr"].to_string();
            let if_expr = raw_if_expr.parse::<LangParser>()?.experssions;
            if if_expr.len() != 1 {
                return Err(ConditionErr::InvalidExperssion(raw_if_expr));
            }

            let mut else_expr = None;
            if caps.name("else_expr").is_some() {
                let raw_expr = caps["else_expr"].to_string();
                let expr = raw_expr.parse::<LangParser>()?.experssions;
                if expr.len() != 1 {
                    return Err(ConditionErr::InvalidExperssion(raw_expr));
                }

                else_expr = Some(expr[0].clone())
            }

            Ok(Self {
                condition,
                if_expr: if_expr[0].clone(),
                else_expr,
            })
        } else {
            Err(ConditionErr::InvalidIfElse(s.to_string()))
        }
    }
}
