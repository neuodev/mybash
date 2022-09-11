use crate::{
    regex::RE_CMP,
    variables::{VarValue, Variable},
};
use regex::Regex;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OperatorErr {
    #[error("`{0}` is not a valid operator")]
    InvalidOperator(String),
}
#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CompareExprErr {
    #[error("Operator error: {0}")]
    OperatorErr(#[from] OperatorErr),
    #[error("Invalid comparson: {0}")]
    InvalidComparson(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompareExpr {
    pub left: String,
    pub right: String,
    pub operator: Operator,
}

impl CompareExpr {
    pub fn cmp(left: &VarValue, right: &VarValue, op: &Operator) -> Result<bool, CompareExprErr> {
        let res = match op {
            Operator::Eq => left == right,
            Operator::NotEq => left != right,
            other => {
                let (left_val, right_val) = CompareExpr::is_valid_int_cmp(left, right)?;
                match other {
                    Operator::Gt => left_val > right_val,
                    Operator::GtEq => left_val >= right_val,
                    Operator::Lt => left_val < right_val,
                    Operator::LtEq => left_val <= right_val,
                    _ => unreachable!(),
                }
            }
        };

        Ok(res)
    }

    fn is_valid_int_cmp(left: &VarValue, right: &VarValue) -> Result<(i32, i32), CompareExprErr> {
        match (&left, &right) {
            (VarValue::Int(left_val), VarValue::Int(right_val)) => {
                Ok((left_val.clone(), right_val.clone()))
            }
            (VarValue::Int(_), VarValue::Str(v)) => Err(CompareExprErr::InvalidComparson(format!(
                "`{}` is not a valid right hand side",
                v
            ))),
            (VarValue::Str(v), VarValue::Int(_)) => Err(CompareExprErr::InvalidComparson(format!(
                "`{}` is not a valid left hand side",
                v
            ))),
            (_, _) => Err(CompareExprErr::InvalidComparson(format!(
                "`{}` & `{}` Invalid right and left hand side",
                left, right
            ))),
        }
    }
}

impl FromStr for CompareExpr {
    type Err = CompareExprErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(RE_CMP).unwrap();

        if let Some(caps) = re.captures(s) {
            Ok(Self {
                left: caps["left"].trim().to_string(),
                right: caps["right"].trim().to_string(),
                operator: caps["op"].parse::<Operator>()?,
            })
        } else {
            Err(CompareExprErr::InvalidComparson(s.into()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::{CompareExpr, CompareExprErr, Operator, OperatorErr};

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

    #[test]
    fn parse_invalid_expr() {
        let expr = "age !! 20";
        let cmp = expr.parse::<CompareExpr>();

        assert_eq!(
            cmp.err().unwrap(),
            CompareExprErr::OperatorErr(OperatorErr::InvalidOperator("!!".into()))
        )
    }
}
