use crate::node::declaration::{ClassDeclaration, Declaration, FunctionDeclaration, MemberModifier, VariableDeclaration};
use crate::node::node::TokenRange;
use crate::node::program::{ImportDefinition, PackageDefinition, Program};
use crate::parser::api::{BoxParseResult, ParseResult, Parser};

impl Parser {
    pub fn check_top_level_declaration(&self, declaration: &Box<dyn Declaration>) -> Option<String> {
        if declaration.is::<FunctionDeclaration>() {
            if let Some(modifier) = &declaration.downcast::<FunctionDeclaration>().unwrap().member_modifier {
                if *modifier != MemberModifier::Native {
                    return Some(format!("Top-level functions should not have member modifier: '{:?}'", modifier));
                }
            }
            return None
        }
        if declaration.is::<VariableDeclaration>() {
            if let Some(modifier) = &declaration.downcast::<VariableDeclaration>().unwrap().member_modifier {
                if *modifier != MemberModifier::Native {
                    return Some(format!("Top-level variables should not have member modifier: '{:?}'", modifier));
                }
            }
            return None;
        }
        if declaration.is::<ClassDeclaration>() {
            return None;
        }
        Some("Not a valid top-level declaration".to_string())
    }

    pub fn parse_top_level_declaration(&mut self) -> BoxParseResult<dyn Declaration> {
        let result = if self.is_annotated_declaration() {
            self.parse_annotated_declaration()
        } else {
            Err(self.err("Expected a top-level declaration", None).into())
        };
        match result {
            Ok(declaration) => {
                if let Some(err) = self.check_top_level_declaration(&declaration) {
                    Err(self.err(err.as_str(), None).into())
                } else {
                    Ok(declaration)
                }
            }
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
        let name = self.parse_identifier_qualified()?;
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_line_break() {
            return Err(self
                .err("Expected line-break after package definition", None)
                .into());
        }
        self.token_buffer.skip_line_break(); // 跳过 ';'
        let package_name = if name.0.is_some() {
            format!("{}.{}", name.0.unwrap(), name.1)
        } else {
            name.1
        };
        Ok(PackageDefinition { name: package_name })
    }

    pub fn is_import_definition(&self) -> bool {
        self.token_buffer.is_identifier_of("import")
    }

    pub fn parse_import_definition(&mut self) -> ParseResult<ImportDefinition> {
        self.token_buffer.next();
        self.token_buffer.skip_whitespaces();
        let qualified = self.parse_identifier_qualified()?;
        if qualified.0.is_none() {
            return Err(self.err("Expected a package name in import definition", None).into());
        }
        self.token_buffer.skip_whitespaces();
        if !self.token_buffer.is_line_break() {
            return Err(self.err("Expected line-break after import definition", None).into());
        }
        self.token_buffer.skip_line_break();
        Ok(ImportDefinition {
            package_name: qualified.0.unwrap(),
            member: qualified.1,
        })
    }

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let start = self.token_buffer.position;
        let package_definition = self.parse_package_definition()?;
        self.token_buffer.skip_whitespaces();
        let mut import_definitions = Vec::new();
        while self.is_import_definition() {
            import_definitions.push(self.parse_import_definition()?);
            self.token_buffer.skip_whitespaces();
        }
        let mut declarations = Vec::new();
        self.token_buffer.skip_whitespaces();
        while self.token_buffer.has_next() {
            if self.is_annotated_declaration() {
                declarations.push(self.parse_top_level_declaration()?);
            } else {
                return Err(self.err("Unexpected token in program", None).into());
            }
            self.token_buffer.skip_whitespaces();
        }
        Ok(Program {
            package_definition,
            import_definitions,
            declarations,
            position: TokenRange::new(start, self.token_buffer.position),
        })
    }
}
