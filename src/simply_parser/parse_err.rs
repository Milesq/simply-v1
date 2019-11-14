#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErr {
    MissingApost { line: usize, r#char: usize },
    FileNotExists(String),
}
