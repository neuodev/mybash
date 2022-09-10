/// A regular expression to match variables declaration. [Interactive example](https://regex101.com/r/Knql0J/1)
pub const RE_VAR: &str =
    r#"(?m)(?P<name>[^:\s]+)\s*:\s*(?P<type>[^:\s]+)\s*=\s*('|")?(?P<value>[^'"\n]+)('|")?"#;
/// A regular expression to match echo statments. [Interactive example](https://regex101.com/r/b89BqY/1)
pub const RE_ECHO: &str = r#"(?m)echo\s+('|")?(?P<expr>[^'"\n]+)('|")?"#;
/// A regular expression to match `if else` statments. [Interactive example](https://regex101.com/r/cfub08/1)
pub const RE_IF_ELSE: &str = r#"(?m)if\s+(?P<con>[^\n]+)\ndo\s+(?P<if_expr>[^\n]+)(\nelse\s*\ndo\s+(?P<else_expr>[^\n]+))?\nendif"#;

/// A regular expression to evaluate comparison operations. [Interactive example](https://regex101.com/r/QLgk8r/1)
pub const RE_CMP: &str = r#"(?m)(?P<left>[^\s]+)\s+(?P<op>[^\s]+)\s+(?P<right>.+)"#;
