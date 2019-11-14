use std::env;

mod cli;
mod helpers;
mod simply_parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        cli::help();
    } else {
        let file_name = args[1].clone();
        simply_parser::parse_file(file_name.clone())
            .unwrap_or_else(|err| print_error(file_name.as_str(), err));
    }
}

use simply_parser::ParseErr::{self, *};

fn print_error(file_name: &str, error: ParseErr) {
    match error {
        FileNotExists(file_name) => {
            println!("File {} doesn't exists!", file_name);
        }
        MissingApost { line, r#char } => {
            println!("Missing apost on {}:{}:{}", file_name, line, r#char);
        }
    }
}
