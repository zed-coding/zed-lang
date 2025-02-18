use std::fmt;

// Error handling structures
#[derive(Debug)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: String,
}

#[derive(Debug)]
pub struct CompilerError {
    pub kind: ErrorKind,
    pub location: SourceLocation,
    pub source_line: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ErrorKind {
    UnexpectedToken { expected: String, found: String },
    UndefinedVariable(String),
    SyntaxError(String),
    InvalidOperator(String),
    IOError(String),
}

impl CompilerError {
    pub fn new(kind: ErrorKind, location: SourceLocation, source_line: String) -> Self {
        CompilerError {
            kind,
            location,
            source_line,
        }
    }

    pub fn format_error(&self) -> String {
        use crate::colors::{error_style, error_location_style, error_source_style, error_pointer_style};

        let mut error = String::new();

        // Error message
        error.push_str(&error_style().apply(&format!("error: {}\n", self.get_error_message())));

        // Location
        error.push_str(&error_location_style().apply(&format!(
            "  --> {}:{}:{}\n",
            self.location.file, self.location.line, self.location.column
        )));

        // Source line with line number
        error.push_str(&error_source_style().apply(&format!(
            "{:4} | {}\n",
            self.location.line, self.source_line
        )));

        // Error pointer
        let mut pointer = String::from("     | ");
        for _ in 0..self.location.column - 1 {
            pointer.push(' ');
        }
        pointer.push('^');
        error.push_str(&error_pointer_style().apply(&pointer));
        error.push('\n');

        error
    }

