use crate::node::expression::{Expression, Identifier};
use crate::node::statement::Statement;
use crate::node::typing::{Type, TypeParameter};
use std::fmt::Debug;
use lambda_core::impl_downcast;
use crate::node::node::{Node, TokenRange};
use crate::tokenizer::token::Token;

pub trait Declaration: Node {}
impl_downcast!(Declaration);

#[derive(Debug, Eq, PartialEq)]
pub enum AccessModifier {
    Public = 1,
    Private = 2,
    Protected = 3,
    Internal = 4,
}

#[derive(Debug, Eq, PartialEq)]
pub enum MemberModifier {
    Open = 1,
    Final = 2,
    Native = 3,
    Abstract = 4,
}

// FunctionDeclaration

#[derive(Debug)]
pub struct FunctionParameter {
    pub name: Identifier,
    pub value_type: Box<dyn Type>,
    pub is_rest: bool,
    pub default_value: Option<Box<dyn Expression>>,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub is_operator: bool,
    pub access_modifier: Option<AccessModifier>,
    pub member_modifier: Option<MemberModifier>,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<FunctionParameter>,
    pub body: Option<Box<dyn Statement>>,
    pub return_type: Option<Box<dyn Type>>,
    pub position: TokenRange
}
impl Node for FunctionDeclaration {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Declaration for FunctionDeclaration {}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub mutable: bool,
    pub access_modifier: Option<AccessModifier>,
    pub member_modifier: Option<MemberModifier>,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<FunctionParameter>,
    pub default_value: Option<Box<dyn Expression>>,
    pub value_type: Option<Box<dyn Type>>,
    pub getter: Option<Box<dyn Statement>>,
    pub setter: Option<(Token, Box<dyn Statement>)>,
    pub delegate: Option<Box<dyn Expression>>,
    pub position: TokenRange
}
impl Node for VariableDeclaration {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Declaration for VariableDeclaration {}

#[derive(Debug)]
pub struct ClassDeclaration {
    pub access_modifier: Option<AccessModifier>,
    pub member_modifier: Option<MemberModifier>,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub super_class: Option<Box<dyn Type>>,
    pub interfaces: Vec<Box<dyn Type>>,
    pub body: Vec<Box<dyn Declaration>>,
    pub position: TokenRange
}
impl Node for ClassDeclaration {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Declaration for ClassDeclaration {}