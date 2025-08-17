use lambda_parser::node::declaration::{Declaration, FunctionDeclaration};
use lambda_parser::node::program::Program;

#[allow(unused_variables)]
pub trait Visitor {

    fn visit_program(&mut self, program: &Program) {}

    fn visit_top_level_declaration(&mut self, declaration: &Box<dyn Declaration>) {}
    fn visit_top_level_function_declaration(&mut self, function_declaration: &FunctionDeclaration) {}
    fn visit_function_declaration(&mut self, function_declaration: &FunctionDeclaration) {}

}
