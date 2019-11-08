use super::AstTree;
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

pub fn build_ast(mut raw: String) -> AstTree {
    hash_literals(&mut raw);
    println!("{}", raw);

    vec![]
}
