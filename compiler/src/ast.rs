#[derive(Debug)]
#[allow(dead_code)]
pub enum AstNode {
    Number(i64),
    Align(i64, Box<AstNode>),
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
    StringLiteral(String),
    ArrayIndex(Box<AstNode>, Box<AstNode>),
    ArrayAssignment(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    InlineAsm {
        template: String,
        outputs: Vec<(String, String)>, // (constraint, expression)
        inputs: Vec<(String, String)>,  // (constraint, expression)
        clobbers: Vec<String>,
    },
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    And,
    Or,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}
