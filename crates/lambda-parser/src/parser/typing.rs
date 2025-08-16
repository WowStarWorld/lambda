use crate::node::expression::Identifier;
use crate::node::typing::{NamedType, NullableType, Type, TypeParameter};
use crate::parser::api::{BoxParseResult, ParseResult, Parser};

impl Parser {
    pub fn is_type(&self) -> bool {
        self.is_named_type()
    }

    pub fn parse_type(&mut self) -> BoxParseResult<dyn Type> {
        let result = if self.is_named_type() {
            self.parse_named_type()
        } else if self.is_bracket_type() {
            self.parse_bracket_type()
        } else {
            Err(self.err("Expected a named type", None).into())
        };
        self.token_buffer.skip_whitespaces();
        if result.is_ok() && self.token_buffer.is_punctuation_of('?') {
            self.token_buffer.next(); // 跳过 '?'
            Ok(Box::new(NullableType { base: result? }))
        } else {
            result
        }
    }

    pub fn is_bracket_type(&self) -> bool {
        self.token_buffer.is_punctuation_of('(')
    }
    pub fn parse_bracket_type(&mut self) -> BoxParseResult<dyn Type> {
        self.token_buffer.next(); // 跳过 '('
        self.token_buffer.skip_whitespaces();
        let parsed_type = self.parse_type()?;
        self.token_buffer.skip_whitespaces();
        if self.token_buffer.is_punctuation_of(')') {
            self.token_buffer.next(); // 跳过 ')'
            Ok(parsed_type)
        } else {
            Err(self.err("Expected ')' to close bracket type", None).into())
        }
    }

    pub fn parse_type_arguments(&mut self) -> ParseResult<Vec<Box<dyn Type>>> {
        let mut type_arguments = Vec::new();
        if self.token_buffer.is_punctuation_of('<') {
            self.token_buffer.next(); // 跳过 '<'
            self.token_buffer.skip_whitespaces();
            loop {
                let type_argument = self.parse_type();
                match type_argument {
                    Ok(ty) => type_arguments.push(ty),
                    Err(err) => return Err(err),
                }
                self.token_buffer.skip_whitespaces();
                if self.token_buffer.is_punctuation_of(',') {
                    self.token_buffer.next(); // 跳过 ','
                    self.token_buffer.skip_whitespaces();
                } else {
                    break;
                }
            }
            if self.token_buffer.is_punctuation_of('>') {
                self.token_buffer.next(); // 跳过 '>'
            } else {
                return Err(self
                    .err("Expected '>' to close type arguments", None)
                    .into());
            }
        }
        Ok(type_arguments)
    }

    pub fn parse_identifier_list(&mut self) -> ParseResult<String> {
        let mut name = self.parse_identifier()?.downcast::<Identifier>().unwrap().get_name();
        while self.token_buffer.is_punctuation_of('.') {
            self.token_buffer.next(); // 跳过 '.'
            name.push('.');
            if !self.token_buffer.is_identifier() {
                return Err(self.err("Expected an identifier after '.'", None).into());
            }
            let next = self.parse_identifier()?.downcast::<Identifier>().unwrap().get_name(); // Identifier
            name.push_str(&next);
        }
        self.token_buffer.skip_whitespaces();
        Ok(name)
    }
    
    pub fn parse_named_type(&mut self) -> BoxParseResult<dyn Type> {
        let name = self.parse_identifier_list()?;
        self.token_buffer.skip_whitespaces();
        let type_arguments = self.parse_type_arguments()?;
        Ok(Box::new(NamedType {
            name,
            type_arguments,
        }))
    }

    pub fn is_named_type(&self) -> bool {
        self.token_buffer.is_identifier()
    }

    pub fn is_type_parameter(&self) -> bool {
        self.token_buffer.is_identifier()
    }

    pub fn parse_type_parameter(&mut self) -> ParseResult<TypeParameter> {
        let name = self
            .parse_identifier()?
            .downcast::<Identifier>()
            .unwrap()
            .clone();

        self.token_buffer.skip_whitespaces();
        if self.token_buffer.is_punctuation_of(':') {
            self.token_buffer.next(); // 跳过 ':'
            self.token_buffer.skip_whitespaces();
            let bound_type = self.parse_type()?;
            Ok(TypeParameter {
                name,
                bound_type: Some(bound_type),
            })
        } else {
            Ok(TypeParameter {
                name,
                bound_type: None,
            })
        }
    }

    pub fn parse_type_parameters(&mut self) -> ParseResult<Vec<TypeParameter>> {
        let mut type_parameters = Vec::new();
        if self.token_buffer.is_punctuation_of('<') {
            self.token_buffer.next(); // 跳过 '<'
            self.token_buffer.skip_whitespaces();
            loop {
                let type_parameter = self.parse_type_parameter()?;
                type_parameters.push(type_parameter);
                self.token_buffer.skip_whitespaces();
                if self.token_buffer.is_punctuation_of(',') {
                    self.token_buffer.next(); // 跳过 ','
                    self.token_buffer.skip_whitespaces();
                } else {
                    break;
                }
            }
            if self.token_buffer.is_punctuation_of('>') {
                self.token_buffer.next(); // 跳过 '>'
            } else {
                return Err(self
                    .err("Expected '>' to close type parameters", None)
                    .into());
            }
        }
        Ok(type_parameters)
    }
}
