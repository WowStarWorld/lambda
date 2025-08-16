use crate::node::statement::{ExpressionStatement, Statement};
use crate::parser::api::{BoxParseResult, Parser};

impl Parser {
    pub fn is_statement(&self) -> bool {
        self.is_expression_statement()
    }
    pub fn parse_statement(&mut self) -> BoxParseResult<dyn Statement> {
        if self.is_expression_statement() {
            self.parse_expression_statement()
        } else {
            Err(self.err("Expected a valid statement", None).into())
        }
    }

    pub fn is_expression_statement(&self) -> bool {
        self.is_expression()
    }
    pub fn parse_expression_statement(&mut self) -> BoxParseResult<dyn Statement> {
        let expression = self.parse_expression();
        if expression.is_err() {
            return Err(self
                .err("Failed to parse expression in statement", expression.err())
                .into());
        }
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_punctuation_of(';') {
            Err(self
                .err("Expected ';' at the end of expression statement", None)
                .into())
        } else {
            self.token_buffer.next(); // 跳过 ';'
            Ok(Box::new(ExpressionStatement::new(expression?)))
        }
    }
}
