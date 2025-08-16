use std::fmt::Debug;
use crate::node::declaration::Declaration;

#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Box<dyn Declaration>>
}
