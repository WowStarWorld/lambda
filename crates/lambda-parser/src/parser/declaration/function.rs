use crate::node::declaration::{Declaration, FunctionParameter};
use crate::node::expression::Identifier;
use crate::node::statement::Statement;
use crate::parser::api::{Throwable, TokenBuffer};
use crate::parser::expression::base::parse_identifier;
use crate::parser::statement::{is_statement, parse_statement};
use crate::parser::typing::{parse_type, parse_type_parameters};

pub fn is_function_declaration(token_buffer: TokenBuffer) -> bool { token_buffer.is_identifier_of("func") }

fn parse_function_parameter(token_buffer: &mut TokenBuffer) -> Result<FunctionParameter, Throwable> {
    let is_rest = token_buffer.peek().map_or(false, |x| x.is_punctuation_of('.'))
        && token_buffer.peek_n(1).map_or(false, |x| x.is_punctuation_of('.'))
        && token_buffer.peek_n(2).map_or(false, |x| x.is_punctuation_of('.'));
    if is_rest {
        token_buffer.next(); // 跳过 '.'
        token_buffer.next(); // 跳过 '.'
        token_buffer.next(); // 跳过 '.'
    }
    let identifier = parse_identifier(token_buffer)?;
    token_buffer.skip_whitespaces();
    if !token_buffer.is_punctuation_of(':') {
        return Err(token_buffer.err("Expected ':' after function parameter name", None).into());
    }
    token_buffer.next(); // 跳过 ':'
    token_buffer.skip_whitespaces();
    let value_type = parse_type(token_buffer)?;
    Ok(FunctionParameter { name: identifier.downcast::<Identifier>().unwrap().clone(), value_type, is_rest })
}

fn parse_function_parameters(token_buffer: &mut TokenBuffer) -> Result<Vec<FunctionParameter>, Throwable> {
    let mut parameters = Vec::new();
    if token_buffer.is_punctuation_of(')') {
        token_buffer.next(); // 跳过 ')'
        return Ok(parameters);
    }
    loop {
        let parameter = parse_function_parameter(token_buffer)?;
        if parameter.is_rest {
            token_buffer.skip_whitespaces();
            if !token_buffer.is_punctuation_of(')') {
                return Err(token_buffer.err("Expected ')' after rest parameter", None).into());
            }
            token_buffer.next(); // 跳过 ')'
            break;
        }
        parameters.push(parameter);
        token_buffer.skip_whitespaces();
        if token_buffer.is_punctuation_of(',') {
            token_buffer.next(); // 跳过 ','
            token_buffer.skip_whitespaces();
        } else if token_buffer.is_punctuation_of(')') {
            token_buffer.next(); // 跳过 ')'
            break;
        } else {
            return Err(token_buffer.err("Expected ',' or ')' after function parameter", None).into());
        }
    }
    Ok(parameters)
}

pub fn parse_function_declaration(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Declaration>, Throwable> {
    token_buffer.next(); // 跳过 'fun'
    token_buffer.skip_whitespaces();
    let type_parameters = parse_type_parameters(token_buffer)?;
    token_buffer.skip_whitespaces();
    let name = parse_identifier(token_buffer)?.downcast::<Identifier>().unwrap().clone();
    token_buffer.skip_whitespaces();
    if !token_buffer.is_punctuation_of('(') {
        return Err(token_buffer.err("Expected '(' after function name", None).into());
    }
    token_buffer.next(); // 跳过 '('
    token_buffer.skip_whitespaces();
    let parameters = parse_function_parameters(token_buffer)?;
    token_buffer.skip_whitespaces();
    let return_type = if token_buffer.is_punctuation_of('-') {
        token_buffer.next(); // 跳过 '-'
        if token_buffer.is_punctuation_of('>') {
            token_buffer.next(); // 跳过 '>'
            token_buffer.skip_whitespaces();
            Some(parse_type(token_buffer)?)
        } else {
            return Err(token_buffer.err("Expected '>' after function parameters", None).into());
        }
    } else {
        token_buffer.skip_whitespaces();
        None
    };
    token_buffer.skip_whitespaces();
    if !token_buffer.is_punctuation_of('{') {
        return Err(token_buffer.err("Expected '{' to start function body", None).into());
    }
    token_buffer.next(); // 跳过 '{'
    token_buffer.skip_whitespaces();
    let mut body: Vec<Box<dyn Statement>> = Vec::new();
    while !token_buffer.is_punctuation_of('}') {
        if !is_statement(token_buffer.clone()) {
            return Err(token_buffer.err("Expected a statement in function body", None).into());
        }
        let statement = parse_statement(token_buffer)?;
        body.push(statement);
        token_buffer.skip_whitespaces();
    }
    token_buffer.next(); // 跳过 '}'
    Ok(Box::new(crate::node::declaration::FunctionDeclaration {
        name,
        type_parameters,
        parameters,
        body,
        return_type
    }))
}

