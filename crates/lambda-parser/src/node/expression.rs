use crate::node::node::HasToken;
use crate::node::statement::Statement;
use crate::node::typing::Type;
use crate::tokenizer::token::{Token, TokenKind};
use lambda_core::impl_downcast;
use std::any::Any;
use std::fmt::Debug;

pub trait Expression: Debug + Any {}
impl_downcast!(Expression);

// Literal
#[derive(Debug)]
pub struct Literal {
    pub token: Token,
}
impl Expression for Literal {}
impl HasToken for Literal {
    fn get_token(&mut self) -> Token {
        self.token.clone()
    }
}

// Identifier
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
}
impl Identifier {
    pub fn get_name(&self) -> String {
        let TokenKind::Identifier { value, .. } = &self.token.kind else {
            panic!("Expected Identifier token kind");
        };
        value.clone()
    }
}
impl Expression for Identifier {}
impl HasToken for Identifier {
    fn get_token(&mut self) -> Token {
        self.token.clone()
    }
}

// BinaryExpression
#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: String,
}
impl Expression for BinaryExpression {}
impl BinaryExpression {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>, operator: String) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }
}

// CallExpression
#[derive(Debug)]
pub struct FunctionArgument {
    pub name: Option<Identifier>,
    pub value: Box<dyn Expression>,
    pub is_rest: bool,
}

#[derive(Debug)]
pub struct CallExpression {
    pub callee: Box<dyn Expression>,
    pub arguments: Vec<FunctionArgument>,
    pub type_arguments: Vec<Box<dyn Type>>,
}
impl Expression for CallExpression {}

// UnaryExpression
#[derive(Debug)]
pub struct UnaryExpression {
    pub expression: Box<dyn Expression>,
    pub operator: String,
}
impl Expression for UnaryExpression {}

// IfExpression
#[derive(Debug)]
pub struct IfExpression {
    pub test: Box<dyn Expression>,
    pub consequent: Box<dyn Expression>,
    pub alternate: Option<Box<dyn Expression>>,
}
impl Expression for IfExpression {}

// BlockExpression
#[derive(Debug)]
pub struct BlockExpression {
    pub statements: Vec<Box<dyn Statement>>,
    pub return_expression: Option<Box<dyn Expression>>,
}
impl Expression for BlockExpression {}
