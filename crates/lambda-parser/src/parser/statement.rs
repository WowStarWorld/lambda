use crate::parser::api::{Throwable, TokenBuffer};
use crate::parser::expression::base::{is_expression, parse_expression};
use crate::node::statement::{ExpressionStatement, Statement};

pub fn is_statement(token_buffer: TokenBuffer) -> bool { is_expression_statement(token_buffer) }
pub fn parse_statement(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Statement>, Throwable> {
    let clone_token_buffer = token_buffer.clone();
    if is_expression_statement(clone_token_buffer) {
        parse_expression_statement(token_buffer)
    } else {
        Err(token_buffer.err("Expected a valid statement", None).into())
    }
}

pub fn is_expression_statement(token_buffer: TokenBuffer) -> bool { is_expression(token_buffer) }
pub fn parse_expression_statement(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Statement>, Throwable> {
    let expression = parse_expression(token_buffer);
    if expression.is_err() {
        return Err(token_buffer.err("Failed to parse expression in statement", None).into());
    }
    token_buffer.skip_whitespaces();
    if !token_buffer.is_punctuation_of(';') {
        Err(token_buffer.err("Expected ';' at the end of expression statement", None).into())
    } else {
        token_buffer.next(); // 跳过 ';'
        Ok(Box::new(ExpressionStatement::new(expression?)))
    }
}
