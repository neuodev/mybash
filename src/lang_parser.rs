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
        let lines = s.trim().lines().collect::<Vec<_>>();
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
                let cond = expr.parse::<Condition>().map_err(|e| Box::new(e))?;
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
        echo::Echo,
        lang_parser::Expression,
        variables::{VarValue, Variable},
    };

    use super::LangParser;

    #[test]
    fn declare_var_and_echo_it() {
        let expr = "name: String = 'Jone'\necho name";
        let result = expr.parse::<LangParser>().unwrap();
        let LangParser { experssions } = result;

        assert_eq!(experssions.len(), 2);
        assert_eq!(
            experssions[0],
            Expression::Var(Variable::new("name", VarValue::Str("Jone".into())))
        );
        assert_eq!(experssions[1], Expression::Echo(Echo("name".into())));
    }
}
