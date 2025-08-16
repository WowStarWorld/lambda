use crate::node::expression::{Expression, UnaryExpression};
use crate::parser::api::{Throwable, TokenBuffer};
use crate::parser::expression::base::parse_base_expression;

pub fn is_unary_expression(token_buffer: TokenBuffer) -> bool {
    is_unary_sign_expression(token_buffer.clone())
}

pub fn parse_unary_expression(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Expression>, Throwable> {
    if is_unary_sign_expression(token_buffer.clone()) {
        parse_unary_sign_expression(token_buffer)
    } else {
        Err(token_buffer.err("Expected a unary expression", None).into())
    }
}

pub fn is_unary_sign_expression(token_buffer: TokenBuffer) -> bool {
    token_buffer.is_punctuation_of('+')
        || token_buffer.is_punctuation_of('-')
        || token_buffer.is_punctuation_of('!')
}
pub fn parse_unary_sign_expression(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Expression>, Throwable> {
    let operator = token_buffer.next().unwrap();
    let expression = parse_base_expression(token_buffer);
    match expression {
        Ok(expr) => Ok(Box::new(UnaryExpression { 
            expression: expr,
            operator: operator.get_raw() 
        })),
        Err(err) => Err(err)
    }
}
