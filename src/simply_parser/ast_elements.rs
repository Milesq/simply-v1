#[derive(Debug)]
pub enum SimplyLiteralElement {
    IntNumber(i32),
    FloatNumber(f32),
    StringValue(String),
}

#[derive(Debug)]
pub enum SimplyElement {
    Literal(SimplyLiteralElement),
}

pub type AstTree = Vec<SimplyElement>;
