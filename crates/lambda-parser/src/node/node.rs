use std::any::Any;
use std::fmt::Debug;
use lambda_core::impl_downcast;
use crate::tokenizer::token::Token;

pub trait HasToken {
    fn get_token(&mut self) -> Token;
}

#[derive(Copy, Clone, Debug)]
pub struct TokenRange {
    pub start: usize,
    pub end: usize
}

impl TokenRange {
    pub fn new(start: usize, end: usize) -> Self { TokenRange { start, end } }
}

pub trait Node : Debug + Any { fn get_position(&self) -> TokenRange; }
impl_downcast!(Node);
