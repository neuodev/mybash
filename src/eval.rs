use regex::Regex;

enum Op {
    Plus,  // '+'
    Mins,  // '-'
    Multi, // '*' | 'x'
    Div,   // '/'
}

fn split<'a>(expr: &'a str, by: &'a str) -> Vec<&'a str> {
    expr.split(by).map(|term| term.trim()).collect()
}

pub fn eval(expr: &str) -> f64 {
    let re = Regex::new(r"(\(|\[)(?P<expr>[^\)\(\]\[]+)(\)|\])").unwrap();
    let mut temp_expr = expr.to_string();
    re.captures_iter(expr).for_each(|cap| {
        let res = eval(&cap["expr"]);
        temp_expr = temp_expr.to_string().replace(&cap[0], &res.to_string());
    });

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
    terms.into_iter().enumerate().for_each(|(idx, term)| {
        let term = match term.parse() {
            Ok(num) => num,
            Err(_) => eval(term),
        };

        if idx == 0 {
            result = term;
            return;
        }

        match op {
            Op::Plus => result += term,
            Op::Mins => result -= term,
            Op::Multi => result *= term,
            Op::Div => result /= term,
        }
    });

    result
}

#[cfg(test)]
mod test {
    use super::eval;

    #[test]
    fn eval_simple_math_expr() {
        let res = eval("12 + 8 / 4 - 3");
        assert_eq!(res, 12.0 + 8.0 / 4.0 - 3.0);
    }

    #[test]
    fn eval_nested_math_expr() {
        let res = eval("(12 / 2) * [30 * 3]");
        assert_eq!(res, (12.0 / 2.0 * (30.0 * 3.0)))
    }
}
