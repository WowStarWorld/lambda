use crate::node::expression::Identifier;
use crate::node::statement::Statement;
use crate::node::typing::{Type, TypeParameter};
use lambda_core::impl_downcast;
use std::any::Any;
use std::fmt::Debug;

pub trait Declaration: Debug + Any {}
impl_downcast!(Declaration);

#[derive(Debug, Eq, PartialEq)]
pub enum AccessModifier {
    Public,
    Private,
    Protected,
    Internal,
}

// FunctionDeclaration
#[derive(Debug)]
pub struct FunctionParameter {
    pub name: Identifier,
    pub value_type: Box<dyn Type>,
    pub is_rest: bool,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub access_modifier: Option<AccessModifier>,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<FunctionParameter>,
    pub body: Box<dyn Statement>,
    pub return_type: Option<Box<dyn Type>>,
}
impl Declaration for FunctionDeclaration {}
