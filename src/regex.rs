/// A regular expression to match variables declaration. [Interactive example](https://regex101.com/r/DUW4Rx/1)
pub const RE_VAR: &str = r#"(?m)(?P<name>[^:\s]+)\s*:\s*(?P<type>[^:\s]+)\s*=\s*(?P<value>.+)"#;
/// A regular expression to match echo statments. [Interactive example](https://regex101.com/r/b89BqY/1)
pub const RE_ECHO: &str = r#"(?m)echo\s+('|")?(?P<expr>[^'"\n]+)('|")?"#;
/// A regular expression to match `if else` statments. [Interactive example](https://regex101.com/r/cfub08/1)
pub const RE_IF_ELSE: &str = r#"(?m)if\s+(?P<con>[^\n]+)\ndo\s+(?P<if_expr>[^\n]+)(\nelse\s*\ndo\s+(?P<else_expr>[^\n]+))?\nendif"#;
/// A regular expression to evaluate comparison operations. [Interactive example](https://regex101.com/r/OjbUgO/1)
pub const RE_CMP: &str =
    r#"(?m)('|")?(?P<left>[^\s'"\n]+)('|")?\s+(?P<op>[^\s]+)\s+('|")?(?P<right>[^'"\n]+)('|")?"#;
/// A regular expression to match commments `#`. [Interactive example](https://regex101.com/r/mAhoLQ/1)
pub const RE_COMMENT: &str = r#"(?m)(^|[^\\])#(.*)"#;
/// A regular expression to spot invalid math experssions. [Interactive example](https://regex101.com/r/kuMDUi/1)
pub const RE_INVALID_MATH_EXPR: &str = r#"(?m)[a-zA-Z=]+"#;
/// A regular expression to match variable expansions `ex: echo "Hello, ${name}"`. [Interactive example](https://regex101.com/r/5BLcW2/1)
pub const RE_VAR_EXPANSION: &str = r#"(?m)\$\{?(?P<var>[^\s"';,}]+)\}?"#;
/// A regular expression to match match the `input("<text>")` function. [Interactive example](https://regex101.com/r/LPHoMk/1)
pub const RE_INPUT_FUNC: &str = r#"(?m)input\(('|")?(?P<text>[^)'"]+)('|")?\)"#;
/// A regular expression for quote removal . [Interactive example](https://regex101.com/r/oJ9u9B/1)
pub const RE_QUOTE_REMOVAL: &str = r#"(?m)('|")?(?P<value>[^'"\n]+)('|")?"#;
