use super::{AstTree, SimplyElement, SimplyLiteralElement, SimplyValue};
use base64::encode;

fn hash_literals(s: &mut String) {
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
        tmp.push_str(&s[0..literals[i] + 1]);
        tmp.push_str(encode(&s[literals[i] + 1..literals[i + 1]]).as_str());
        tmp.push_str(&s[literals[i + 1]..]);
        *s = tmp;
    }
}

fn split(code: String) -> Vec<String> {
    let mut ret = Vec::new();
    let mut tmp = String::new();

    let mut in_literal = false;

    let is_alpha = |chr: &char| (*chr >= 'a' && *chr <= 'z') || (*chr >= 'A' && *chr <= 'Z');

    for chr in code.chars() {
        if chr == '"' {
            in_literal = !in_literal;
        }

        if is_alpha(&chr) || in_literal || chr == '"' {
            tmp.push(chr);
        } else {
            ret.push(tmp);
            ret.push(format!("{}", chr));
            tmp = String::new();
        }
    }

    ret.into_iter()
        .filter(|ref x| x.trim().len() > 0)
        .collect::<Vec<String>>()
}

pub fn build_ast(mut code: String) -> AstTree {
    hash_literals(&mut code);
    let mut ast = Vec::new();

    for expr in split(code).iter() {
        ast.push(match expr.as_str() {
            "func" => SimplyElement::FuncDec(String::from("okok")),
            _ => SimplyElement::VariableDeclaration(String::from("ok")),
        });
    }

    ast
}
