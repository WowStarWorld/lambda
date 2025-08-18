use lambda_parser::node::declaration::{ClassDeclaration, Declaration, FunctionDeclaration, VariableDeclaration};
use lambda_parser::node::program::Program;

pub type VisitResult = Result<(), String>;

#[allow(unused_variables)]
pub trait Visitor {

    fn visit_program(&mut self, program: &Program) -> VisitResult { Ok(()) }

    fn visit_top_level_declaration(&mut self, declaration: &Box<dyn Declaration>) -> VisitResult { Ok(()) }
    fn visit_top_level_function_declaration(&mut self, function_declaration: &FunctionDeclaration) -> VisitResult { Ok(()) }
    fn visit_top_level_variable_declaration(&mut self, function_declaration: &VariableDeclaration) -> VisitResult { Ok(()) }

    fn visit_class_declaration(&mut self, class_declaration: &ClassDeclaration) -> VisitResult { Ok(()) }
    fn visit_function_declaration(&mut self, function_declaration: &FunctionDeclaration) -> VisitResult { Ok(()) }
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> VisitResult { Ok(()) }

    fn visit_statement(&mut self, statement: &Box<dyn lambda_parser::node::statement::Statement>) -> VisitResult { Ok(()) }
    fn visit_if_statement(&mut self, if_statement: &lambda_parser::node::statement::IfStatement) -> VisitResult { Ok(()) }
    fn visit_return_statement(&mut self, return_statement: &lambda_parser::node::statement::ReturnStatement) -> VisitResult { Ok(()) }
    fn visit_block_statement(&mut self, block_statement: &lambda_parser::node::statement::BlockStatement) -> VisitResult { Ok(()) }
    fn visit_expression_statement(&mut self, expression_statement: &lambda_parser::node::statement::ExpressionStatement) -> VisitResult { Ok(()) }
    fn visit_declaration_statement(&mut self, declaration_statement: &lambda_parser::node::statement::DeclarationStatement) -> VisitResult { Ok(()) }

    fn visit_expression(&mut self, expression: &Box<dyn lambda_parser::node::expression::Expression>) -> VisitResult { Ok(()) }
    fn visit_identifier(&mut self, identifier: &lambda_parser::node::expression::Identifier) -> VisitResult { Ok(()) }
    fn visit_literal(&mut self, literal: &lambda_parser::node::expression::Literal) -> VisitResult { Ok(()) }
    fn visit_binary_expression(&mut self, binary_expression: &lambda_parser::node::expression::BinaryExpression) -> VisitResult { Ok(()) }
    fn visit_unary_expression(&mut self, unary_expression: &lambda_parser::node::expression::UnaryExpression) -> VisitResult { Ok(()) }

}
