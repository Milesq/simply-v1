use super::Operator::{self, *};

pub fn is_operator(fragment: &str) -> Option<Operator> {
    Some(match fragment {
        "=" => Assign,
        "==" => IsEqual(false),
        "!=" => IsEqual(true),

        ">" => Greater(false),
        ">=" => Greater(true),

        "<" => Less(false),
        "<=" => Less(true),

        "+" => Add(false),
        "+=" => Add(true),

        "-" => Subtract(false),
        "-=" => Subtract(true),

        "/" => Div(false),
        "/=" => Div(true),

        "*" => Multiply(false),
        "*=" => Multiply(true),

        "%" => Modulo(false),
        "%=" => Modulo(true),

        "!" => Negation,

        // Binary operators
        "~" => BinaryNegation,
        "&&" => LogicalAnd,
        "&" => BinaryAnd,

        "||" => LogicalOr,
        "|" => BinaryOr,

        "^" => Xor,

        _ => return None,
    })
}
