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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Condition {
    pub condition: String,
    pub if_expr: Expression,
    pub else_expr: Option<Expression>,
}

impl Condition {
    pub fn is_if_statment(s: &str) -> bool {
        s.trim().starts_with("if ")
    }

    pub fn is_endif(s: &str) -> bool {
        s.trim() == "endif"
    }

    pub fn from_lines(
        lines: &Vec<&str>,
        start_idx: usize,
    ) -> Result<(String, usize), ConditionErr> {
        let num_of_lines = lines.len();

        if start_idx >= num_of_lines {
            return Err(ConditionErr::InvalidIdx(format!(
                "Index out of range. idx = {start_idx}. 0 <= idx < {}",
                num_of_lines
            )));
        }

        if !Condition::is_if_statment(lines[start_idx]) {
            return Err(ConditionErr::InvalidExperssion(format!(
                "Expr: {} is not a valid if statment",
                lines[start_idx]
            )));
        }

        let mut curr_idx = start_idx;
        while curr_idx < num_of_lines - 1 {
            curr_idx += 1;

            let line = lines[curr_idx];

            if Condition::is_if_statment(line) {
                return Err(ConditionErr::InvalidExperssion(format!(
                    "Found another if statment before ending the first one: {}",
                    line
                )));
            }

            if Condition::is_endif(line) {
                break;
            }
        }

        if lines[curr_idx] != "endif" {
            return Err(ConditionErr::InvalidExperssion(format!(
                "Expected `endif` but found {}",
                lines[curr_idx]
            )));
        }

        let expr_lines = &lines[start_idx..curr_idx + 1];

        Ok((expr_lines.join("\n").to_string(), curr_idx))
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
    #[error("Invalid Indexing")]
    InvalidIdx(String),
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
        lang_parser::{Expression, ParseErr},
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
            ConditionErr::ParseErr(ParseErr::InvalidExperssion(
                "echo'Hello, World'".to_string()
            ))
        )
    }

    #[test]
    fn parse_if_statments_with_line_and_idx() {
        let lines = vec!["if condtion", "do echo 'hello, world'", "endif"];
        let (expr, idx) = Condition::from_lines(&lines, 0).unwrap();

        assert_eq!(expr, lines.join("\n"));
        assert_eq!(idx, 2)
    }

    #[test]
    fn multi_line_parsing_with_if_and_else_expr() {
        let lines = vec![
            "if condtion",
            "do echo 'hello, world'",
            "else",
            "do num: Int = 32",
            "endif",
        ];
        let (expr, idx) = Condition::from_lines(&lines, 0).unwrap();

        assert_eq!(expr, lines.join("\n"));
        assert_eq!(idx, 4)
    }

    #[test]
    fn multi_line_parsing_as_part_of_other_expr() {
        let lines = vec![
            "name: Str: Jone",
            "if condtion",
            "do echo 'hello, world'",
            "else",
            "do num: Int = 32",
            "endif",
            "echo name",
        ];
        let (expr, idx) = Condition::from_lines(&lines, 1).unwrap();

        assert_eq!(expr, lines[1..6].join("\n").to_string());
        assert_eq!(idx, 5)
    }

    #[test]
    fn parse_if_without_endif() {
        let lines = vec![
            "name: Str: Jone",
            "if condtion",
            "do echo 'hello, world'",
            "else",
            "do num: Int = 32",
        ];

        let res = Condition::from_lines(&lines, 1).err().unwrap();

        assert_eq!(
            res,
            ConditionErr::InvalidExperssion(format!("Expected `endif` but found {}", lines[4]))
        );
    }
}
