use crate::node::expression::Identifier;
use std::fmt::Debug;
use crate::node::node::{Node, TokenRange};

pub trait Type: Debug + Node {}

#[derive(Debug)]
pub struct NamedType {
    pub name: String,
    pub type_arguments: Vec<Box<dyn Type>>,
    pub position: TokenRange
}
impl Node for NamedType {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Type for NamedType {}

#[derive(Debug)]
pub struct NullableType {
    pub base: Box<dyn Type>,
    pub position: TokenRange
}
impl Node for NullableType {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Type for NullableType {}

#[derive(Debug)]
pub struct TypeParameter {
    pub name: Identifier,
    pub bound_type: Option<Box<dyn Type>>,
}
