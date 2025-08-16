use once_cell::sync::Lazy;
use crate::parser::api::{Throwable, TokenBuffer};
use crate::parser::expression::base::{is_expression, parse_base_expression, parse_expression};
use crate::node::expression::Expression;
use crate::tokenizer::token::{Token, TokenKind};

pub const BINARY_OPERATORS: &[&[&str]] = &[
    &["**"],
    &["*", "/", "%"],
    &["+", "-"],
    &["==", "!=", ">=", "<=", ">", "<"],
    &["&&", "&"],
    &["||", "|"],
    &["="]
];

pub static BINARY_OPERATORS_FLATTEN: Lazy<Vec<&str>> = Lazy::new(|| {
    let mut flatten: Vec<&str> = Vec::new();
    for operators in BINARY_OPERATORS {
        flatten.extend_from_slice(operators);
    }
    flatten
});

pub fn parse_binary_expression(token_buffer: &mut TokenBuffer) -> Result<Box<dyn Expression>, Throwable> {
    let left = parse_base_expression(token_buffer)?;
    let mut binary_expression_parts: Vec<(String, Box<dyn Expression>)> = Vec::new();
    while token_buffer.has_next() {
        let position = token_buffer.position;
        let binary_operator_part = parse_binary_operator_part(token_buffer);
        match binary_operator_part {
            Ok((operator, right)) => {
                binary_expression_parts.push((operator, right));
            }
            Err(_) => {
                token_buffer.position = position;
                break;
            }
        }
    }
    build_binary_operator(left, binary_expression_parts)
}

fn parse_binary_operator_part(token_buffer: &mut TokenBuffer) -> Result<(String, Box<dyn Expression>), Throwable> {
    let mut operator = String::new();
    token_buffer.skip_whitespaces();
    while token_buffer.has_next() {
        if let Some(Token { kind: TokenKind::Punctuation(char), .. }) = token_buffer.peek() {
            operator.push(*char);
            let count = BINARY_OPERATORS_FLATTEN.iter()
                .filter(|&&op| op.starts_with(operator.as_str()))
                .count();
            let has_next_punctuation = token_buffer.peek_n(1)
                .map_or(false, |next_token| matches!(next_token.kind, TokenKind::Punctuation(_)));
            if count == 0 {
                return Err(token_buffer.err(format!("Invalid operator: {}", operator).as_str(), None).into());
            } else if count == 1 || !has_next_punctuation {
                token_buffer.position += 1;
                token_buffer.skip_whitespaces();
                if !is_expression(token_buffer.clone()) {
                    return Err(token_buffer.err("Expected an expression after operator", None).into());
                }
                let right = parse_expression(token_buffer)?;
                return Ok((operator, right));
            }
            token_buffer.position += 1
        } else {
            break
        }
    }
    Err(token_buffer.err("Expected a binary operator", None).into())
}


fn get_operator_priority(op: &str) -> Option<usize> {
    for (priority, ops) in BINARY_OPERATORS.iter().enumerate() {
        if ops.contains(&op) {
            return Some(priority);
        }
    }
    None
}

pub fn build_binary_operator(
    mut left: Box<dyn Expression>,
    parts: Vec<(String, Box<dyn Expression>)>
) -> Result<Box<dyn Expression>, Throwable> {
    if parts.is_empty() {
        return Ok(left);
    }
    // 按优先级分组
    let mut grouped: Vec<Vec<(String, Box<dyn Expression>)>> = Vec::new();
    let mut current_priority = get_operator_priority(&parts[0].0).unwrap();
    let mut current_group: Vec<(String, Box<dyn Expression>)> = Vec::new();
    for (op, expr) in parts.into_iter() {
        let pri = get_operator_priority(&op).ok_or_else(|| format!("Unknown operator: {}", op))?;
        if pri == current_priority {
            current_group.push((op, expr));
        } else {
            grouped.push(current_group);
            current_group = vec![(op, expr)];
            current_priority = pri;
        }
    }
    grouped.push(current_group);
    // 从高优先级到低优先级递归构造
    for group in grouped.into_iter() {
        let mut iter = group.into_iter();
        while let Some((op, right)) = iter.next() {
            // 构造Token
            left = Box::new(crate::node::expression::BinaryExpression::new(left, right, op));
        }
    }
    Ok(left)
}