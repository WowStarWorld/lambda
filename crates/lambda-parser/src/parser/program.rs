use crate::node::declaration::Declaration;
use crate::parser::api::{BoxParseResult, ParseResult, Parser};

impl Parser {
    pub fn is_top_level_declaration(&self) -> bool {
        self.is_function_declaration()
    }

    pub fn parse_top_level_declaration(&mut self) -> BoxParseResult<dyn Declaration> {
        if self.is_function_declaration() {
            self.parse_function_declaration()
        } else {
            Err(self.err("Expected a top-level declaration", None).into())
        }
    }

    pub fn parse_program(&mut self) -> ParseResult<Vec<Box<dyn Declaration>>> {
        let mut declarations = Vec::new();
        self.token_buffer.skip_whitespaces();
        while self.token_buffer.has_next() {
            if self.is_top_level_declaration() {
                declarations.push(self.parse_top_level_declaration()?);
            } else {
                return Err(self.err("Unexpected token in program", None).into());
            }
            self.token_buffer.skip_whitespaces();
        }
        Ok(declarations)
    }
}
