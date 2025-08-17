use crate::node::expression::{Expression, Identifier};
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

#[derive(Debug, Eq, PartialEq)]
pub enum OverridableModifier {
    Open,
    Final,
}

// FunctionDeclaration

#[derive(Debug, Eq, PartialEq)]
pub enum FunctionModifier {
    Native,
    Abstract,
}
#[derive(Debug)]
pub struct FunctionParameter {
    pub name: Identifier,
    pub value_type: Box<dyn Type>,
    pub is_rest: bool,
    pub default_value: Option<Box<dyn Expression>>,
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

#[derive(Debug)]
pub struct NoBodyFunctionDeclaration {
    pub modifier: Option<FunctionModifier>,
    pub access_modifier: Option<AccessModifier>,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<Box<dyn Type>>,
}
