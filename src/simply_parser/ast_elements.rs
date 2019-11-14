#![allow(dead_code)]

#[derive(Debug)]
pub enum SimplyLiteralElement {
    IntNumber(i32),
    FloatNumber(f32),
    StringValue(String),
}

#[derive(Debug)]
pub enum SimplyValue {
    Variable(String), // Var name
    Array(Vec<SimplyValue>),
    Literal(SimplyLiteralElement),
}

#[derive(Debug)]
pub enum SimplyElement {
    FuncDec(String),             // Func name
    VariableDeclaration(String), // Var name
    Identifier(SimplyValue),     // Type
    IfStatement,                 // Condition
    Operator(String),
    OpeningCurlyBracket,
    ClosingCurlyBracket,

    OpeningBracket,
    ClosingBracket,
}

pub type SimplyElements = Vec<SimplyElement>;
pub type AstTree = Vec<SimplyElement>;
