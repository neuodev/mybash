use std::str::FromStr;
use thiserror::Error;

use crate::{
    conditions::{Condition, ConditionErr},
    echo::{Echo, EchoErr},
    variables::{VarErr, Variable},
};

#[derive(Debug)]
pub struct LangParser {
    pub experssions: Vec<Expression>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseErr {
    #[error("Invalid variable: `{0}`")]
    VarErr(#[from] VarErr),
    #[error("Echo Error: `{0}`")]
    EchoErr(#[from] EchoErr),
    #[error("Condtion Error: `{0}`")]
    CondtionErr(#[from] Box<ConditionErr>),
    #[error("Invlaid experssion: {0}")]
    InvalidExperssion(String),
}

impl FromStr for LangParser {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .trim()
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>();
        let mut idx = 0;
        let mut experssions = Vec::new();
        while idx < lines.len() {
            let line = lines[idx];
            if Variable::is_var(line) {
                experssions.push(Expression::Var(line.parse::<Variable>()?))
            } else if Echo::is_echo(line) {
                experssions.push(Expression::Echo(line.parse::<Echo>()?))
            } else if Condition::is_if_statment(line) {
                let (expr, curr_idx) =
                    Condition::from_lines(&lines, idx).map_err(|e| Box::new(e))?;
                idx = curr_idx;
                experssions.push(Expression::Condition(Box::new(
                    expr.parse::<Condition>().map_err(|e| Box::new(e))?,
                )));
            } else {
                return Err(ParseErr::InvalidExperssion(line.into()));
            }

            idx += 1;
        }

        Ok(Self { experssions })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Var(Variable),
    Echo(Echo),
    Condition(Box<Condition>),
}

#[cfg(test)]
mod test {
    use crate::{
        cmp::CompareExpr,
        conditions::Condition,
        echo::Echo,
        lang_parser::Expression,
        variables::{VarValue, Variable},
    };

    use super::LangParser;

    #[test]
    fn declare_var_and_echo_it() {
        let expr = "name: string = 'Jone'\necho name";
        let result = expr.parse::<LangParser>().unwrap();
        let LangParser { experssions } = result;

        assert_eq!(experssions.len(), 2);
        assert_eq!(
            experssions[0],
            Expression::Var(Variable::new("name", VarValue::Str("Jone".into())))
        );
        assert_eq!(experssions[1], Expression::Echo(Echo("name".into())));
    }

    #[test]
    fn parse_if_statment() {
        let expr = include_str!("../lang/script_1.mb");
        let result = expr.parse::<LangParser>().unwrap();
        let LangParser { experssions } = result;

        assert_eq!(experssions.len(), 3);
        assert_eq!(
            experssions[0],
            Expression::Var(Variable::new("age", VarValue::Int(30)))
        );
        assert_eq!(experssions[1], Expression::Echo(Echo("age".into())));
        assert_eq!(
            experssions[2],
            Expression::Condition(Box::new(Condition {
                condition: CompareExpr {
                    left: "age".into(),
                    right: "40".into(),
                    operator: crate::cmp::Operator::Gt
                },
                if_expr: Expression::Echo(Echo("I am old".into())),
                else_expr: Some(Expression::Echo(Echo("I am still young".into())))
            }))
        );
    }
}
