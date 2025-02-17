use crate::ast::{AstNode, BinaryOperator};
use crate::lexer::{Lexer, Result, Token, TokenType};
use std::collections::HashSet;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    declared_functions: HashSet<String>,
    defined_functions: HashSet<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Result<Self> {
        let mut parser = Parser {
            lexer,
            current_token: Token {
                token_type: TokenType::EOF,
                line: 0,
                column: 0,
            },
            declared_functions: HashSet::new(),
            defined_functions: HashSet::new(),
        };
        parser.current_token = parser.lexer.next_token()?;
        Ok(parser)
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
            let statement = self.parse_statement()?;
            statements.push(statement);
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

    fn parse_statement(&mut self) -> Result<AstNode> {
        match &self.current_token.token_type {
            TokenType::Function => self.parse_function_declaration(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::LBrace => self.parse_block(),
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
            TokenType::PrintLn => {
                self.eat(TokenType::PrintLn)?;
                self.eat(TokenType::LParen)?;
                let expr = self.parse_expression()?;
                self.eat(TokenType::RParen)?;
                Ok(AstNode::PrintLn(Box::new(expr)))
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