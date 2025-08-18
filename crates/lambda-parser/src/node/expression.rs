use crate::node::node::{HasToken, Node, TokenRange};
use crate::node::statement::Statement;
use crate::node::typing::Type;
use crate::tokenizer::token::{NumberRadix, Token, TokenKind};
use lambda_core::impl_downcast;
use std::fmt::Debug;
use std::str::FromStr;
use bigdecimal::{BigDecimal, Num};
use bigdecimal::num_bigint::{BigInt, ToBigInt};

pub trait Expression: Node {}
impl_downcast!(Expression);

// Literal
#[derive(Debug)]
pub struct Literal {
    pub token: Token,
    pub position: TokenRange
}
impl Literal {
    pub fn is_number(&self) -> bool { matches!(self.token.kind, TokenKind::NumberLiteral { .. }) }
    pub fn is_integer(&self) -> bool {
        if !self.is_number() {
            return false;
        }
        let TokenKind::NumberLiteral { radix, .. } = &self.token.kind else {
            return false;
        };
        if let NumberRadix::Decimal { integer, fraction, exponent } = radix {
            integer.is_some() && fraction.is_none() && exponent.is_none()
        } else {
            true
        }
    }
    pub fn is_character(&self) -> bool { matches!(self.token.kind, TokenKind::CharacterLiteral { .. }) }
    pub fn is_float(&self) -> bool { self.is_number() && !self.is_integer() }
    pub fn is_string(&self) -> bool { matches!(self.token.kind, TokenKind::StringLiteral { .. }) }

    pub fn get_character(&self) -> char {
        let TokenKind::CharacterLiteral { value, .. } = &self.token.kind else {
            panic!("Expected CharacterLiteral token kind"); 
        };
        *value
    }
    
    pub fn get_string(&self) -> String {
        let TokenKind::StringLiteral { value, .. } = &self.token.kind else {
            panic!("Expected StringLiteral token kind");
        };
        value.clone()
    }

    pub fn get_float(&self) -> BigDecimal {
        if let TokenKind::NumberLiteral { radix, .. } = &self.token.kind {
            if let NumberRadix::Decimal { integer, fraction, exponent } = radix {
                let integer = integer.clone().unwrap_or("0".to_string());
                let fraction = fraction.clone().unwrap_or("0".to_string());
                let exponent = exponent.clone().unwrap_or("".to_string());
                let string = format!("{}.{}{}",  integer, fraction, exponent);
                BigDecimal::from_str(string.as_str()).ok().unwrap()
            } else {
                let raw = self.token.get_raw().chars().skip(2).collect::<String>();
                let radix: u32 = match radix {
                    NumberRadix::Binary => 2,
                    NumberRadix::Octal => 8,
                    NumberRadix::Hexadecimal => 16,
                    _ => panic!("Unsupported radix for float conversion")
                };
                BigDecimal::from_str_radix(raw.as_str(), radix).ok().unwrap()
            }
        } else {
            panic!("Expected NumberLiteral token kind");
        }
    }

    pub fn get_integer(&self) -> BigInt {
        self.get_float().to_bigint().unwrap()
    }

}
impl Node for Literal {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Expression for Literal {}
impl HasToken for Literal {
    fn get_token(&mut self) -> Token {
        self.token.clone()
    }
}

// Identifier
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub position: TokenRange
}
impl Identifier {
    pub fn get_name(&self) -> String {
        let TokenKind::Identifier { value, .. } = &self.token.kind else {
            panic!("Expected Identifier token kind");
        };
        value.clone()
    }
}
impl Node for Identifier {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Expression for Identifier {}
impl HasToken for Identifier {
    fn get_token(&mut self) -> Token {
        self.token.clone()
    }
}

// BinaryExpression
#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: String,
    pub position: TokenRange
}
impl Node for BinaryExpression {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Expression for BinaryExpression {}
impl BinaryExpression {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>, operator: String, position: TokenRange) -> Self {
        Self {
            left,
            right,
            operator,
            position
        }
    }
}

// CallExpression
#[derive(Debug)]
pub struct FunctionArgument {
    pub name: Option<Identifier>,
    pub value: Box<dyn Expression>,
    pub is_rest: bool
}

#[derive(Debug)]
pub struct CallExpression {
    pub callee: Box<dyn Expression>,
    pub arguments: Vec<FunctionArgument>,
    pub type_arguments: Vec<Box<dyn Type>>,
    pub position: TokenRange
}
impl Node for CallExpression {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Expression for CallExpression {}

// UnaryExpression
#[derive(Debug)]
pub struct UnaryExpression {
    pub expression: Box<dyn Expression>,
    pub operator: String,
    pub position: TokenRange
}
impl Node for UnaryExpression {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Expression for UnaryExpression {}

// IfExpression
#[derive(Debug)]
pub struct IfExpression {
    pub test: Box<dyn Expression>,
    pub consequent: Box<dyn Expression>,
    pub alternate: Option<Box<dyn Expression>>,
    pub position: TokenRange
}
impl Node for IfExpression {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Expression for IfExpression {}

// BlockExpression
#[derive(Debug)]
pub struct BlockExpression {
    pub statements: Vec<Box<dyn Statement>>,
    pub return_expression: Option<Box<dyn Expression>>,
    pub position: TokenRange
}
impl Node for BlockExpression {
    fn get_position(&self) -> TokenRange { self.position }
}
impl Expression for BlockExpression {}
