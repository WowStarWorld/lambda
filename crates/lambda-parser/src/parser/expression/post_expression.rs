use crate::node::expression::{CallExpression, Expression, FunctionArgument, Identifier};
use crate::node::node::TokenRange;
use crate::parser::api::{BoxParseResult, ParseResult, Parser};

impl Parser {
    pub fn is_post_expression(&self) -> bool {
        self.is_call_expression()
    }

    pub fn parse_post_expression(
        &mut self,
        target: Box<dyn Expression>,
    ) -> BoxParseResult<dyn Expression> {
        let result = if self.is_call_expression() {
            self.parse_call_expression(target)
        } else {
            Err(self.err("Expected a post expression", None).into())
        };
        match result {
            Ok(result) => {
                self.token_buffer.skip_whitespaces();
                if self.is_post_expression() {
                    self.parse_post_expression(result)
                } else {
                    Ok(result)
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn parse_function_arguments(&mut self) -> ParseResult<Vec<FunctionArgument>> {
        let mut arguments = Vec::new();
        if self.token_buffer.is_punctuation_of('(') {
            self.token_buffer.next(); // 跳过 '('
            self.token_buffer.skip_whitespaces();
            if self.token_buffer.is_punctuation_of(')') {
                self.token_buffer.next(); // 跳过 ')'
                return Ok(arguments); // 没有参数
            }
            while self.token_buffer.has_next() {
                self.token_buffer.skip_whitespaces();
                let is_rest = self.token_buffer.is_punctuation_of('*');
                if is_rest { 
                    self.token_buffer.next(); // 跳过 '*'
                    arguments.push(FunctionArgument {
                        value: self.parse_expression()?,
                        is_rest: true,
                        name: None,
                    });
                } else {
                    let name = if self.token_buffer.is_identifier() {
                        let start = self.token_buffer.position;
                        let identifier = self.token_buffer.next().unwrap();
                        let identifier_end = self.token_buffer.position;
                        self.token_buffer.skip_whitespaces();
                        if self.token_buffer.is_punctuation_of('=') {
                            self.token_buffer.next();
                            self.token_buffer.skip_whitespaces();
                            Some(Identifier { token: identifier, position: TokenRange::new(start, identifier_end)})
                        } else {
                            self.token_buffer.position = start;
                            None
                        }
                    } else {
                        None
                    };
                    arguments.push(FunctionArgument {
                        name,
                        value: self.parse_expression()?,
                        is_rest: false,
                    });
                }
                self.token_buffer.skip_whitespaces();
                if self.token_buffer.is_punctuation_of(',') {
                    self.token_buffer.next(); // Skip ','
                    self.token_buffer.skip_whitespaces();
                } else if self.token_buffer.is_punctuation_of(')') {
                    break; // End of arguments
                } else {
                    return Err(self
                        .err("Expected ',' or ')' in function arguments", None)
                        .into());
                }
            }
            if !self.token_buffer.is_punctuation_of(')') {
                return Err(self
                    .err("Expected ')' to close function arguments", None)
                    .into());
            }
            self.token_buffer.next(); // Skip ')'
        }
        Ok(arguments)
    }

    pub fn parse_call_expression(
        &mut self,
        callee: Box<dyn Expression>,
    ) -> BoxParseResult<dyn Expression> {
        let start = callee.get_position().start;
        let type_arguments = self.parse_type_arguments()?;
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_punctuation_of('(') {
            return Err(self.err("Expected '('", None).into());
        }
        let arguments = self.parse_function_arguments()?;
        Ok(Box::new(CallExpression {
            callee,
            arguments,
            type_arguments,
            position: TokenRange::new(start, self.token_buffer.position)
        }))
    }

    pub fn is_call_expression(&self) -> bool {
        self.token_buffer.is_punctuation_of('(')
            || (self.token_buffer.is_punctuation_of('<')
                && self.sub_parser(0).parse_type_arguments().is_ok())
    }
}
