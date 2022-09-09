use crate::regex::RE_ECHO;
use regex::Regex;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EchoErr {
    #[error("`{0}` doesn't match echo epxer")]
    NoMatch(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Echo(pub String);

impl Echo {
    pub fn is_echo(s: &str) -> bool {
        let re = Regex::new(RE_ECHO).unwrap();
        re.is_match(s)
    }
}

impl FromStr for Echo {
    type Err = EchoErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(RE_ECHO).unwrap();

        if let Some(caps) = re.captures(s) {
            Ok(Self(caps["expr"].to_string()))
        } else {
            Err(EchoErr::NoMatch(s.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::echo::EchoErr;

    use super::Echo;

    #[test]
    fn should_create_echo() {
        let expr = "echo 'with single quotes'";
        assert_eq!(
            expr.parse::<Echo>().unwrap(),
            Echo("with single quotes".into())
        );
        let expr = "echo \"with double quotes\"";
        assert_eq!(
            expr.parse::<Echo>().unwrap(),
            Echo("with double quotes".into())
        );
        let expr = "echo     $1";
        assert_eq!(expr.parse::<Echo>().unwrap(), Echo("$1".into()));
        let expr = "echo               some_var";
        assert_eq!(expr.parse::<Echo>().unwrap(), Echo("some_var".into()));
    }

    #[test]
    fn should_get_invalid_echo_expr() {
        let expr = "invalid echo";
        assert_eq!(
            expr.parse::<Echo>().err().unwrap(),
            EchoErr::NoMatch(expr.into())
        );
    }
}
