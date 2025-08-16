use crate::node::expression::{Expression, Identifier, Literal};
use crate::parser::api::{BoxParseResult, Parser};
use crate::tokenizer::token::TokenKind;

impl Parser {
    pub fn is_expression(&self) -> bool {
        self.is_literal()
            || self.is_bracket_expression()
            || self.is_identifier()
            || self.is_unary_expression()
    }

    pub fn parse_expression(&mut self) -> BoxParseResult<dyn Expression> {
        self.parse_binary_expression()
    }

    pub fn parse_base_expression(&mut self) -> BoxParseResult<dyn Expression> {
        self.token_buffer.skip_whitespaces();
        let result: BoxParseResult<dyn Expression> = if self.is_literal() {
            Ok(self.parse_literal())
        } else if self.is_bracket_expression() {
            self.parse_bracket_expression()
        } else if self.is_identifier() {
            self.parse_identifier()
        } else if self.is_unary_expression() {
            self.parse_unary_expression()
        } else {
            Err(self.err("Expected a literal expression", None).into())
        };
        match result {
            Ok(expression) => {
                self.token_buffer.skip_whitespaces();
                if self.is_post_expression() {
                    self.parse_post_expression(expression)
                } else {
                    Ok(expression)
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn is_identifier(&self) -> bool {
        self.token_buffer.is_identifier()
    }
    pub fn parse_identifier(&mut self) -> BoxParseResult<dyn Expression> {
        if self.is_identifier() {
            let token = self.token_buffer.next().unwrap();
            Ok(Box::new(Identifier { token }))
        } else {
            Err(self.err("Expected an identifier", None).into())
        }
    }

    pub fn is_bracket_expression(&self) -> bool {
        self.token_buffer.is_punctuation_of('(')
    }
    pub fn parse_bracket_expression(&mut self) -> BoxParseResult<dyn Expression> {
        self.token_buffer.next(); // 跳过 '('
        self.token_buffer.skip_whitespaces();
        let expression = self.parse_expression();
        self.token_buffer.skip_whitespaces();
        if self.token_buffer.is_punctuation_of(')') {
            self.token_buffer.next();
            expression
        } else {
            Err(self.err("Expected ')'", None).into())
        }
    }

    pub fn is_literal(&self) -> bool {
        let next = self.token_buffer.peek();
        if let Some(token) = next {
            matches!(
                token.kind,
                TokenKind::NumberLiteral { .. } | TokenKind::StringLiteral { .. }
            )
        } else {
            false
        }
    }

    pub fn parse_literal(&mut self) -> Box<Literal> {
        Box::new(Literal {
            token: self.token_buffer.next().unwrap(),
        })
    }
}
