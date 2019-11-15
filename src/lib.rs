use std::{fs::File, io::Read, path::Path};

mod ast;
mod ast_elements;
mod helpers;
mod parse_err;

pub use ast_elements::*;
pub use parse_err::ParseErr;

pub fn parse_file(path: String) -> Result<(), parse_err::ParseErr> {
    if !Path::new(&path).exists() {
        return Err(ParseErr::FileNotExists(path.clone()));
    }

    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();
    parse_simply(buf)?;

    Ok(())
}

pub fn parse_simply(code: String) -> Result<(), ParseErr> {
    ast::build_ast(code)?;

    Ok(())
}

pub fn print_error(file_name: &str, error: ParseErr) {
    use ParseErr::*;

    match error {
        FileNotExists(file_name) => {
            println!("File {} doesn't exists!", file_name);
        }
        MissingApost { line, r#char } => {
            println!("Missing apost on {}:{}:{}", file_name, line, r#char);
        }
        UnsupportedElement(el) => println!("Unsupported element! '{}'", el),
    }
}
