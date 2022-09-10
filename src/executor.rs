use crate::{conditions::Condition, echo::Echo, lang_parser::Expression, variables::VarValue};
use std::collections::HashMap;

pub struct Executor<'a> {
    vars: HashMap<&'a String, &'a VarValue>,
    expressions: &'a Vec<Expression>,
}

impl<'a> Executor<'a> {
    pub fn new(expressions: &'a Vec<Expression>) -> Self {
        let mut vars = HashMap::new();
        expressions.iter().for_each(|e| {
            if let Expression::Var(var) = e {
                vars.insert(&var.name, &var.value);
            }
        });

        Self { vars, expressions }
    }

    pub fn execute(&self) {
        for expr in self.expressions {
            if let Expression::Echo(Echo(s)) = expr {
                let value = match self.vars.get(&s) {
                    Some(val) => match val {
                        VarValue::Int(val) => val.to_string(),
                        VarValue::Str(val) => val.to_string(),
                    },
                    None => s.to_string(),
                };

                println!("{}", value);
            } else if let Expression::Condition(con) = expr {
                let Condition {
                    if_expr,
                    else_expr,
                    condition,
                } = con.as_ref();
            }
        }
    }
}
