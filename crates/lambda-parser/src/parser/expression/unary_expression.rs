use crate::node::expression::{Expression, UnaryExpression};
use crate::parser::api::{BoxParseResult, Parser};

impl Parser {
    pub fn is_unary_expression(&self) -> bool {
        self.is_unary_sign_expression()
    }

    pub fn parse_unary_expression(&mut self) -> BoxParseResult<dyn Expression> {
        if self.is_unary_sign_expression() {
            self.parse_unary_sign_expression()
        } else {
            Err(self.err("Expected a unary expression", None).into())
        }
    }

    pub fn is_unary_sign_expression(&self) -> bool {
        self.token_buffer.is_punctuation_of('+')
            || self.token_buffer.is_punctuation_of('-')
            || self.token_buffer.is_punctuation_of('!')
    }
    pub fn parse_unary_sign_expression(&mut self) -> BoxParseResult<dyn Expression> {
        let operator = self.token_buffer.next().unwrap();
        let expression = self.parse_base_expression();
        match expression {
            Ok(expr) => Ok(Box::new(UnaryExpression {
                expression: expr,
                operator: operator.get_raw(),
            })),
            Err(err) => Err(err),
        }
    }
}
