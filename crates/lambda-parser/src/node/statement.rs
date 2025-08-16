use crate::node::expression::Expression;
use lambda_core::impl_downcast;
use std::any::Any;
use std::fmt::Debug;

pub trait Statement: Debug + Any {}
impl_downcast!(Statement);

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

// ReturnStatement
#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Option<Box<dyn Expression>>,
}
impl Statement for ReturnStatement {}
impl ReturnStatement {
    pub fn new(expression: Option<Box<dyn Expression>>) -> Self {
        Self { expression }
    }
}

// BlockStatement
#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}
impl Statement for BlockStatement {}

// IfStatement
#[derive(Debug)]
pub struct IfStatement {
    pub test: Box<dyn Expression>,
    pub consequent: Box<dyn Statement>,
    pub alternate: Option<Box<dyn Statement>>,
}
impl Statement for IfStatement {}
