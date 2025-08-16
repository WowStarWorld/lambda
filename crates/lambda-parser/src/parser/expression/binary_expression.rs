use crate::node::expression::Expression;
use crate::parser::api::{BoxParseResult, ParseResult, Parser};
use crate::tokenizer::token::{Token, TokenKind};
use once_cell::sync::Lazy;

pub const BINARY_OPERATORS: &[&[&str]] = &[
    &["**"],
    &["*", "/", "%"],
    &["+", "-"],
    &["&&", "&"],
    &["||", "|"],
    &["==", "!=", "===", "!==", ">=", "<=", ">", "<"],
];

pub static BINARY_OPERATORS_FLATTEN: Lazy<Vec<&str>> = Lazy::new(|| {
    let mut flatten: Vec<&str> = Vec::new();
    for operators in BINARY_OPERATORS {
        flatten.extend_from_slice(operators);
    }
    flatten
});

fn get_operator_priority(op: &str) -> Option<usize> {
    for (priority, ops) in BINARY_OPERATORS.iter().enumerate() {
        if ops.contains(&op) {
            return Some(priority);
        }
    }
    None
}

fn build_binary_operator(
    mut left: Box<dyn Expression>,
    parts: Vec<(String, Box<dyn Expression>)>,
) -> BoxParseResult<dyn Expression> {
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
            left = Box::new(crate::node::expression::BinaryExpression::new(
                left, right, op,
            ));
        }
    }
    Ok(left)
}

impl Parser {
    pub fn parse_binary_expression(&mut self) -> BoxParseResult<dyn Expression> {
        let left = self.parse_base_expression()?;
        let mut binary_expression_parts: Vec<(String, Box<dyn Expression>)> = Vec::new();
        while self.token_buffer.has_next() {
            let position = self.token_buffer.position;
            let binary_operator_part = self.parse_binary_operator_part();
            match binary_operator_part {
                Ok((operator, right)) => {
                    binary_expression_parts.push((operator, right));
                }
                Err(_) => {
                    self.token_buffer.position = position;
                    break;
                }
            }
        }
        build_binary_operator(left, binary_expression_parts)
    }

    fn parse_binary_operator_part(&mut self) -> ParseResult<(String, Box<dyn Expression>)> {
        let mut operator = String::new();
        self.token_buffer.skip_whitespaces();
        while self.token_buffer.has_next() {
            if let Some(Token {
                kind: TokenKind::Punctuation(char),
                ..
            }) = self.token_buffer.peek()
            {
                operator.push(*char);
                let count = BINARY_OPERATORS_FLATTEN
                    .iter()
                    .filter(|&&op| op.starts_with(operator.as_str()))
                    .count();
                let has_next_punctuation =
                    self.token_buffer.peek_n(1).map_or(false, |next_token| {
                        matches!(next_token.kind, TokenKind::Punctuation(_))
                    });
                if count == 0 {
                    return Err(self
                        .err(format!("Invalid operator: {}", operator).as_str(), None)
                        .into());
                } else if count == 1 || !has_next_punctuation {
                    self.token_buffer.position += 1;
                    self.token_buffer.skip_whitespaces();
                    if !self.is_expression() {
                        return Err(self
                            .err("Expected an expression after operator", None)
                            .into());
                    }
                    let right = self.parse_expression()?;
                    return Ok((operator, right));
                }
                self.token_buffer.position += 1
            } else {
                break;
            }
        }
        Err(self.err("Expected a binary operator", None).into())
    }
}
