use crate::node::declaration::{AccessModifier, ClassDeclaration, Declaration, MemberModifier};
use crate::node::expression::Identifier;
use crate::node::node::TokenRange;
use crate::parser::api::{BoxParseResult, Parser};

impl Parser {
    pub fn is_class_declaration(&self) -> bool {
        self.token_buffer.is_identifier_of("class")
    }

    pub fn check_class_inner_declaration(&self, declaration: &Box<dyn Declaration>) -> Option<String> {
        if declaration.is::<ClassDeclaration>() {
            return Some("Inner classes are not allowed".to_string());
        }
        None
    }
    
    pub fn parse_class_declaration(
        &mut self,
        access_modifier: Option<AccessModifier>,
        member_modifier: Option<MemberModifier>,
        start: usize,
    ) -> BoxParseResult<dyn Declaration> {
        if member_modifier.iter().clone().any(|x| *x == MemberModifier::Native) {
            return Err(self.err("Class declaration cannot be native", None).into());
        }
        if access_modifier.iter().clone().any(|x| *x == AccessModifier::Private) {
            return Err(self.err("Class declaration cannot be private", None).into());
        }
        self.token_buffer.next(); // 跳过 'class'
        self.token_buffer.skip_whitespaces();
        let name = self
            .parse_identifier()?
            .downcast::<Identifier>()
            .unwrap()
            .clone();
        self.token_buffer.skip_whitespaces();
        let type_parameters = self.parse_type_parameters()?;
        self.token_buffer.skip_whitespaces();
        let mut super_class = None;
        let mut interfaces = Vec::new();
        if self.token_buffer.is_punctuation_of(':') {
            self.token_buffer.next(); // 跳过 ':'
            self.token_buffer.skip_whitespaces();
            super_class = Some(self.parse_named_type()?);
            self.token_buffer.skip_whitespaces();
            while self.token_buffer.is_punctuation_of(',') {
                self.token_buffer.next(); // 跳过 ','
                self.token_buffer.skip_whitespaces();
                let interface = self.parse_named_type()?;
                interfaces.push(interface);
            }
        }
        let mut body = Vec::new();
        if self.token_buffer.is_punctuation_of('{') {
            self.token_buffer.next(); // 跳过 '{'
            self.token_buffer.skip_whitespaces();
            while !self.token_buffer.is_punctuation_of('}') {
                let declaration = self.parse_annotated_declaration()?;
                body.push(declaration);
                self.token_buffer.skip_whitespaces();
            }
            for declaration in &body {
                if let Some(error) = self.check_class_inner_declaration(declaration) {
                    return Err(self.err(error.as_str(), None).into());
                }
            }
            self.token_buffer.next(); // 跳过 '}'
            Ok(Box::new(ClassDeclaration {
                access_modifier,
                member_modifier,
                name,
                type_parameters,
                super_class,
                interfaces,
                body,
                position: TokenRange::new(start, self.token_buffer.position),
            }))
        } else {
            Err(self.err("Expected '{' to start class body", None).into())
        }
    }
}
