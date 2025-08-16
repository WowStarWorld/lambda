use crate::node::expression::Identifier;
use crate::node::typing::{NamedType, NullableType, Type, TypeParameter};
use crate::parser::api::{Throwable, TokenBuffer};
use crate::parser::expression::base::parse_identifier;

pub fn is_type(token_buffer: TokenBuffer) -> bool {
    is_named_type(token_buffer.clone())
}

pub fn parse_type(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Type>, Throwable> {
    let result = if is_named_type(token_buffer.clone()) {
        parse_named_type(token_buffer)
    } else if is_bracket_type(token_buffer.clone()) {
        parse_bracket_type(token_buffer)
    } else {
        Err(token_buffer.err("Expected a named type", None).into())
    };
    token_buffer.skip_whitespaces();
    if result.is_ok() && token_buffer.is_punctuation_of('?') {
        token_buffer.next(); // 跳过 '?'
        Ok(Box::new(NullableType { base: result? }))
    } else { result }
}

pub fn is_bracket_type(token_buffer: TokenBuffer) -> bool { token_buffer.is_punctuation_of('(') }
pub fn parse_bracket_type(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Type>, Throwable> {
    token_buffer.next(); // 跳过 '('
    token_buffer.skip_whitespaces();
    let parsed_type = parse_type(token_buffer)?;
    token_buffer.skip_whitespaces();
    if token_buffer.is_punctuation_of(')') {
        token_buffer.next(); // 跳过 ')'
        Ok(parsed_type)
    } else {
        Err(token_buffer.err("Expected ')' to close bracket type", None).into())
    }
}

pub fn parse_type_arguments(token_buffer: &mut TokenBuffer) -> Result<Vec<Box<dyn Type>>, Throwable> {
    let mut type_arguments = Vec::new();
    if token_buffer.is_punctuation_of('<') {
        token_buffer.next(); // 跳过 '<'
        token_buffer.skip_whitespaces();
        loop {
            let type_argument = parse_type(token_buffer);
            match type_argument {
                Ok(ty) => type_arguments.push(ty),
                Err(err) => return Err(err)
            }
            token_buffer.skip_whitespaces();
            if token_buffer.is_punctuation_of(',') {
                token_buffer.next(); // 跳过 ','
                token_buffer.skip_whitespaces();
            } else {
                break;
            }
        }
        if token_buffer.is_punctuation_of('>') {
            token_buffer.next(); // 跳过 '>'
        } else {
            return Err(token_buffer.err("Expected '>' to close type arguments", None).into());
        }
    }
    Ok(type_arguments)
}

pub fn parse_named_type(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Type>, Throwable> {
    let mut name = token_buffer.next().unwrap().get_raw().to_string();
    while token_buffer.is_punctuation_of('.') {
        token_buffer.next(); // 条股哦 '.'
        name.push('.');
        if !token_buffer.is_identifier() {
            return Err(token_buffer.err("Expected an identifier after '.'", None).into());
        }
        let next = token_buffer.next().unwrap(); // Identifier
        name.push_str(&next.get_raw());
    }
    token_buffer.skip_whitespaces();
    let type_arguments = parse_type_arguments(token_buffer)?;
    Ok(Box::new(NamedType { name, type_arguments }))
}

pub fn is_named_type(token_buffer: TokenBuffer) -> bool { token_buffer.is_identifier() }

pub fn is_type_parameter(token_buffer: TokenBuffer) -> bool { token_buffer.is_identifier() }

pub fn parse_type_parameter(token_buffer: &mut TokenBuffer) -> Result<TypeParameter, Throwable> {
    let name = parse_identifier(token_buffer)?.downcast::<Identifier>().unwrap().clone();

    token_buffer.skip_whitespaces();
    if token_buffer.is_punctuation_of(':') {
        token_buffer.next(); // 跳过 ':'
        token_buffer.skip_whitespaces();
        let bound_type = parse_type(token_buffer)?;
        Ok(TypeParameter { name, bound_type: Some(bound_type) })
    } else {
        Ok(TypeParameter { name, bound_type: None })
    }
}

pub fn parse_type_parameters(token_buffer: &mut TokenBuffer) -> Result<Vec<TypeParameter>, Throwable> {
    let mut type_parameters = Vec::new();
    if token_buffer.is_punctuation_of('<') {
        token_buffer.next(); // 跳过 '<'
        token_buffer.skip_whitespaces();
        loop {
            let type_parameter = parse_type_parameter(token_buffer)?;
            type_parameters.push(type_parameter);
            token_buffer.skip_whitespaces();
            if token_buffer.is_punctuation_of(',') {
                token_buffer.next(); // 跳过 ','
                token_buffer.skip_whitespaces();
            } else {
                break;
            }
        }
        if token_buffer.is_punctuation_of('>') {
            token_buffer.next(); // 跳过 '>'
        } else {
            return Err(token_buffer.err("Expected '>' to close type parameters", None).into());
        }
    }
    Ok(type_parameters)
}
