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

impl Condition {
    pub fn is_if_statment(s: &str) -> bool {
        s.trim().starts_with("if ")
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
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
        if let Some(caps) = re.captures(s.trim()) {
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

#[cfg(test)]
mod test {
    use crate::{
        conditions::ConditionErr,
        echo::Echo,
        lang_parser::Expression,
        variables::{VarValue, Variable},
    };

    use super::Condition;

    #[test]
    fn parse_if_statments_only() {
        let expr = "if condition\ndo echo 'Hello, World'\nendif";
        let Condition {
            condition,
            if_expr,
            else_expr,
        } = expr.parse::<Condition>().unwrap();

        assert_eq!(condition, "condition".to_string());
        assert_eq!(if_expr, Expression::Echo(Echo("Hello, World".to_string())));
        assert!(else_expr.is_none());
    }

    #[test]
    fn parse_if_else_statments() {
        let expr = "if some_condition\ndo name: String = 'Hello, World'\nelse\ndo echo 'Hello, World'\nendif";
        let Condition {
            condition,
            if_expr,
            else_expr,
        } = expr.parse::<Condition>().unwrap();

        assert_eq!(condition, "some_condition".to_string());
        assert_eq!(
            if_expr,
            Expression::Var(Variable::new("name", VarValue::Str("Hello, World".into())))
        );
        assert_eq!(
            else_expr,
            Some(Expression::Echo(Echo("Hello, World".into())))
        );
    }

    #[test]
    fn parse_if_statments_with_invalid_expr() {
        let expr = "if condition\ndo echo'Hello, World'\nendif";
        let res = expr.parse::<Condition>();
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            ConditionErr::InvalidExperssion("echo'Hello, World'".to_string())
        )
    }
}
