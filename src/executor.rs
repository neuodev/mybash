use crate::{
    cmp::CompareExpr,
    conditions::Condition,
    echo::Echo,
    lang_parser::Expression,
    variables::{VarValue, Variable},
};
use std::collections::HashMap;

pub struct Executor<'a> {
    vars: HashMap<&'a str, &'a VarValue>,
    expressions: &'a Vec<Expression>,
}

impl<'a> Executor<'a> {
    pub fn new(expressions: &'a Vec<Expression>) -> Self {
        let mut vars = HashMap::new();
        expressions.iter().for_each(|e| {
            if let Expression::Var(var) = e {
                vars.insert(var.name.as_str(), &var.value);
            }
        });

        Self { vars, expressions }
    }

    pub fn execute(&self) {
        for expr in self.expressions {
            if let Expression::Echo(Echo(s)) = expr {
                self.eval_echo(s);
            } else if let Expression::Condition(con) = expr {
                self.eval_condition(con.as_ref())
            }
        }
    }

    fn eval_echo(&self, s: &str) {
        match self.vars.get(&s) {
            Some(val) => println!("{}", val),
            None => println!("{}", s),
        };
    }

    fn eval_condition(&self, con: &Condition) {
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
    }

    fn found_var_or_create(&self, s: &str) -> VarValue {
        if let Some(v) = self.vars.get(s) {
            return (*v).clone();
        }

        let var = match s.parse::<i32>() {
            Ok(num) => VarValue::Int(num),
            Err(_) => VarValue::Str(s.to_string()),
        };

        var
    }
}
