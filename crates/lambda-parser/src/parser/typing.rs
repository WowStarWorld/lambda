use crate::node::expression::Identifier;
use crate::node::node::TokenRange;
use crate::node::typing::{NamedType, Type, TypeParameter};
use crate::parser::api::{BoxParseResult, ParseResult, Parser};

pub type Qualified = (Option<String>, String);
pub fn is_same_qualified(a: &Qualified, b: &Qualified) -> bool {
    let a_first = (&a.0).clone();
    let b_first = (&b.0).clone();
    let a_second = (&a.1).clone();
    let b_second = (&b.1).clone();
    if a_first.is_none() && b_first.is_none() {
        return a_second == b_second;
    }
    if a_first.is_some() && b_first.is_some() {
        return a_first.unwrap() == b_first.unwrap() && a_second == b_second;
    }
    false
}

pub fn qualified_to_string(qualified: &Qualified) -> String {
    if let Some(first) = &qualified.0 {
        format!("{}.{}", first, qualified.1)
    } else {
        qualified.1.clone()
    }
}

impl Parser {
    pub fn is_type(&self) -> bool {
        self.is_named_type()
    }

    pub fn parse_type(&mut self) -> BoxParseResult<dyn Type> {
        if self.is_named_type() {
            self.parse_named_type()
        } else if self.is_bracket_type() {
            self.parse_bracket_type()
        } else {
            Err(self.err("Expected a named type", None).into())
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

    pub fn parse_identifier_qualified(&mut self) -> ParseResult<Qualified> {
        let mut start = self
            .parse_identifier()?
            .downcast::<Identifier>()
            .unwrap()
            .get_name();
        let mut list = Vec::new();
        while self.token_buffer.is_punctuation_of('.') {
            self.token_buffer.next(); // 跳过 '.'
            if !self.token_buffer.is_identifier() {
                return Err(self.err("Expected an identifier after '.'", None).into());
            }
            let next = self
                .parse_identifier()?
                .downcast::<Identifier>()
                .unwrap()
                .get_name(); // Identifier
            list.push(next);
        }
        if list.is_empty() {
            return Ok((None, start));
        }
        // 获取并移除最后一项
        let end = list.pop().unwrap();
        if !list.is_empty() {
            start.push('.');
        }
        Ok((Some(start + &*(list.join("."))), end))
    }

    pub fn parse_named_type(&mut self) -> BoxParseResult<dyn Type> {
        let start = self.token_buffer.position;
        let name = self.parse_identifier_qualified()?;
        self.token_buffer.skip_whitespaces();
        let type_arguments = self.parse_type_arguments()?;
        Ok(Box::new(NamedType {
            name,
            type_arguments,
            position: TokenRange::new(start, self.token_buffer.position),
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
