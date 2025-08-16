use crate::tokenizer::token::{Token, TokenKind};
use crate::tokenizer::tokenizer::{SrcInfo, Tokenizer};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub struct TokenBuffer {
    pub tokens: Vec<Token>,
    pub position: usize,
    pub src_info: SrcInfo,
}

pub struct Parser {
    pub token_buffer: TokenBuffer
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self { Self { token_buffer: TokenBuffer::new(tokenizer), } }
    pub fn from_token_buffer(token_buffer: TokenBuffer) -> Self { Self { token_buffer } }
    pub fn err(&self, message: &str, cause: Option<Box<dyn std::error::Error>>) -> SyntaxError {
        self.token_buffer.err(message, cause)
    }
    
    pub fn sub_parser(&self, offset: usize) -> Self {
        Self { token_buffer: self.token_buffer.sub_token_buffer(offset) }
    }
}

impl TokenBuffer {
    pub fn new(mut tokenizer: Tokenizer) -> Self {
        Self {
            tokens: tokenizer.collect().unwrap(),
            src_info: tokenizer.src_info,
            position: 0,
        }
    }
    
    pub fn sub_token_buffer(&self, offset: usize) -> Self {
        let start = self.position + offset;
        let end = self.tokens.len();
        if start >= end {
            return Self {
                tokens: Vec::new(),
                position: 0,
                src_info: self.src_info.clone(),
            };
        }
        Self {
            tokens: self.tokens[start..end].to_vec(),
            position: 0,
            src_info: self.src_info.clone(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }

    pub fn skip_whitespaces(&mut self) {
        while let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Whitespace(_) => self.position += 1,
                _ => break,
            }
        }
    }

    pub fn peek_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.position + n)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position);
        if token.is_some() {
            self.position += 1;
        }
        token.cloned()
    }

    pub fn has_next(&self) -> bool {
        self.position < self.tokens.len()
    }

    pub fn set_position(&mut self, position: usize) {
        if position < self.tokens.len() {
            self.position = position;
        } else {
            self.position = self.tokens.len();
        }
    }

    pub fn err(&self, message: &str, cause: Option<Box<dyn std::error::Error>>) -> SyntaxError {
        SyntaxError {
            message: message.to_string(),
            cause,
            position: self.position,
            token_buffer: self.clone(),
        }
    }
}

pub struct SyntaxError {
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub position: usize,
    pub token_buffer: TokenBuffer,
}

impl SyntaxError {
    pub fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 定位错误 token 的行和列
        let mut line = 1;
        let mut col = 1;
        let tokens = &self.token_buffer.tokens;
        for i in 0..self.position {
            let token = &tokens[i];
            match &token.kind {
                TokenKind::Whitespace(s) => {
                    for (_, character) in s.chars().enumerate() {
                        if character == '\n' {
                            line += 1;
                            col = 1;
                        } else {
                            col += 1;
                        }
                    }
                }
                _ => col += token.get_raw().chars().count(),
            }
        }
        // 输出
        writeln!(f, "SyntaxError: {}", self.message)?;
        writeln!(
            f,
            "    at line {}, column {} ({}:{})",
            line, col, self.token_buffer.src_info.filename, self.position
        )?;
        if self.cause.is_some() {
            writeln!(f, "Caused by {}", self.cause.as_ref().unwrap())
        } else {
            Ok(())
        }
    }
}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}
impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}
impl std::error::Error for SyntaxError {}

pub type Throwable = Box<dyn std::error::Error>;
pub type ParseResult<T> = Result<T, Throwable>;
pub type BoxParseResult<T> = ParseResult<Box<T>>;

impl TokenBuffer {
    pub fn is_identifier(&self) -> bool {
        self.peek().map_or(false, |token| token.is_identifier())
    }
    pub fn is_identifier_of(&self, identifier: &str) -> bool {
        self.peek()
            .map_or(false, |token| token.is_identifier_of(identifier))
    }
    pub fn is_punctuation(&self) -> bool {
        self.peek().map_or(false, |token| token.is_punctuation())
    }
    pub fn is_punctuation_of(&self, punctuation: char) -> bool {
        self.peek()
            .map_or(false, |token| token.is_punctuation_of(punctuation))
    }
}
