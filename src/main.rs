use std::env;

mod cli;
mod helpers;
mod simply_parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        cli::help();
    } else {
        simply_parser::parse_file(args[1].clone()).unwrap_or_else(|why| println!("{:?}", why));
    }
}
