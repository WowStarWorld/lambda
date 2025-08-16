use crate::node::declaration::Declaration;
use crate::parser::api::{Throwable, TokenBuffer};
use crate::parser::declaration::function::{is_function_declaration, parse_function_declaration};

pub fn is_top_level_declaration(token_buffer: TokenBuffer) -> bool {
    is_function_declaration(token_buffer.clone())
}

pub fn parse_top_level_declaration(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Declaration>, Throwable> {
    if is_function_declaration(token_buffer.clone()) {
        parse_function_declaration(token_buffer)
    } else {
        Err(token_buffer.err("Expected a top-level declaration", None).into())
    }
}

pub fn parse_program(token_buffer: &mut TokenBuffer) -> Result<Vec<Box<dyn Declaration>>, Throwable> {
    let mut declarations = Vec::new();
    token_buffer.skip_whitespaces();
    while token_buffer.has_next() {
        if is_top_level_declaration(token_buffer.clone()) {
            declarations.push(parse_top_level_declaration(token_buffer)?);
        } else {
            return Err(token_buffer.err("Unexpected token in program", None).into());
        }
        token_buffer.skip_whitespaces();
    }
    Ok(declarations)
}
