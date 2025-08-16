use crate::node::expression::Expression;
use std::fmt::Debug;

pub trait Statement: Debug {}

// ExpressionStatement
#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}
impl Statement for ExpressionStatement {}
impl ExpressionStatement {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}
