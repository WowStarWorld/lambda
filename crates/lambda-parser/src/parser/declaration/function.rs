use crate::node::declaration::{Declaration, FunctionParameter};
use crate::node::expression::Identifier;
use crate::node::statement::Statement;
use crate::parser::api::{BoxParseResult, ParseResult, Parser};

impl Parser {
    pub fn is_function_declaration(&self) -> bool {
        self.token_buffer.is_identifier_of("func")
    }

    pub fn parse_function_parameter(&mut self) -> ParseResult<FunctionParameter> {
        let is_rest = self
            .token_buffer
            .peek()
            .map_or(false, |x| x.is_punctuation_of('.'))
            && self
                .token_buffer
                .peek_n(1)
                .map_or(false, |x| x.is_punctuation_of('.'))
            && self
                .token_buffer
                .peek_n(2)
                .map_or(false, |x| x.is_punctuation_of('.'));
        if is_rest {
            self.token_buffer.next(); // 跳过 '.'
            self.token_buffer.next(); // 跳过 '.'
            self.token_buffer.next(); // 跳过 '.'
        }
        let identifier = self.parse_identifier()?;
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_punctuation_of(':') {
            return Err(self
                .err("Expected ':' after function parameter name", None)
                .into());
        }
        self.token_buffer.next(); // 跳过 ':'
        self.token_buffer.skip_whitespaces();
        let value_type = self.parse_type()?;
        Ok(FunctionParameter {
            name: identifier.downcast::<Identifier>().unwrap().clone(),
            value_type,
            is_rest,
        })
    }

    pub fn parse_function_parameters(&mut self) -> ParseResult<Vec<FunctionParameter>> {
        let mut parameters = Vec::new();
        if self.token_buffer.is_punctuation_of(')') {
            self.token_buffer.next(); // 跳过 ')'
            return Ok(parameters);
        }
        loop {
            let parameter = self.parse_function_parameter()?;
            if parameter.is_rest {
                self.token_buffer.skip_whitespaces();
                if !self.token_buffer.is_punctuation_of(')') {
                    return Err(self.err("Expected ')' after rest parameter", None).into());
                }
                self.token_buffer.next(); // 跳过 ')'
                break;
            }
            parameters.push(parameter);
            self.token_buffer.skip_whitespaces();
            if self.token_buffer.is_punctuation_of(',') {
                self.token_buffer.next(); // 跳过 ','
                self.token_buffer.skip_whitespaces();
            } else if self.token_buffer.is_punctuation_of(')') {
                self.token_buffer.next(); // 跳过 ')'
                break;
            } else {
                return Err(self
                    .err("Expected ',' or ')' after function parameter", None)
                    .into());
            }
        }
        Ok(parameters)
    }

    pub fn parse_function_declaration(&mut self) -> BoxParseResult<dyn Declaration> {
        self.token_buffer.next(); // 跳过 'fun'
        self.token_buffer.skip_whitespaces();
        let type_parameters = self.parse_type_parameters()?;
        self.token_buffer.skip_whitespaces();
        let name = self
            .parse_identifier()?
            .downcast::<Identifier>()
            .unwrap()
            .clone();
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_punctuation_of('(') {
            return Err(self.err("Expected '(' after function name", None).into());
        }
        self.token_buffer.next(); // 跳过 '('
        self.token_buffer.skip_whitespaces();
        let parameters = self.parse_function_parameters()?;
        self.token_buffer.skip_whitespaces();
        let return_type = if self.token_buffer.is_punctuation_of('-') {
            self.token_buffer.next(); // 跳过 '-'
            if self.token_buffer.is_punctuation_of('>') {
                self.token_buffer.next(); // 跳过 '>'
                self.token_buffer.skip_whitespaces();
                Some(self.parse_type()?)
            } else {
                return Err(self
                    .err("Expected '>' after function parameters", None)
                    .into());
            }
        } else {
            self.token_buffer.skip_whitespaces();
            None
        };
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_punctuation_of('{') {
            return Err(self.err("Expected '{' to start function body", None).into());
        }
        self.token_buffer.next(); // 跳过 '{'
        self.token_buffer.skip_whitespaces();
        let mut body: Vec<Box<dyn Statement>> = Vec::new();
        while !self.token_buffer.is_punctuation_of('}') {
            if !self.is_statement() {
                return Err(self
                    .err("Expected a statement in function body", None)
                    .into());
            }
            let statement = self.parse_statement()?;
            body.push(statement);
            self.token_buffer.skip_whitespaces();
        }
        self.token_buffer.next(); // 跳过 '}'
        Ok(Box::new(crate::node::declaration::FunctionDeclaration {
            name,
            type_parameters,
            parameters,
            body,
            return_type,
        }))
    }
}
