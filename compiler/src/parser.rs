use crate::ast::{AstNode, BinaryOperator};
use crate::lexer::{ErrorKind, Lexer, Result, Token, TokenType};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    declared_functions: HashSet<String>,
    defined_functions: HashSet<String>,
    included_files: HashSet<PathBuf>,
    current_dir: PathBuf,
}

impl Parser {
    pub fn new(lexer: Lexer, base_path: &Path) -> Result<Self> {
        let mut parser = Parser {
            lexer,
            current_token: Token {
                token_type: TokenType::EOF,
                line: 0,
                column: 0,
            },
            declared_functions: HashSet::new(),
            defined_functions: HashSet::new(),
            included_files: HashSet::new(),
            current_dir: base_path.parent().unwrap_or(Path::new(".")).to_path_buf(),
        };
        parser.current_token = parser.lexer.next_token()?;
        Ok(parser)
    }

    fn parse_include(&mut self) -> Result<Vec<AstNode>> {
        self.eat(TokenType::Include)?;

        // Get the string literal for the file path
        let file_path = match &self.current_token.token_type {
            TokenType::StringLiteral(path) => {
                let path = path.clone();
                self.eat(TokenType::StringLiteral(path.clone()))?;
                path
            }
            _ => {
                return Err(self.lexer.create_error(crate::lexer::ErrorKind::SyntaxError(
                    "expected string literal after @include".to_string(),
                )))
            }
        };

        // Optional 'from' directive for specifying base path
        let base_path = if let TokenType::From = self.current_token.token_type {
            self.eat(TokenType::From)?;
            match &self.current_token.token_type {
                TokenType::StringLiteral(path) => {
                    let path = path.clone();
                    self.eat(TokenType::StringLiteral(path.clone()))?;
                    PathBuf::from(path)
                }
                _ => {
                    return Err(self.lexer.create_error(crate::lexer::ErrorKind::SyntaxError(
                        "expected string literal after 'from'".to_string(),
                    )))
                }
            }
        } else {
            self.current_dir.clone()
        };

        self.eat(TokenType::Semicolon)?;

        // Resolve the full path
        let full_path = base_path.join(&file_path);
        let canonical_path = match full_path.canonicalize() {
            Ok(path) => path,
            Err(e) => {
                return Err(self.lexer.create_error(crate::lexer::ErrorKind::IOError(
                    format!("failed to resolve path '{}': {}", file_path, e)
                )))
            }
        };

        // Check for circular includes
        if !self.included_files.insert(canonical_path.clone()) {
            return Err(self.lexer.create_error(crate::lexer::ErrorKind::SyntaxError(
                format!("circular include detected: {}", file_path)
            )));
        }

        // Read and parse the included file
        let source = match fs::read_to_string(&canonical_path) {
            Ok(content) => content,
            Err(e) => {
                return Err(self.lexer.create_error(crate::lexer::ErrorKind::IOError(
                    format!("couldn't read '{}': {}", file_path, e)
                )))
            }
        };

        // Create a new lexer and parser for the included file
        let included_lexer = Lexer::new(&source, canonical_path.to_string_lossy().into_owned());
        let mut included_parser = Parser {
            lexer: included_lexer,
            current_token: Token {
                token_type: TokenType::EOF,
                line: 0,
                column: 0,
            },
            declared_functions: self.declared_functions.clone(),
            defined_functions: self.defined_functions.clone(),
            included_files: self.included_files.clone(),
            current_dir: canonical_path.parent().unwrap_or(Path::new(".")).to_path_buf(),
        };
        included_parser.current_token = included_parser.lexer.next_token()?;

        // Parse the included file
        let nodes = included_parser.parse_program()?;

        // Update function tracking sets
        self.declared_functions = included_parser.declared_functions;
        self.defined_functions = included_parser.defined_functions;
        self.included_files = included_parser.included_files;

        Ok(nodes)
    }

