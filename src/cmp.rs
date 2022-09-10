use crate::regex::RE_CMP;
use regex::Regex;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OperatorErr {
    #[error("`{0}` is not a valid operator")]
    InvalidOperator(String),
}
#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Eq,
    NotEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
}

impl FromStr for Operator {
    type Err = OperatorErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s.trim() {
            "==" => Self::Eq,
            "!=" => Self::NotEq,
            ">" => Self::Gt,
            ">=" => Self::GtEq,
            "<" => Self::Lt,
            "<=" => Self::LtEq,
            _ => return Err(OperatorErr::InvalidOperator(s.into())),
        };

        Ok(op)
    }
}

#[derive(Debug, Error)]
pub enum CompareExprErr {
    #[error("Operator error: {0}")]
    OperatorErr(#[from] OperatorErr),
    #[error("Invalid comparson: {0}")]
    InvalidComparson(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompareExpr {
    pub left: String,
    pub right: String,
    pub operator: Operator,
}

impl FromStr for CompareExpr {
    type Err = CompareExprErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(RE_CMP).unwrap();

        if let Some(caps) = re.captures(s) {
            Ok(Self {
                left: caps["left"].to_string(),
                right: caps["right"].to_string(),
                operator: caps["op"].parse::<Operator>()?,
            })
        } else {
            Err(CompareExprErr::InvalidComparson(s.into()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::{CompareExpr, Operator};

    #[test]
    fn new_operator() {
        assert_eq!("==".parse::<Operator>().unwrap(), Operator::Eq);
        assert_eq!("!=".parse::<Operator>().unwrap(), Operator::NotEq);
        assert_eq!(">".parse::<Operator>().unwrap(), Operator::Gt);
        assert_eq!(">=".parse::<Operator>().unwrap(), Operator::GtEq);
        assert_eq!("<".parse::<Operator>().unwrap(), Operator::Lt);
        assert_eq!("<=".parse::<Operator>().unwrap(), Operator::LtEq);
        assert!("..".parse::<Operator>().is_err());
    }

    #[test]
    fn parse_equality_expr() {
        let expr = "age > 20";
        let cmp = expr.parse::<CompareExpr>().unwrap();

        assert_eq!(
            cmp,
            CompareExpr {
                left: "age".into(),
                right: "20".into(),
                operator: Operator::Gt
            }
        )
    }
}
