use crate::node::expression::Expression;
use lambda_core::impl_downcast;
use std::any::Any;
use std::fmt::Debug;
use crate::node::node::{Node, TokenRange};

pub trait Statement: Debug + Any + Node {}
impl_downcast!(Statement);

// ExpressionStatement
#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
    pub position: TokenRange,
}
impl Node for ExpressionStatement {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Statement for ExpressionStatement {}

// ReturnStatement
#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Option<Box<dyn Expression>>,
    pub position: TokenRange,
}
impl Node for ReturnStatement {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Statement for ReturnStatement {}

// BlockStatement
#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
    pub position: TokenRange,
}
impl Node for BlockStatement {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Statement for BlockStatement {}

// IfStatement
#[derive(Debug)]
pub struct IfStatement {
    pub test: Box<dyn Expression>,
    pub consequent: Box<dyn Statement>,
    pub alternate: Option<Box<dyn Statement>>,
    pub position: TokenRange,
}
impl Node for IfStatement {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Statement for IfStatement {}