    fn eat(&mut self, expected_type: TokenType) -> Result<()> {
        if std::mem::discriminant(&self.current_token.token_type)
            == std::mem::discriminant(&expected_type)
        {
            self.current_token = self.lexer.next_token()?;
            Ok(())
        } else {
            Err(self
                .lexer
                .create_error(crate::lexer::ErrorKind::UnexpectedToken {
                    expected: expected_type.to_string(),
                    found: self.current_token.token_type.to_string(),
                }))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<AstNode>> {
        let mut statements = Vec::new();
        while self.current_token.token_type != TokenType::EOF {
            match self.current_token.token_type {
                TokenType::Include => {
                    // Handle include directive
                    let included_nodes = self.parse_include()?;
                    statements.extend(included_nodes);
                }
                _ => {
                    let statement = self.parse_statement()?;
                    statements.push(statement);
                }
            }
        }

        // Verify all declared functions are defined
        for func_name in &self.declared_functions {
            if !self.defined_functions.contains(func_name) {
                return Err(self.lexer.create_error(crate::lexer::ErrorKind::SyntaxError(
                    format!("function '{}' declared but not defined", func_name),
                )));
            }
        }

        Ok(statements)
    }

    fn parse_inline_asm(&mut self) -> Result<AstNode> {
        self.eat(TokenType::Asm)?;

        // Skip any whitespace after 'asm'
        self.lexer.skip_whitespace_and_comments();

        // Parse template string
        let template = match &self.current_token.token_type {
            TokenType::StringLiteral(s) => {
                let s = s.clone();
                self.eat(TokenType::StringLiteral(s.clone()))?;
                s
            }
            _ => return Err(self.lexer.create_error(ErrorKind::SyntaxError(
                "expected string literal for asm template".to_string(),
            ))),
        };

        let mut outputs = Vec::new();
        let mut inputs = Vec::new();
        let mut clobbers = Vec::new();

        // Skip whitespace before checking for colon
        self.lexer.skip_whitespace_and_comments();

        // Parse constraints if present
        if self.current_token.token_type == TokenType::Colon {
            self.eat(TokenType::Colon)?;

            // Parse output operands until next colon or semicolon
            while self.current_token.token_type != TokenType::Colon
                && self.current_token.token_type != TokenType::Semicolon {

                if !outputs.is_empty() {
                    self.eat(TokenType::Comma)?;
                }

                // Parse constraint
                let constraint = match &self.current_token.token_type {
                    TokenType::StringLiteral(s) => {
                        let s = s.clone();
                        self.eat(TokenType::StringLiteral(s.clone()))?;
                        s
                    }
                    _ => return Err(self.lexer.create_error(ErrorKind::SyntaxError(
                        "expected string literal for constraint".to_string(),
                    ))),
                };

                self.eat(TokenType::LeftBracket)?;

                // Parse expression
                let expr = match &self.current_token.token_type {
                    TokenType::Identifier(name) => {
                        let name = name.clone();
                        self.eat(TokenType::Identifier(name.clone()))?;
                        name
                    }
                    _ => return Err(self.lexer.create_error(ErrorKind::SyntaxError(
                        "expected identifier for output operand".to_string(),
                    ))),
                };

                self.eat(TokenType::RightBracket)?;
                outputs.push((constraint, expr));

                // Skip whitespace before next token
                self.lexer.skip_whitespace_and_comments();
            }

            // Parse inputs if there's a second colon
            if self.current_token.token_type == TokenType::Colon {
                self.eat(TokenType::Colon)?;

                while self.current_token.token_type != TokenType::Colon
                    && self.current_token.token_type != TokenType::Semicolon {

                    if !inputs.is_empty() {
                        self.eat(TokenType::Comma)?;
                    }

                    // Parse constraint
                    let constraint = match &self.current_token.token_type {
                        TokenType::StringLiteral(s) => {
                            let s = s.clone();
                            self.eat(TokenType::StringLiteral(s.clone()))?;
                            s
                        }
                        _ => return Err(self.lexer.create_error(ErrorKind::SyntaxError(
                            "expected string literal for constraint".to_string(),
                        ))),
                    };

                    self.eat(TokenType::LeftBracket)?;

                    // Parse expression
                    let expr = match &self.current_token.token_type {
                        TokenType::Identifier(name) => {
                            let name = name.clone();
                            self.eat(TokenType::Identifier(name.clone()))?;
                            name
                        }
                        _ => return Err(self.lexer.create_error(ErrorKind::SyntaxError(
                            "expected identifier for input operand".to_string(),
                        ))),
                    };

                    self.eat(TokenType::RightBracket)?;
                    inputs.push((constraint, expr));

                    // Skip whitespace before next token
                    self.lexer.skip_whitespace_and_comments();
                }
            }

            // Parse clobbers if there's a third colon
            if self.current_token.token_type == TokenType::Colon {
                self.eat(TokenType::Colon)?;

                while self.current_token.token_type != TokenType::Semicolon {
                    if !clobbers.is_empty() {
                        self.eat(TokenType::Comma)?;
                    }

                    match &self.current_token.token_type {
                        TokenType::StringLiteral(s) => {
                            let s = s.clone();
                            self.eat(TokenType::StringLiteral(s.clone()))?;
                            clobbers.push(s);
                        }
                        _ => return Err(self.lexer.create_error(ErrorKind::SyntaxError(
                            "expected string literal for clobber".to_string(),
                        ))),
                    }

                    // Skip whitespace before next token
                    self.lexer.skip_whitespace_and_comments();
                }
            }
        }

        self.eat(TokenType::Semicolon)?;

        Ok(AstNode::InlineAsm {
            template,
            outputs,
            inputs,
            clobbers,
        })
    }

    fn parse_statement(&mut self) -> Result<AstNode> {
        match &self.current_token.token_type {
            TokenType::Function => self.parse_function_declaration(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::LBrace => self.parse_block(),
            TokenType::Asm => self.parse_inline_asm(),
            _ => {
                let expr = self.parse_expression()?;
                self.eat(TokenType::Semicolon)?;
                Ok(expr)
            }
        }
    }

    fn parse_if_statement(&mut self) -> Result<AstNode> {
        self.eat(TokenType::If)?;
        self.eat(TokenType::LParen)?;
        let condition = self.parse_expression()?;
        self.eat(TokenType::RParen)?;

        let then_branch = Box::new(self.parse_statement()?);

        let else_branch = if let TokenType::Else = self.current_token.token_type {
            self.eat(TokenType::Else)?;
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(AstNode::If(Box::new(condition), then_branch, else_branch))
    }

    fn parse_while_statement(&mut self) -> Result<AstNode> {
        self.eat(TokenType::While)?;
        self.eat(TokenType::LParen)?;
        let condition = self.parse_expression()?;
        self.eat(TokenType::RParen)?;
        let body = self.parse_statement()?;
        Ok(AstNode::While(Box::new(condition), Box::new(body)))
    }

    fn parse_block(&mut self) -> Result<AstNode> {
        self.eat(TokenType::LBrace)?;
        let mut statements = Vec::new();

        while self.current_token.token_type != TokenType::RBrace {
            statements.push(self.parse_statement()?);
        }

        self.eat(TokenType::RBrace)?;
        Ok(AstNode::Block(statements))
    }

    fn parse_expression(&mut self) -> Result<AstNode> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<AstNode> {
        let expr = self.parse_comparison()?;

        if let TokenType::Assign = self.current_token.token_type {
            if let AstNode::Variable(name) = expr {
                self.eat(TokenType::Assign)?;
                let value = self.parse_assignment()?;
                return Ok(AstNode::Assignment(name, Box::new(value)));
            } else {
                return Err(self
                    .lexer
                    .create_error(crate::lexer::ErrorKind::SyntaxError(
                        "invalid assignment target".to_string(),
                    )));
            }
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<AstNode> {
        let mut expr = self.parse_additive()?;

        loop {
            let op = match &self.current_token.token_type {
                TokenType::Equals => {
                    self.eat(TokenType::Equals)?;
                    BinaryOperator::Equals
                }
                TokenType::Less => {
                    self.eat(TokenType::Less)?;
                    BinaryOperator::Less
                }
                TokenType::Greater => {
                    self.eat(TokenType::Greater)?;
                    BinaryOperator::Greater
                }
                TokenType::LessEqual => {
                    self.eat(TokenType::LessEqual)?;
                    BinaryOperator::LessEqual
                }
                TokenType::GreaterEqual => {
                    self.eat(TokenType::GreaterEqual)?;
                    BinaryOperator::GreaterEqual
                }
                _ => break,
            };

            let right = self.parse_additive()?;
            expr = AstNode::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<AstNode> {
        let mut expr = self.parse_multiplicative()?;

        loop {
            let op = match &self.current_token.token_type {
                TokenType::Plus => {
                    self.eat(TokenType::Plus)?;
                    BinaryOperator::Add
                }
                TokenType::Minus => {
                    self.eat(TokenType::Minus)?;
                    BinaryOperator::Subtract
                }
                _ => break,
            };

            let right = self.parse_multiplicative()?;
            expr = AstNode::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<AstNode> {
        let mut expr = self.parse_primary()?;

        loop {
            let op = match &self.current_token.token_type {
                TokenType::Multiply => {
                    self.eat(TokenType::Multiply)?;
                    BinaryOperator::Multiply
                }
                TokenType::Divide => {
                    self.eat(TokenType::Divide)?;
                    BinaryOperator::Divide
                }
                _ => break,
            };

            let right = self.parse_primary()?;
            expr = AstNode::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn is_function_declared(&self, name: &str) -> bool {
        self.declared_functions.contains(name)
    }

    fn is_function_defined(&self, name: &str) -> bool {
        self.defined_functions.contains(name)
    }

    fn parse_function_declaration(&mut self) -> Result<AstNode> {
        self.eat(TokenType::Function)?;

        // Parse function name
        let name = match &self.current_token.token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.eat(TokenType::Identifier(name.clone()))?;
                name
            }
            _ => {
                return Err(self.lexer.create_error(crate::lexer::ErrorKind::SyntaxError(
                    "expected function name".to_string(),
                )))
            }
        };

        // Check if function is already defined
        if self.is_function_defined(&name) {
            return Err(self.lexer.create_error(crate::lexer::ErrorKind::SyntaxError(
                format!("function '{}' is already defined", name),
            )));
        }

        // Parse parameters
        self.eat(TokenType::LParen)?;
        let mut parameters = Vec::new();

        if let TokenType::Identifier(_) = &self.current_token.token_type {
            if let TokenType::Identifier(param) = self.current_token.token_type.clone() {
                parameters.push(param.clone());
                self.eat(TokenType::Identifier(param))?;

                while let TokenType::Comma = self.current_token.token_type {
                    self.eat(TokenType::Comma)?;
                    if let TokenType::Identifier(param) = self.current_token.token_type.clone() {
                        parameters.push(param.clone());
                        self.eat(TokenType::Identifier(param))?;
                    }
                }
            }
        }

        self.eat(TokenType::RParen)?;

        // Check if this is a predeclaration
        if self.current_token.token_type == TokenType::Semicolon {
            self.eat(TokenType::Semicolon)?;
            self.declared_functions.insert(name.clone());
            return Ok(AstNode::FunctionPredecl(name, parameters));
        }

        // Parse function body
        let body = self.parse_block()?;

        // Add to defined functions set
        self.defined_functions.insert(name.clone());
        self.declared_functions.insert(name.clone());

        Ok(AstNode::FunctionDecl(name, parameters, Box::new(body)))
    }

    fn parse_function_call(&mut self, name: String) -> Result<AstNode> {
        // Check if function is declared
        if !self.is_function_declared(&name) {
            return Err(self.lexer.create_error(crate::lexer::ErrorKind::SyntaxError(
                format!("call to undeclared function '{}'", name),
            )));
        }

        self.eat(TokenType::LParen)?;
        let mut arguments = Vec::new();

        if self.current_token.token_type != TokenType::RParen {
            arguments.push(self.parse_expression()?);

            while let TokenType::Comma = self.current_token.token_type {
                self.eat(TokenType::Comma)?;
                arguments.push(self.parse_expression()?);
            }
        }

        self.eat(TokenType::RParen)?;
        Ok(AstNode::FunctionCall(name, arguments))
    }

    fn parse_primary(&mut self) -> Result<AstNode> {
        match &self.current_token.token_type.clone() {
            TokenType::Number(n) => {
                let value = *n;
                self.eat(TokenType::Number(value))?;
                Ok(AstNode::Number(value))
            }
            TokenType::StringLiteral(s) => {
                let value = s.clone();
                self.eat(TokenType::StringLiteral(value.clone()))?;
                Ok(AstNode::StringLiteral(value))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.eat(TokenType::Identifier(name.clone()))?;

                // Check if this is a function call
                if self.current_token.token_type == TokenType::LParen {
                    self.parse_function_call(name)
                } else {
                    Ok(AstNode::Variable(name))
                }
            }
            TokenType::LParen => {
                self.eat(TokenType::LParen)?;
                let expr = self.parse_expression()?;
                self.eat(TokenType::RParen)?;
                Ok(expr)
            }
            _ => Err(self
                .lexer
                .create_error(crate::lexer::ErrorKind::SyntaxError(format!(
                    "unexpected token in expression: {}",
                    self.current_token.token_type
                )))),
        }
    }

    fn parse_return_statement(&mut self) -> Result<AstNode> {
        self.eat(TokenType::Return)?;

        let value = if self.current_token.token_type != TokenType::Semicolon {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.eat(TokenType::Semicolon)?;
        Ok(AstNode::Return(value))
    }
}
