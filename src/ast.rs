use super::{
    AstTree, Operator, ParseErr, SimplyElement as El, SimplyLiteralElement as Literal,
    SimplyValue as Val,
};
use base64::encode;
use regex::Regex;

mod operators;
use operators::is_operator;

fn hash_literals(s: &mut String) -> Result<(), ParseErr> {
    let mut in_literal = false;
    let mut skip = false;
    let mut literals = Vec::new();

    for (chr, i) in s.chars().zip(0..) {
        if skip {
            skip = false;
            continue;
        }

        if in_literal {
            match chr {
                '\\' => {
                    skip = true;
                }
                '"' => {
                    in_literal = false;
                    literals.push(i);
                }
                _ => {}
            }
        } else if chr == '"' {
            literals.push(i);
            in_literal = true;
        }
    }

    for i in (0..literals.len()).step_by(2).rev() {
        let mut tmp = String::new();
        tmp.push_str(&s[0..=literals[i]]);

        if literals.len() == i + 1 {
            let (line, chr) = crate::helpers::get_line(s, literals[i]);
            return Err(ParseErr::MissingApost { line, r#char: chr });
        }

        tmp.push_str(encode(&s[literals[i] + 1..literals[i + 1]]).as_str());
        tmp.push_str(&s[literals[i + 1]..]);
        *s = tmp;
    }

    Ok(())
}

fn is_one_of(chr: char, chars: Vec<char>) -> bool {
    chars.iter().any(|current| current == &chr)
}

fn split(code: String) -> Vec<String> {
    let mut ret = Vec::new();
    let mut tmp = String::new();

    let mut in_literal = false;
    let mut is_one_expr = false;

    let is_alpha = |chr: &char| (*chr >= 'a' && *chr <= 'z') || (*chr >= 'A' && *chr <= 'Z');

    for chr in code.chars() {
        if chr == '"' {
            in_literal = !in_literal;
        }

        if is_one_of(chr, "+-*/%!<>=".chars().collect::<Vec<char>>()) {
            is_one_expr = true;
        }

        if is_one_expr || is_alpha(&chr) || in_literal || chr == '"' {
            tmp.push(chr);
        } else {
            ret.push(tmp);
            ret.push(format!("{}", chr));
            tmp = String::new();
        }

        if is_one_expr {
            is_one_expr = false;
        }
    }

    ret.into_iter()
        .filter(|ref x| !x.trim().is_empty())
        .collect::<Vec<String>>()
}

macro_rules! r {
    ($regexp: expr) => {
        Regex::new($regexp).unwrap()
    };
}

pub fn build_ast(mut code: String) -> Result<AstTree, ParseErr> {
    let int = r!(r"^[0-9]+$");
    let float = r!(r"^[0-9]+\.[0-9]+$");
    let ident = r!(r"[a-zA-Z]([a-zA-Z0-9]+)?");

    hash_literals(&mut code)?;
    let mut ast = Vec::new();

    type Action = fn(&String, String) -> Result<El, String>;
    let mut action: Option<Action> = None;
    let mut buffer = String::new();

    for expr in split(code).iter() {
        if let Some(func) = action {
            let ast_element = func(&expr, buffer.clone());

            match ast_element {
                Err(new_part) => buffer.push_str(new_part.as_str()),
                Ok(parsed) => {
                    ast.push(parsed);
                    action = None;
                    buffer = String::new();
                }
            }
        } else {
            ast.push(match expr.as_str() {
                "func" => {
                    action = Some(|func_name, _| Ok(El::FuncDec(func_name.to_string())));
                    continue;
                }
                "let" => {
                    action = Some(|var_name, _| Ok(El::VariableDeclaration(var_name.to_string())));
                    continue;
                }
                "if" => El::IfStatement,
                "return" => El::ReturnStatement,
                "@" => El::ObjectExpression,
                "{" => El::OpeningCurlyBraces,
                "}" => El::ClosingCurlyBraces,
                "(" => El::OpeningParentheses,
                ")" => El::ClosingParentheses,
                "[" => El::OpeningBracket,
                "]" => El::ClosingBracket,
                "," => El::Comma,
                "." => El::Dot,
                _ => {
                    if let Some(operator) = is_operator(expr) {
                        El::Operator(operator)
                    } else if int.is_match(expr) {
                        El::Identifier(Val::Literal(Literal::IntNumber(
                            expr.parse::<i32>().unwrap(),
                        )))
                    } else if float.is_match(expr) {
                        El::Identifier(Val::Literal(Literal::FloatNumber(
                            expr.parse::<f32>().unwrap(),
                        )))
                    } else if expr.get(0..1).unwrap() == "\"" {
                        El::Identifier(Val::Literal(Literal::StringValue(expr.to_string())))
                    } else if ident.is_match(expr) {
                        El::Identifier(Val::Variable(expr.to_string()))
                    } else {
                        return Err(ParseErr::UnsupportedElement(expr.to_string()));
                    }
                }
            });
        }
    }

    Ok(ast)
}
