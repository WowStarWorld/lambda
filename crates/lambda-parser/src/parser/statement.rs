use crate::node::node::TokenRange;
use crate::node::statement::{
    BlockStatement, ExpressionStatement, IfStatement, ReturnStatement, Statement,
};
use crate::parser::api::{BoxParseResult, Parser};

impl Parser {
    pub fn is_statement(&self) -> bool {
        self.is_return_statement()
            || self.is_block_statement()
            || self.is_if_statement()
            || self.is_expression_statement()
    }
    pub fn parse_statement(&mut self) -> BoxParseResult<dyn Statement> {
        if self.is_return_statement() {
            self.parse_return_statement()
        } else if self.is_block_statement() {
            self.parse_block_statement()
        } else if self.is_if_statement() {
            self.parse_if_statement()
        } else if self.is_expression_statement() {
            self.parse_expression_statement()
        } else {
            Err(self.err("Expected a valid statement", None).into())
        }
    }

    pub fn is_expression_statement(&self) -> bool {
        self.is_expression()
    }

    pub fn parse_expression_statement(&mut self) -> BoxParseResult<dyn Statement> {
        let expression = self.parse_expression()?;
        let start = expression.get_position().start;
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_line_break() {
            Err(self
                .err(
                    "Expected line-break at the end of expression statement",
                    None,
                )
                .into())
        } else {
            self.token_buffer.skip_line_break();
            let end = self.token_buffer.position;
            Ok(Box::new(ExpressionStatement { expression, position: TokenRange::new(start, end) }))
        }
    }

    pub fn is_return_statement(&self) -> bool {
        self.token_buffer.is_identifier_of("return")
    }
    pub fn parse_return_statement(&mut self) -> BoxParseResult<dyn Statement> {
        let start = self.token_buffer.position;
        if !self.is_return_statement() {
            return Err(self.err("Expected 'return' statement", None).into());
        }
        self.token_buffer.next(); // 跳过 'return'
        self.token_buffer.skip_whitespaces();
        let expression = if self.is_expression() {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_line_break() {
            return Err(self
                .err("Expected line-break at the end of return statement", None)
                .into());
        }
        self.token_buffer.skip_line_break();
        Ok(Box::new(ReturnStatement { expression, position: TokenRange::new(start, self.token_buffer.position) }))
    }

    pub fn is_block_statement(&self) -> bool {
        self.token_buffer.is_punctuation_of('{')
    }
    pub fn parse_block_statement(&mut self) -> BoxParseResult<dyn Statement> {
        let start = self.token_buffer.position;
        self.token_buffer.next(); // 跳过 '{'
        self.token_buffer.skip_whitespaces();
        let mut body: Vec<Box<dyn Statement>> = Vec::new();
        while !self.token_buffer.is_punctuation_of('}') {
            if !self.is_statement() {
                return Err(self
                    .err("Expected a statement in function body", None)
                    .into());
            }
            let statement = self.parse_statement()?;
            body.push(statement);
            self.token_buffer.skip_whitespaces();
        }
        self.token_buffer.next(); // 跳过 '}'
        Ok(Box::new(BlockStatement { statements: body, position: TokenRange::new(start, self.token_buffer.position) }))
    }

    pub fn is_if_statement(&self) -> bool {
        self.token_buffer.is_identifier_of("if")
    }
    pub fn parse_if_statement(&mut self) -> BoxParseResult<dyn Statement> {
        let start = self.token_buffer.position;
        self.token_buffer.next();
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_punctuation_of('(') {
            return Err(self.err("Expected '(' after 'if'", None).into());
        }
        self.token_buffer.next(); // 跳过 '('
        self.token_buffer.skip_whitespaces();
        let test = self.parse_expression()?;
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_punctuation_of(')') {
            return Err(self.err("Expected ')' after condition", None).into());
        }
        self.token_buffer.next(); // 跳过 ')'
        self.token_buffer.skip_whitespaces();
        if !self.is_statement() {
            return Err(self
                .err("Expected statement after 'if' condition", None)
                .into());
        }
        let consequent = self.parse_statement()?;
        self.token_buffer.skip_whitespaces();
        let alternate = if self.token_buffer.is_identifier_of("else") {
            self.token_buffer.next(); // 跳过 'else'
            self.token_buffer.skip_whitespaces();
            if !self.is_statement() {
                return Err(self.err("Expected statement after 'else'", None).into());
            }
            Some(self.parse_statement()?)
        } else {
            None
        };
        Ok(Box::new(IfStatement {
            test,
            consequent,
            alternate,
            position: TokenRange::new(start, self.token_buffer.position),
        }))
    }
}
