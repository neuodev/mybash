use crate::regex::*;
use regex::Regex;
use std::str::FromStr;
use thiserror::Error;

pub enum VarValue {
    Int(i32),
    Str(String),
}

#[derive(Debug, Error)]
pub enum VarErr {
    #[error("`{0}` is not a valid int")]
    InvalidInt(String),
    #[error("`{0}` is not a valid variable declaration")]
    InvlaidVarDeclaration(String),
    #[error("`{0}` is not valid datatypes\nDatatype: [Int, Str, String]")]
    InvalidDataType(String),
}

pub struct Variable {
    name: String,
    value: VarValue,
}

impl FromStr for Variable {
    type Err = VarErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(RE_VAR).unwrap();

        if let Some(caps) = re.captures(s) {
            let name = caps["name"].to_string();
            let value = caps["value"].to_string();
            let data = match &caps["type"] {
                "Str" | "String" => VarValue::Str(value),
                "Int" => match value.parse::<i32>() {
                    Ok(val) => VarValue::Int(val),
                    Err(_) => {
                        return Err(VarErr::InvalidInt(format!(
                            "`{}` is not a valid int",
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
