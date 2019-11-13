use super::{
    AstTree, SimplyElement as El, SimplyElements, SimplyLiteralElement as Literal,
    SimplyValue as Val,
};
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

    let mut action: Option<fn(&String, &SimplyElements) -> (El, bool)> = None;
    let mut buffer: SimplyElements = Vec::new();

    for expr in split(code).iter() {
        if let Some(func) = action {
            let (ast_element, ended) = func(expr, &buffer);

            if !ended {
                buffer.push(ast_element);
            } else {
                for el in buffer {
                    ast.push(el);
                }

                ast.push(ast_element);
                action = None;
                buffer = Vec::new();
            }
        } else {
            ast.push(match expr.as_str() {
                "func" => {
                    action = Some(|func_name, _| (El::FuncDec(func_name.to_string()), true));
                    continue;
                }
                "let" => {
                    action =
                        Some(|var_name, _| (El::VariableDeclaration(var_name.to_string()), true));
                    continue;
                }
                "if" => {
                    action = Some(|condition, _| (El::IfStatement(condition.to_string()), true));
                    continue;
                }
                "{" => El::OpeningBracket,
                "}" => El::ClosingBracket,
                "(" => {
                    action = Some(|val, prevs| {
                        if val == ")" {
                            let filtered = prevs
                                .iter()
                                .filter(|el| {
                                    if let El::Identifier(Val::Variable(name)) = el {
                                        name != ","
                                    } else {
                                        true
                                    }
                                })
                                .collect::<Vec<&El>>();
                            (El::FuncInvocation(filtered), true)
                        } else {
                            (El::Identifier(Val::Variable(val.into())), val == ")")
                        }
                    });

                    continue;
                }
                _ => El::Identifier(Val::Variable(expr.into())),
            });
        }
    }

    // println!("{:#?}", ast);

    ast
}