    fn get_error_message(&self) -> String {
        match &self.kind {
            ErrorKind::UnexpectedToken { expected, found } => {
                format!("expected {}, found {}", expected, found)
            }
            ErrorKind::UndefinedVariable(name) => {
                format!("undefined variable `{}`", name)
            }
            ErrorKind::SyntaxError(msg) => msg.clone(),
            ErrorKind::InvalidOperator(op) => {
                format!("invalid operator `{}`", op)
            }
            ErrorKind::IOError(msg) => {
                format!("IO error: {}", msg)
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, CompilerError>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(i64),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    If,
    Else,
    While,
    Equals,
    Less,
    Greater,
    Function,
    Return,
    Comma,
    StringLiteral(String),
    LessEqual,
    GreaterEqual,
    Include,
    From,
    Asm,
    Colon,
    LeftBracket,
    RightBracket,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Number(n) => write!(f, "number {}", n),
            TokenType::Identifier(s) => write!(f, "identifier {}", s),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Multiply => write!(f, "*"),
            TokenType::Divide => write!(f, "/"),
            TokenType::Assign => write!(f, "="),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::While => write!(f, "while"),
            TokenType::Equals => write!(f, "=="),
            TokenType::Less => write!(f, "<"),
            TokenType::Greater => write!(f, ">"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Function => write!(f, "fn"),
            TokenType::Return => write!(f, "return"),
            TokenType::Comma => write!(f, ","),
            TokenType::StringLiteral(s) => write!(f, "string \"{}\"", s),
            TokenType::Include => write!(f, "@include"),
            TokenType::From => write!(f, "from"),
            TokenType::Asm => write!(f, "asm"),
            TokenType::Colon => write!(f, ":"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::EOF => write!(f, "end of file"),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    filename: String,
    source_lines: Vec<String>,
}

impl Lexer {
    pub fn new(input: &str, filename: String) -> Self {
        let source_lines: Vec<String> = input.lines().map(String::from).collect();
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            filename,
            source_lines,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        if let Some(ch) = self.peek() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    fn process_escape_sequence(&mut self) -> Result<char> {
        match self.peek() {
            Some(ch) => {
                self.advance();
                match ch {
                    'n' => Ok('\n'),
                    't' => Ok('\t'),
                    'r' => Ok('\r'),
                    '\\' => Ok('\\'),
                    '"' => Ok('"'),
                    '0' => Ok('\0'),
                    'b' => Ok('\x08'), // backspace
                    'f' => Ok('\x0C'), // form feed
                    'v' => Ok('\x0B'), // vertical tab
                    '\'' => Ok('\''),
                    _ => Err(self.create_error(ErrorKind::SyntaxError(format!(
                        "invalid escape sequence: \\{}",
                        ch
                    )))),
                }
            }
            None => Err(self.create_error(ErrorKind::SyntaxError(
                "incomplete escape sequence".to_string(),
            ))),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    pub fn create_error(&self, kind: ErrorKind) -> CompilerError {
        let source_line = if self.line <= self.source_lines.len() {
            self.source_lines[self.line - 1].clone()
        } else {
            String::new()
        };

        CompilerError::new(
            kind,
            SourceLocation {
                line: self.line,
                column: self.column,
                file: self.filename.clone(),
            },
            source_line,
        )
    }

    fn read_string(&mut self) -> Result<Token> {
        let start_column = self.column;
        self.advance(); // Skip opening quote
        let mut string = String::new();

        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.advance();
                    return Ok(Token {
                        token_type: TokenType::StringLiteral(string),
                        line: self.line,
                        column: start_column,
                    });
                }
                '\\' => {
                    self.advance();
                    let escaped_char = self.process_escape_sequence()?;
                    string.push(escaped_char);
                }
                _ => {
                    string.push(ch);
                    self.advance();
                }
            }
        }

        Err(self.create_error(ErrorKind::SyntaxError(
            "unterminated string literal".to_string(),
        )))
    }

    fn read_number(&mut self) -> Result<Token> {
        let start_column = self.column;
        let mut number = String::new();

        while let Some(ch) = self.peek() {
            if !ch.is_digit(10) {
                break;
            }
            number.push(ch);
            self.advance();
        }

        match number.parse() {
            Ok(n) => Ok(Token {
                token_type: TokenType::Number(n),
                line: self.line,
                column: start_column,
            }),
            Err(_) => Err(self.create_error(ErrorKind::SyntaxError(format!(
                "invalid number: {}",
                number
            )))),
        }
    }

    pub fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            while let Some(ch) = self.peek() {
                if !ch.is_whitespace() {
                    break;
                }
                self.advance();
            }

            // Check for comments
            match self.peek() {
                Some('/') => {
                    self.advance();
                    match self.peek() {
                        // Single-line comment //
                        Some('/') => {
                            self.advance();
                            while let Some(ch) = self.peek() {
                                if ch == '\n' {
                                    break;
                                }
                                self.advance();
                            }
                        }
                        // Multi-line comment /* */
                        Some('*') => {
                            self.advance();
                            loop {
                                match self.peek() {
                                    None => {
                                        return;
                                    }
                                    Some('*') => {
                                        self.advance();
                                        if let Some('/') = self.peek() {
                                            self.advance();
                                            break;
                                        }
                                    }
                                    Some(_) => {
                                        self.advance();
                                    }
                                }
                            }
                        }
                        // Not a comment, put the '/' back
                        _ => {
                            self.position -= 1;
                            self.column -= 1;
                            return;
                        }
                    }
                }
                _ => return,
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        self.skip_whitespace_and_comments();

        match self.peek() {
            None => Ok(Token {
                token_type: TokenType::EOF,
                line: self.line,
                column: self.column,
            }),
            Some('@') => {
                self.advance();
                let mut identifier = String::new();
                while let Some(ch) = self.peek() {
                    if !ch.is_alphanumeric() && ch != '_' {
                        break;
                    }
                    identifier.push(ch);
                    self.advance();
                }

                match identifier.as_str() {
                    "include" => Ok(Token {
                        token_type: TokenType::Include,
                        line: self.line,
                        column: self.column - identifier.len() - 1,
                    }),
                    _ => Err(self.create_error(ErrorKind::SyntaxError(format!(
                        "unknown directive @{}",
                        identifier
                    )))),
                }
            }
            Some(':') => {
                let start_column = self.column;
                self.advance();
                Ok(Token {
                    token_type: TokenType::Colon,
                    line: self.line,
                    column: start_column,
                })
            },
            Some(ch) => match ch {
                '0'..='9' => self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
                '+' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Plus,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                '-' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Minus,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                '*' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Multiply,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                '/' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Divide,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Ok(Token {
                            token_type: TokenType::Equals,
                            line: self.line,
                            column: self.column - 2,
                        })
                    } else {
                        Ok(Token {
                            token_type: TokenType::Assign,
                            line: self.line,
                            column: self.column - 1,
                        })
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Ok(Token {
                            token_type: TokenType::LessEqual,
                            line: self.line,
                            column: self.column - 2,
                        })
                    } else {
                        Ok(Token {
                            token_type: TokenType::Less,
                            line: self.line,
                            column: self.column - 1,
                        })
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Ok(Token {
                            token_type: TokenType::GreaterEqual,
                            line: self.line,
                            column: self.column - 2,
                        })
                    } else {
                        Ok(Token {
                            token_type: TokenType::Greater,
                            line: self.line,
                            column: self.column - 1,
                        })
                    }
                }
                ';' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Semicolon,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                '(' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LParen,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                ')' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::RParen,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                '{' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LBrace,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                '}' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::RBrace,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                ',' => {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Comma,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
                '"' => self.read_string(),
                ':' => {
                    let col = self.column;
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Colon,
                        line: self.line,
                        column: col,
                    })
                },
                '[' => {
                    let col = self.column;
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LeftBracket,
                        line: self.line,
                        column: col,
                    })
                },
                ']' => {
                    let col = self.column;
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::RightBracket,
                        line: self.line,
                        column: col,
                    })
                },
                _ => Err(self.create_error(ErrorKind::SyntaxError(format!(
                    "unexpected character: {}",
                    ch
                )))),
            },
        }
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let start_column = self.column;
        let mut identifier = String::new();

        while let Some(ch) = self.peek() {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            identifier.push(ch);
            self.advance();
        }

        let token_type = match identifier.as_str() {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "fn" => TokenType::Function,
            "return" => TokenType::Return,
            "from" => TokenType::From,
            "asm" => TokenType::Asm,
            _ => TokenType::Identifier(identifier),
        };

        Ok(Token {
            token_type,
            line: self.line,
            column: start_column,
        })
    }
}
