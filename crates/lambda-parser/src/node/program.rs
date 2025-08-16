use crate::node::declaration::Declaration;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Box<dyn Declaration>>,
}
