use crate::node::declaration::{Declaration, FunctionDeclaration};
use crate::node::program::{PackageDefinition, Program};
use crate::parser::api::{BoxParseResult, ParseResult, Parser};

impl Parser {
    pub fn is_top_level_declaration(&self, declaration: &Box<dyn Declaration>) -> bool {
        declaration.is::<FunctionDeclaration>()
    }

    pub fn parse_top_level_declaration(&mut self) -> BoxParseResult<dyn Declaration> {
        let result = if self.is_annotated() {
            self.parse_annotated()
        } else {
            Err(self.err("Expected a top-level declaration", None).into())
        };
        match result {
            Ok(declaration) => {
                if self.is_top_level_declaration(&declaration) {
                    Ok(declaration)
                } else {
                    Err(self.err("Invalid top-level declaration", None).into())
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn parse_package_definition(&mut self) -> ParseResult<PackageDefinition> {
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.has_next() {
            return Err(self.err("Expected package definition", None).into());
        }
        if !self.token_buffer.is_identifier_of("package") {
            return Err(self.err("Expected 'package' keyword", None).into());
        }
        self.token_buffer.next(); // 跳过 'package'
        self.token_buffer.skip_whitespaces();
        let name = self.parse_identifier_list()?;
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_punctuation_of(';') {
            return Err(self.err("Expected ';' after package definition", None).into());
        }
        self.token_buffer.next(); // 跳过 ';'
        Ok(PackageDefinition { name })
    }

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let package_definition = self.parse_package_definition()?;
        let mut declarations = Vec::new();
        self.token_buffer.skip_whitespaces();
        while self.token_buffer.has_next() {
            if self.is_annotated() {
                declarations.push(self.parse_top_level_declaration()?);
            } else {
                return Err(self.err("Unexpected token in program", None).into());
            }
            self.token_buffer.skip_whitespaces();
        }
        Ok(
            Program { package_definition, declarations }
        )
    }
}
