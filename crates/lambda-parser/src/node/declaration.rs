use crate::node::expression::Identifier;
use crate::node::statement::Statement;
use crate::node::typing::{Type, TypeParameter};
use std::fmt::Debug;

pub trait Declaration: Debug {}

// MainDeclaration
#[derive(Debug)]
pub struct FunctionParameter {
    pub name: Identifier,
    pub value_type: Box<dyn Type>,
    pub is_rest: bool,
}
#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<FunctionParameter>,
    pub body: Vec<Box<dyn Statement>>,
    pub return_type: Option<Box<dyn Type>>,
}
impl Declaration for FunctionDeclaration {}
