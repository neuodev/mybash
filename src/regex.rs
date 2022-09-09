/// A regular expression to match variables declaration. [Interactive example](https://regex101.com/r/Knql0J/1)
pub const RE_VAR: &str =
    r#"(?im)(?P<name>[^:\s]+)\s*:\s*(?P<type>[^:\s]+)\s*=\s*('|")?(?P<value>[^'"\n]+)('|")?"#;
/// A regular expression to match echo statments. [Interactive example](https://regex101.com/r/b89BqY/1)
pub const RE_ECHO: &str = r#"(?im)echo\s+('|")?(?P<expr>[^'"\n]+)('|")?"#;
