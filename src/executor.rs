use crate::{
    cmp::{CompareExpr, CompareExprErr},
    conditions::Condition,
    echo::Echo,
    lang_parser::Expression,
    regex::RE_VAR_EXPANSION,
    variables::{VarValue, Variable},
};
use regex::Regex;
use std::{collections::HashMap, env};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExeError {
    #[error("Compare Expr Error: {0}")]
    CompareExprErr(#[from] CompareExprErr),
}

pub struct Executor<'a> {
    vars: HashMap<&'a str, VarValue>,
    expressions: &'a Vec<Expression>,
    args: Vec<String>,
}

impl<'a> Executor<'a> {
    pub fn new(expressions: &'a Vec<Expression>) -> Self {
        let mut vars = HashMap::new();
        expressions.iter().for_each(|e| {
            if let Expression::Var(var) = e {
                vars.insert(var.name.as_str(), var.value.clone());
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
            } else if let Expression::Var(Variable { name, value }) = expr {
                let result = self.eval_var_expansion(&value.to_string());
                self.vars.insert(&name, result);
            }
        }

        Ok(())
    }

    fn eval_echo(&self, s: &str) {
        let res = self.eval_var_expansion(s);
        println!("{}", res);
    }

    fn eval_var_expansion(&self, s: &str) -> VarValue {
        let re = Regex::new(RE_VAR_EXPANSION).unwrap();
        let mut replaced_str = s.to_string();
        re.captures_iter(s).for_each(|caps| {
            let var = &caps["var"];
            let var_value = self
                .get_var_value(var, true)
                .unwrap_or(VarValue::Str("".into()));

            replaced_str = replaced_str.replace(&caps[0], &var_value.to_string());
        });

        self.get_var_value(&replaced_str, false)
            .unwrap_or(VarValue::Str(replaced_str))
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
                    self.vars.insert(&var.name, var.value.clone());
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
        if let Some(v) = self.get_var_value(s, false) {
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

    fn get_var_value(&self, s: &'a str, is_expansion: bool) -> Option<VarValue> {
        match self.vars.get(s) {
            Some(var) => Some((*var).clone()),
            None => {
                if !s.starts_with("$") && is_expansion == false {
                    return None;
                }
                let mut chars = s.chars();
                chars.next();
                let mut var = chars.as_str();

                if is_expansion == true {
                    var = s;
                }

                let value = match var.parse::<usize>() {
                    Ok(idx) => {
                        VarValue::Str(self.args.get(idx + 1).unwrap_or(&"".to_string()).clone())
                    }
                    Err(_) => VarValue::Str(env::var(var).unwrap_or_default()),
                };

                Some(value)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lang_parser::LangParser, variables::VarValue};

    use super::Executor;

    #[test]
    fn eval_var_expansion_with_curly_braces_syntax() {
        let expr = "name: str=Jone\necho \"Hello, ${name}\"";
        let parse_result = expr.parse::<LangParser>().unwrap();
        let exe = Executor::new(&parse_result.experssions);

        let value = exe.eval_var_expansion("Hello, ${name}");

        assert_eq!(value, VarValue::Str("Hello, Jone".into()));
    }

    #[test]
    fn eval_var_expansion_without_curly_braces() {
        let expr = "name: str=Jone\necho \"Hello, $name\"";
        let parse_result = expr.parse::<LangParser>().unwrap();
        let exe = Executor::new(&parse_result.experssions);

        let value = exe.eval_var_expansion("Hello, $name");

        assert_eq!(value, VarValue::Str("Hello, Jone".into()));
    }
}
