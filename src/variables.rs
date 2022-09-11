use crate::{eval::eval, regex::*};
use regex::Regex;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VarValue {
    Int(i32),
    Str(String),
    Bool(bool),
}

impl Display for VarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarValue::Int(val) => write!(f, "{}", val),
            VarValue::Str(val) => write!(f, "{}", val),
            VarValue::Bool(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VarErr {
    #[error("`{0}` is not a valid int")]
    InvalidInt(String),
    #[error("`{0}` is not a valid variable declaration")]
    InvlaidVarDeclaration(String),
    #[error("`{0}` is not valid datatypes\nDatatype: [Int, Str, String]")]
    InvalidDataType(String),
    #[error("`{0}` is not valid boolean")]
    InvalidBool(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variable {
    pub name: String,
    pub value: VarValue,
}

impl Variable {
    pub fn new<T: Into<String> + Display>(name: T, value: VarValue) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }

    pub fn is_var(s: &str) -> bool {
        let re = Regex::new(RE_VAR).unwrap();
        re.is_match(s)
    }

    pub fn is_int(&self) -> bool {
        if let VarValue::Int(_) = self.value {
            true
        } else {
            false
        }
    }
}

impl FromStr for Variable {
    type Err = VarErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(RE_VAR).unwrap();

        if let Some(caps) = re.captures(s) {
            let name = caps["name"].trim().to_string();
            let value = caps["value"].trim().to_string();
            let data = match &caps["type"] {
                "str" | "string" => VarValue::Str(value),
                "int" => match eval(&value) {
                    Ok(val) => VarValue::Int(val as i32),
                    Err(_) => {
                        return Err(VarErr::InvalidInt(format!(
                            "`{}` is not a valid int expression",
                            value
                        )))
                    }
                },
                "bool" => match value.parse::<bool>() {
                    Ok(val) => VarValue::Bool(val),
                    Err(_) => {
                        return Err(VarErr::InvalidBool(format!(
                            "`{}` is not a valid boolean",
                            value
                        )))
                    }
                },
                _ => return Err(VarErr::InvalidDataType(caps["type"].to_string())),
            };

            Ok(Variable { name, value: data })
        } else {
            Err(VarErr::InvlaidVarDeclaration(s.to_string()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_new_str_var() {
        let var_name = "name".to_string();
        let var_value = "Jone".to_string();
        let var = Variable::new(&var_name, VarValue::Str(var_value.clone()));

        assert_eq!(var.name, var_name);
        assert_eq!(var.value, VarValue::Str(var_value));
    }

    #[test]
    fn create_new_int_var() {
        let var_name = "age".to_string();
        let var_value = 30;
        let var = Variable::new(&var_name, VarValue::Int(var_value));

        assert_eq!(var.name, var_name);
        assert_eq!(var.value, VarValue::Int(var_value));
    }

    #[test]
    fn new_str_var_with_double_quotes() {
        let expr = "name: str = \"Jone\"";
        let var = expr.parse::<Variable>().unwrap();

        assert_eq!(
            var,
            Variable {
                name: "name".into(),
                value: VarValue::Str("Jone".into())
            }
        )
    }

    #[test]
    fn new_str_var_with_single_quotes() {
        let expr = "email: str = 'something@whatmatter.com'";
        let var = expr.parse::<Variable>().unwrap();

        assert_eq!(
            var,
            Variable {
                name: "email".into(),
                value: VarValue::Str("something@whatmatter.com".into())
            }
        )
    }

    #[test]
    fn new_int_var() {
        let expr = "age: int = 31";
        let var = expr.parse::<Variable>().unwrap();

        assert_eq!(
            var,
            Variable {
                name: "age".into(),
                value: VarValue::Int(31)
            }
        )
    }

    #[test]
    fn new_bool_var() {
        let expr = "is_married: bool = false";
        let var = expr.parse::<Variable>().unwrap();

        assert_eq!(
            var,
            Variable {
                name: "is_married".into(),
                value: VarValue::Bool(false)
            }
        )
    }
}
