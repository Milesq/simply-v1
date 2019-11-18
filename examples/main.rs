use simply_build_ast::parse_file;

fn main() {
    println!("{:#?}", parse_file("examples/main.sl").unwrap());
}
