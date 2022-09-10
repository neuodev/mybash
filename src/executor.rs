use crate::{echo::Echo, lang_parser::Expression, variables::VarValue};
use std::collections::HashMap;

pub struct Executor;

impl Executor {
    pub fn execute(expressions: Vec<Expression>) {
        // Create variable map
        let mut var_map = HashMap::new();

        expressions.iter().for_each(|e| {
            if let Expression::Var(var) = e {
                var_map.insert(&var.name, &var.value);
            }
        });

        for expr in &expressions {
            if let Expression::Echo(Echo(s)) = expr {
                let value = match var_map.get(&s) {
                    Some(val) => match val {
                        VarValue::Int(val) => val.to_string(),
                        VarValue::Str(val) => val.to_string(),
                    },
                    None => s.to_string(),
                };

                println!("{}", value);
            } else if let Expression::Condition(con) = expr {
            }
        }
    }
}
