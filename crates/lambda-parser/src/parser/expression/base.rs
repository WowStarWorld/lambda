use crate::parser::api::{Throwable, TokenBuffer};
use crate::parser::expression::binary_expression::parse_binary_expression;
use crate::node::expression::{Expression, Identifier, Literal};
use crate::parser::expression::post_expression::{is_post_expression, parse_post_expression};
use crate::parser::expression::unary_expression::{is_unary_expression, parse_unary_expression};
use crate::tokenizer::token::TokenKind;

pub fn is_expression(token_buffer: TokenBuffer) -> bool {
    is_literal(token_buffer.clone())
        || is_bracket_expression(token_buffer.clone())
        || is_identifier(token_buffer.clone())
        || is_unary_expression(token_buffer.clone())
}

pub fn parse_expression(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Expression>, Throwable> {
    parse_binary_expression(token_buffer)
}

pub fn parse_base_expression(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Expression>, Throwable> {
    token_buffer.skip_whitespaces();
    let result: Result<Box<dyn Expression>, Throwable> = if is_literal(token_buffer.clone()) {
        Ok(parse_literal(token_buffer))
    } else if is_bracket_expression(token_buffer.clone()) {
        parse_bracket_expression(token_buffer)
    } else if is_identifier(token_buffer.clone()) {
        parse_identifier(token_buffer)
    } else if is_unary_expression(token_buffer.clone()) {
        parse_unary_expression(token_buffer)
    } else {
        Err(token_buffer.err("Expected a literal expression", None).into())
    };
    match result {
        Ok(expression) => {
            token_buffer.skip_whitespaces();
            if is_post_expression(token_buffer.clone()) {
                parse_post_expression(token_buffer, expression)
            } else {
                Ok(expression)
            }
        },
        Err(err) => Err(err)
    }
}

pub fn is_identifier(token_buffer: TokenBuffer) -> bool { token_buffer.is_identifier() }
pub fn parse_identifier(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Expression>, Throwable> {
    if is_identifier(token_buffer.clone()) {
        let token = token_buffer.next().unwrap();
        Ok(Box::new(Identifier { token }))
    } else {
        Err(token_buffer.err("Expected an identifier", None).into())
    }
}

pub fn is_bracket_expression(token_buffer: TokenBuffer) -> bool { token_buffer.is_punctuation_of('(') }
pub fn parse_bracket_expression(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Expression>, Throwable> {
    token_buffer.next(); // 跳过 '('
    token_buffer.skip_whitespaces();
    let expression = parse_expression(token_buffer);
    token_buffer.skip_whitespaces();
    if token_buffer.is_punctuation_of(')') {
        token_buffer.next();
        expression
    } else {
        Err(token_buffer.err("Expected ')'", None).into())
    }
}

pub fn is_literal(token_buffer: TokenBuffer) -> bool {
    let next = token_buffer.peek();
    if let Some(token) = next {
        matches!(token.kind, TokenKind::NumberLiteral { .. } | TokenKind::StringLiteral { .. })
    } else {
        false
    }
}

pub fn parse_literal(token_buffer: &mut TokenBuffer) -> Box<Literal> {
    Box::new(Literal { token: token_buffer.next().unwrap() })
}
