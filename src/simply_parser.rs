use std::{fs::File, io::Read, path::Path};

mod ast;
mod ast_elements;
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
