use crate::lang_parser::Expression;

/// A representation of `if else` statments
///
/// Example
/// ```
/// if <condition>
/// do <expr1>
/// else
/// do <expr2>
/// endif
/// ```
/// Or
/// ```
/// if <condition>
/// do <expr>
/// endif
/// ```
pub struct Condition {
    if_expr: Expression,
    else_expr: Option<Expression>,
}
