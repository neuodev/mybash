use regex::Regex;

use crate::regex::RE_INVALID_MATH_EXPR;

enum Op {
    Plus,  // '+'
    Mins,  // '-'
    Multi, // '*' | 'x'
    Div,   // '/'
}

fn split<'a>(expr: &'a str, by: &'a str) -> Vec<&'a str> {
    expr.split(by).map(|term| term.trim()).collect()
}

pub fn eval(expr: &str) -> Result<f64, String> {
    let re_validate = Regex::new(RE_INVALID_MATH_EXPR).unwrap();

    if re_validate.is_match(expr) {
        return Err(format!("`{}` is not a valid math expression", expr));
    }

    let re = Regex::new(r"(\(|\[)(?P<expr>[^\)\(\]\[]+)(\)|\])").unwrap();
    let mut temp_expr = expr.to_string();
    for cap in re.captures_iter(expr) {
        let res = eval(&cap["expr"]);
        temp_expr = temp_expr.to_string().replace(&cap[0], &res?.to_string());
    }

    let expr = temp_expr.as_str();
    let mut terms = split(expr, "+");
    let mut op = Op::Plus;

    if terms.len() == 1 {
        terms = split(expr, "-");
        op = Op::Mins;
    }

    if terms.len() == 1 {
        terms = split(expr, "x");
        op = Op::Multi;
    }

    if terms.len() == 1 {
        terms = split(expr, "*");
        op = Op::Multi;
    }

    if terms.len() == 1 {
        terms = split(expr, "/");
        op = Op::Div;
    }

    let mut result: f64 = 0.0;
    for (idx, term) in terms.into_iter().enumerate() {
        let term = match term.parse() {
            Ok(num) => num,
            Err(_) => eval(term)?,
        };

        if idx == 0 {
            result = term;
            continue;
        }

        match op {
            Op::Plus => result += term,
            Op::Mins => result -= term,
            Op::Multi => result *= term,
            Op::Div => result /= term,
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::eval;

    #[test]
    fn eval_simple_math_expr() {
        let res = eval("12 + 8 / 4 - 3").unwrap();
        assert_eq!(res, 12.0 + 8.0 / 4.0 - 3.0);
    }

    #[test]
    fn eval_nested_math_expr() {
        let res = eval("(12 / 2) * [30 * 3]").unwrap();
        assert_eq!(res, (12.0 / 2.0 * (30.0 * 3.0)))
    }

    #[test]
    fn eval_invalid_math_expr() {
        let res = eval("expr");
        assert!(res.is_err())
    }
}
