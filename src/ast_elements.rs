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
pub enum Operator {
    IsEqual(bool), // Negation
    Greater(bool), // Or equal?
    Less(bool),    // Or equal?
    Assign,
    Add(bool),      // And assign?
    Subtract(bool), // And assign?
    Div(bool),      // And assign?
    Multiply(bool), // And assign?
    Modulo(bool),   // And assign?
    Negation,

    // Binary and logical
    BinaryNegation,
    LogicalAnd,
    BinaryAnd,
    LogicalOr,
    BinaryOr,
    Xor,
}

#[derive(Debug)]
pub enum SimplyElement {
    FuncDec(String),             // Func name
    VariableDeclaration(String), // Var name
    Identifier(SimplyValue),     // Type
    IfStatement,                 // Condition
    ReturnStatement,
    ObjectExpression,
    Operator(Operator),

    OpeningCurlyBraces,
    ClosingCurlyBraces,

    OpeningParentheses,
    ClosingParentheses,

    OpeningBracket,
    ClosingBracket,

    Comma,
    Dot,
}

pub type SimplyElements = Vec<SimplyElement>;
pub type AstTree = Vec<SimplyElement>;
