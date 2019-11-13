use std::{fs::File, io::Read, path::Path};

mod ast;
mod ast_elements;

use ast_elements::*;

pub fn parse_file<'a>(path: String) -> Result<(), &'a str> {
    if !Path::new(&path).exists() {
        return Err("File doesn't exists");
    }

    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();
    parse_simply(buf);

    Ok(())
}

pub fn parse_simply(code: String) {
    ast::build_ast(code);
}
