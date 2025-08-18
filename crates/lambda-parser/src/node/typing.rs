use crate::node::expression::Identifier;
use std::fmt::Debug;
use lambda_core::impl_downcast;
use crate::node::node::{Node, TokenRange};
use crate::parser::typing::Qualified;

pub trait Type: Node {}
impl_downcast!(Type);

#[derive(Debug)]
pub struct NamedType {
    pub name: Qualified,
    pub type_arguments: Vec<Box<dyn Type>>,
    pub position: TokenRange
}
impl Node for NamedType {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Type for NamedType {}

#[derive(Debug)]
pub struct TypeParameter {
    pub name: Identifier,
    pub bound_type: Option<Box<dyn Type>>,
}
