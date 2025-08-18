use crate::node::expression::Expression;
use lambda_core::impl_downcast;
use std::fmt::Debug;
use crate::node::declaration::Declaration;
use crate::node::node::{Node, TokenRange};

pub trait Statement: Node {}
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

// DeclarationStatement
#[derive(Debug)]
pub struct DeclarationStatement {
    pub declaration: Box<dyn Declaration>,
    pub position: TokenRange,
}
impl DeclarationStatement {
    pub fn new(declaration: Box<dyn Declaration>) -> Self {
        let position = declaration.get_position().clone();
        Self { declaration, position }
    }
}
impl Node for DeclarationStatement {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Statement for DeclarationStatement {}
