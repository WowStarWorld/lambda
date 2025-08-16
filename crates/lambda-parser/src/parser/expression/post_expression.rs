use crate::node::expression::{CallExpression, Expression, FunctionArgument};
use crate::parser::api::{Throwable, TokenBuffer};
use crate::parser::expression::base::parse_expression;
use crate::parser::typing::parse_type_arguments;

pub fn is_post_expression(token_buffer: TokenBuffer) -> bool {
    is_call_expression(token_buffer.clone())
}

pub fn parse_post_expression(token_buffer: &mut TokenBuffer, target: Box<dyn Expression>) -> Result<Box<dyn Expression>, Throwable> {
    let result = if is_call_expression(token_buffer.clone()) {
        parse_call_expression(token_buffer, target)
    } else {
        Err(token_buffer.err("Expected a post expression", None).into())
    };
    match result {
        Ok(result) => {
            token_buffer.skip_whitespaces();
            if is_post_expression(token_buffer.clone()) {
                parse_post_expression(token_buffer, result)
            } else {
                Ok(result)
            }
        }
        Err(err) => Err(err)
    }
}

pub fn parse_function_arguments(token_buffer: &mut TokenBuffer) -> Result<Vec<FunctionArgument>, Throwable> {
    let mut arguments = Vec::new();
    if token_buffer.is_punctuation_of('(') {
        token_buffer.next(); // 跳过 '('
        token_buffer.skip_whitespaces();
        while token_buffer.has_next() {
            token_buffer.skip_whitespaces();
            let is_rest = token_buffer.is_punctuation_of('.');
            if is_rest {
                token_buffer.next(); // 跳过 '.'
                if token_buffer.is_punctuation_of('.') {
                    token_buffer.next(); // 跳过 '.'
                    if !token_buffer.is_punctuation_of('.') {
                        return Err(token_buffer.err("Expected '...' for rest parameter", None).into());
                    }
                    token_buffer.next(); // 跳过 '.'
                    arguments.push(FunctionArgument { base: parse_expression(token_buffer)?, is_rest: true });
                } else {
                    return Err(token_buffer.err("Expected '...' for rest parameter", None).into());
                }
            } else {
                arguments.push(FunctionArgument { base: parse_expression(token_buffer)?, is_rest: false });
            }
            token_buffer.skip_whitespaces();
            if token_buffer.is_punctuation_of(',') {
                token_buffer.next(); // Skip ','
                token_buffer.skip_whitespaces();
            } else if token_buffer.is_punctuation_of(')') {
                break; // End of arguments
            } else {
                return Err(token_buffer.err("Expected ',' or ')' in function arguments", None).into());
            }
        }
        if !token_buffer.is_punctuation_of(')') {
            return Err(token_buffer.err("Expected ')' to close function arguments", None).into());
        }
        token_buffer.next(); // Skip ')'
    }
    Ok(arguments)
}

pub fn parse_call_expression(token_buffer: &mut TokenBuffer, callee: Box<dyn Expression>) -> Result<Box<dyn Expression>, Throwable> {
    let type_arguments = parse_type_arguments(token_buffer)?;
    token_buffer.skip_whitespaces();
    if !token_buffer.is_punctuation_of('(') {
        return Err(token_buffer.err("Expected '('", None).into());
    }
    let arguments = parse_function_arguments(token_buffer)?;
    Ok(Box::new(CallExpression { callee, arguments, type_arguments }))
}

pub fn is_call_expression(token_buffer: TokenBuffer) -> bool {
    token_buffer.is_punctuation_of('(') || token_buffer.is_punctuation_of('<')
}
