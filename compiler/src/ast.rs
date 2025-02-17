#[derive(Debug)]
#[allow(dead_code)]
pub enum AstNode {
    Number(i64),
    Variable(String),
    BinaryOp(Box<AstNode>, BinaryOperator, Box<AstNode>),
    Assignment(String, Box<AstNode>),
    Block(Vec<AstNode>),
    If(Box<AstNode>, Box<AstNode>, Option<Box<AstNode>>),
    While(Box<AstNode>, Box<AstNode>),
    FunctionDecl(String, Vec<String>, Box<AstNode>),
    FunctionPredecl(String, Vec<String>),
    FunctionCall(String, Vec<AstNode>),
    Return(Option<Box<AstNode>>),
    PrintLn(Box<AstNode>),
    StringLiteral(String),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}
