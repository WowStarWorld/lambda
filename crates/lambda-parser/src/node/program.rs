use crate::node::declaration::Declaration;
use std::fmt::Debug;

#[derive(Debug)]
pub struct PackageDefinition {
    pub name: String
}

#[derive(Debug)]
pub struct Program {
    pub package_definition: PackageDefinition,
    pub declarations: Vec<Box<dyn Declaration>>,
}
