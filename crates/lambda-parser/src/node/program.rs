use crate::node::declaration::Declaration;
use std::fmt::Debug;
use crate::node::node::{Node, TokenRange};

// PackageDefinition
#[derive(Debug)]
pub struct PackageDefinition {
    pub name: String,
}

#[derive(Debug)]
pub struct ImportDefinition {
    pub package_name: String,
    pub member: String,
}

// Program
#[derive(Debug)]
pub struct Program {
    pub package_definition: PackageDefinition,
    pub import_definitions: Vec<ImportDefinition>,
    pub declarations: Vec<Box<dyn Declaration>>,
    pub position: TokenRange,
}

impl Node for Program {
    fn get_position(&self) -> TokenRange { self.position }
}

