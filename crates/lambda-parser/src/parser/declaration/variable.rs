use crate::node::declaration::{AccessModifier, Declaration, MemberModifier, VariableDeclaration};
use crate::node::expression::Identifier;
use crate::node::node::TokenRange;
use crate::parser::api::{BoxParseResult, Parser};

impl Parser {

    pub fn is_variable_declaration(&self) -> bool {
        self.token_buffer.is_identifier_of("val")
            || self.token_buffer.is_identifier_of("var")
    }

    pub fn parse_variable_declaration(
        &mut self,
        access_modifier: Option<AccessModifier>,
        member_modifier: Option<MemberModifier>,
        start: usize
    ) -> BoxParseResult<dyn Declaration> {
        let has_value = member_modifier
            .iter().clone()
            .all(|x| *x != MemberModifier::Native && *x != MemberModifier::Abstract);
        let mutable = self.token_buffer.next()
            .unwrap().get_raw() == "var"; // 跳过 'var' 或 'val'
        self.token_buffer.skip_whitespaces();
        let type_parameters = self.parse_type_parameters()?;
        self.token_buffer.skip_whitespaces();
        let name = self
            .parse_identifier()?
            .downcast::<Identifier>()
            .unwrap()
            .clone();
        self.token_buffer.skip_whitespaces();
        let value_type = if self.token_buffer.is_punctuation_of(':') {
            self.token_buffer.next(); // 跳过 ':'
            self.token_buffer.skip_whitespaces();
            Some(self.parse_type()?)
        } else {
            self.token_buffer.skip_whitespaces();
            None
        };
        let default_value = if self.token_buffer.is_punctuation_of('=') {
            self.token_buffer.next(); // 跳过 '='
            self.token_buffer.skip_whitespaces();
            Some(self.parse_expression()?)
        } else {
            self.token_buffer.skip_whitespaces();
            None
        };
        if !has_value && default_value.is_some() {
            return Err(self.err(format!("Variable declaration with modifier '{:?}' cannot have a default value", member_modifier).as_str(), None).into());
        }
        let delegate = if self.token_buffer.is_identifier_of("by") {
            self.token_buffer.next(); // 跳过 'by'
            self.token_buffer.skip_whitespaces();
            Some(self.parse_expression()?)
        } else {
            None
        };
        if !has_value && delegate.is_some() {
            return Err(self.err(format!("Variable declaration with modifier '{:?}' cannot have a delegate", member_modifier).as_str(), None).into());
        }
        if default_value.is_some() && delegate.is_some() {
            return Err(self.err("Variable declaration cannot have both default value and delegate", None).into());
        }
        let getter = if self.token_buffer.is_identifier_of("get") {
            self.token_buffer.next(); // 跳过 'get'
            self.token_buffer.skip_whitespaces();
            if !self.is_block_statement() {
                return Err(self.err("Expected block statement after 'get()'", None).into());
            }
            Some(self.parse_block_statement()?)
        } else {
            None
        };
        self.token_buffer.skip_whitespaces();
        let setter = if self.token_buffer.is_identifier_of("set") {
            self.token_buffer.next(); // 跳过 'set'
            self.token_buffer.skip_whitespaces();
            if !self.token_buffer.is_punctuation_of('(') {
                return Err(self.err("Expected '(' after 'set'", None).into());
            }
            self.token_buffer.next(); // 跳过 '('
            self.token_buffer.skip_whitespaces();
            if !self.token_buffer.is_identifier() {
                return Err(self.err("Expected identifier as parameter name in 'set()'", None).into());
            }
            let parameter = self.token_buffer.next().unwrap();
            if !self.token_buffer.is_punctuation_of(')') {
                return Err(self.err("Expected ')' after 'set('", None).into());
            }
            self.token_buffer.next(); // 跳过 ')'
            self.token_buffer.skip_whitespaces();
            if !self.is_block_statement() {
                return Err(self.err("Expected block statement after 'set()'", None).into());
            }
            Some((parameter, self.parse_block_statement()?))
        } else {
            None
        };
        if !has_value && (getter.is_some() || setter.is_some()) {
            return Err(self.err(format!("Variable declaration with modifier '{:?}' cannot have a getter or setter", member_modifier).as_str(), None).into());
        }
        if delegate.is_some() && (getter.is_some() || setter.is_some()) {
            return Err(self.err("Variable declaration with delegate cannot have a getter or setter", None).into());
        }
        if !mutable && setter.is_some() {
            return Err(self.err("Variable declaration without 'var' cannot have a setter", None).into());
        }
        if !self.token_buffer.is_line_break() {
            return Err(self.err("Expected line-break after variable declaration", None).into());
        }
        self.token_buffer.skip_line_break();
        Ok(Box::new(VariableDeclaration {
            mutable,
            access_modifier,
            member_modifier,
            name,
            type_parameters,
            parameters: vec![],
            default_value,
            value_type,
            getter, setter, delegate,
            position: TokenRange::new(start, self.token_buffer.position),
        }))
    }

}