use std::any::Any;
use std::fmt::{Debug};
use lambda_core::impl_downcast;
use crate::node::node::{HasToken};
use crate::node::typing::Type;
use crate::tokenizer::token::Token;

pub trait Expression: Debug + Any {}
impl_downcast!(Expression);

// Literal
#[derive(Debug)]
pub struct Literal { pub token: Token }
impl Expression for Literal {}
impl HasToken for Literal { fn get_token(&mut self) -> Token { self.token.clone() } }

// Identifier
#[derive(Debug, Clone)]
pub struct Identifier { pub token: Token }
impl Identifier { pub fn to_expression(&self) -> &dyn Expression { self } }
impl Expression for Identifier {}
impl HasToken for Identifier { fn get_token(&mut self) -> Token { self.token.clone() } }

// BinaryExpression
#[derive(Debug)]
pub struct BinaryExpression { pub left: Box<dyn Expression>, pub right: Box<dyn Expression>, pub operator: String }
impl Expression for BinaryExpression {}
impl BinaryExpression { pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>, operator: String) -> Self { Self { left, right, operator } } }

#[derive(Debug)]
pub struct FunctionArgument {
    pub base: Box<dyn Expression>,
    pub is_rest: bool
}

#[derive(Debug)]
pub struct CallExpression { pub callee: Box<dyn Expression>, pub arguments: Vec<FunctionArgument>, pub type_arguments: Vec<Box<dyn Type>> }
impl Expression for CallExpression {}

#[derive(Debug)]
pub struct UnaryExpression { pub expression: Box<dyn Expression>, pub operator: String }
impl Expression for UnaryExpression {}
