use crate::node::declaration::{AccessModifier, Declaration, MemberModifier};
use crate::node::expression::Identifier;
use crate::parser::api::{BoxParseResult, ParseResult, Parser};

impl Parser {
    pub fn is_access_modifier(&self) -> bool {
        self.token_buffer.is_identifier_of("public")
            || self.token_buffer.is_identifier_of("private")
            || self.token_buffer.is_identifier_of("protected")
            || self.token_buffer.is_identifier_of("internal")
    }

    pub fn is_member_modifier(&self) -> bool {
        self.token_buffer.is_identifier_of("native")
            || self.token_buffer.is_identifier_of("abstract")
            || self.token_buffer.is_identifier_of("open")
            || self.token_buffer.is_identifier_of("final")
    }
    pub fn passe_member_modifier(&mut self) -> ParseResult<MemberModifier> {
        match self
            .parse_identifier()?
            .downcast::<Identifier>()
            .unwrap()
            .get_name()
            .as_str()
        {
            "native" => Ok(MemberModifier::Native),
            "abstract" => Ok(MemberModifier::Abstract),
            "open" => Ok(MemberModifier::Open),
            "final" => Ok(MemberModifier::Final),
            _ => Err(self.err("Expected member modifier", None).into()),
        }
    }

    pub fn parse_access_modifier(&mut self) -> ParseResult<AccessModifier> {
        match self
            .parse_identifier()?
            .downcast::<Identifier>()
            .unwrap()
            .get_name()
            .as_str()
        {
            "public" => Ok(AccessModifier::Public),
            "private" => Ok(AccessModifier::Private),
            "protected" => Ok(AccessModifier::Protected),
            "internal" => Ok(AccessModifier::Internal),
            _ => Err(self.err("Expected access modifier", None).into()),
        }
    }

    pub fn is_annotated_declaration(&self) -> bool {
        self.is_access_modifier()
            || self.is_member_modifier()
            || self.is_function_declaration()
            || self.is_variable_declaration()
            || self.is_class_declaration()
    }

    pub fn parse_annotated_declaration(&mut self) -> BoxParseResult<dyn Declaration> {
        let start = self.token_buffer.position;
        let access_modifier = if self.is_access_modifier() {
            Some(self.parse_access_modifier()?)
        } else {
            None
        };
        self.token_buffer.skip_whitespaces();
        let member_modifier = if self.is_member_modifier() {
            Some(self.passe_member_modifier()?)
        } else {
            None
        };
        self.token_buffer.skip_whitespaces();
        if self.is_function_declaration() {
            self.parse_function_declaration(access_modifier, member_modifier, start)
        } else if self.is_variable_declaration() {
            self.parse_variable_declaration(access_modifier, member_modifier, start)
        } else if self.is_class_declaration() {
            self.parse_class_declaration(access_modifier, member_modifier, start)
        } else {
            Err(self.err("Expected an annotated declaration", None).into())
        }
    }
}
