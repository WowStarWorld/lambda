use crate::node::declaration::{AccessModifier, Declaration};
use crate::node::expression::Identifier;
use crate::parser::api::{BoxParseResult, ParseResult, Parser};

impl Parser {

    pub fn is_access_modifier(&self) -> bool {
        self.token_buffer.is_identifier_of("public")
            || self.token_buffer.is_identifier_of("private")
            || self.token_buffer.is_identifier_of("protected")
            || self.token_buffer.is_identifier_of("internal")
    }

    pub fn parse_access_modifier(&mut self) -> ParseResult<AccessModifier> {
        match self.parse_identifier()?.downcast::<Identifier>().unwrap().get_name().as_str() {
            "public" => Ok(AccessModifier::Public),
            "private" => Ok(AccessModifier::Private),
            "protected" => Ok(AccessModifier::Protected),
            "internal" => Ok(AccessModifier::Internal),
            _ => Err(self.err("Expected access modifier", None).into()),
        }
    }

    pub fn is_annotated(&self) -> bool {
        self.is_access_modifier() || self.is_function_declaration()
    }

    pub fn parse_annotated(&mut self) -> BoxParseResult<dyn Declaration> {
        let access_modifier = if self.is_access_modifier() {
            Some(self.parse_access_modifier()?)
        } else {
            None
        };
        self.token_buffer.skip_whitespaces();
        if self.is_function_declaration() {
            self.parse_function_declaration(access_modifier)
        } else {
            Err(self.err("Expected an annotated declaration", None).into())
        }
    }

}