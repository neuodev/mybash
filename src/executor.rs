use crate::{
    cmp::{CompareExpr, CompareExprErr},
    conditions::Condition,
    echo::Echo,
    lang_parser::Expression,
    variables::{VarValue, Variable},
};
use std::{collections::HashMap, env};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExeError {
    #[error("Compare Expr Error: {0}")]
    CompareExprErr(#[from] CompareExprErr),
}

pub struct Executor<'a> {
    vars: HashMap<&'a str, &'a VarValue>,
    expressions: &'a Vec<Expression>,
    args: Vec<String>,
}

impl<'a> Executor<'a> {
    pub fn new(expressions: &'a Vec<Expression>) -> Self {
        let mut vars = HashMap::new();
        expressions.iter().for_each(|e| {
            if let Expression::Var(var) = e {
                vars.insert(var.name.as_str(), &var.value);
            }
        });
        let args = env::args().collect::<Vec<_>>();
        Self {
            vars,
            expressions,
            args,
        }
    }

    pub fn execute(&mut self) -> Result<(), ExeError> {
        for expr in self.expressions {
            if let Expression::Echo(Echo(s)) = expr {
                self.eval_echo(s);
            } else if let Expression::Condition(con) = expr {
                self.eval_condition(con.as_ref())?;
            }
        }

        Ok(())
    }

    fn eval_echo(&self, s: &str) {
        match self.vars.get(&s) {
            Some(val) => println!("{}", val),
            None => println!("{}", s),
        };
    }

    fn eval_condition(&mut self, con: &'a Condition) -> Result<(), ExeError> {
        let Condition {
            if_expr,
            else_expr,
            condition,
        } = con;
        let CompareExpr {
            left,
            right,
            operator,
        } = condition;

        let left_val = self.found_var_or_create(left);
        let right_val = self.found_var_or_create(right);
        let is_true = CompareExpr::cmp(&left_val, &right_val, operator)?;
        let expr = match (is_true, else_expr.is_some()) {
            (false, false) => None,
            (false, true) => match else_expr {
                Some(expr) => Some(expr),
                None => None,
            },
            (true, _) => Some(if_expr),
        };

        if let Some(expr) = expr {
            match expr {
                Expression::Var(var) => {
                    self.vars.insert(&var.name, &var.value);
                }
                Expression::Echo(Echo(s)) => {
                    self.eval_echo(s);
                }
                Expression::Condition(_) => todo!(),
            };
        }

        Ok(())
    }

    fn found_var_or_create(&self, s: &str) -> VarValue {
        if let Some(v) = self.get_var_value(s) {
            return v;
        }

        let var = match s.parse::<i32>() {
            Ok(num) => VarValue::Int(num),
            Err(_) => match s.parse::<bool>() {
                Ok(b) => VarValue::Bool(b),
                Err(_) => VarValue::Str(s.to_string()),
            },
        };

        var
    }

    fn get_var_value(&self, s: &'a str) -> Option<VarValue> {
        match self.vars.get(s) {
            Some(var) => Some((*var).clone()),
            None => {
                if !s.starts_with("$") {
                    return None;
                }
                let mut chars = s.chars();
                chars.next();
                let var = chars.as_str();

                let value = match var.parse::<usize>() {
                    Ok(idx) => VarValue::Str(self.args.get(idx).unwrap_or(&"".to_string()).clone()),
                    Err(_) => VarValue::Str(env::var(var).unwrap_or_default()),
                };

                Some(value)
            }
        }
    }
}
