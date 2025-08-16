use std::fmt::{Debug};
use crate::node::expression::Expression;

pub trait Statement : Debug {}

// ExpressionStatement
#[derive(Debug)]
pub struct ExpressionStatement { pub expression: Box<dyn Expression> }
impl Statement for ExpressionStatement {}
impl ExpressionStatement { pub fn new(expression: Box<dyn Expression>) -> Self { Self { expression } } }

