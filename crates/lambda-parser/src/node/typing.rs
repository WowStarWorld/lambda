use crate::node::expression::Identifier;
use std::fmt::Debug;

pub trait Type: Debug {}

#[derive(Debug)]
pub struct NamedType {
    pub name: String,
    pub type_arguments: Vec<Box<dyn Type>>,
}
impl Type for NamedType {}

#[derive(Debug)]
pub struct NullableType {
    pub base: Box<dyn Type>,
}
impl Type for NullableType {}

#[derive(Debug)]
pub struct TypeParameter {
    pub name: Identifier,
    pub bound_type: Option<Box<dyn Type>>,
}
