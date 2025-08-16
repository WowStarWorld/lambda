use crate::tokenizer::token::Token;

pub trait HasToken {
    fn get_token(&mut self) -> Token;
}
