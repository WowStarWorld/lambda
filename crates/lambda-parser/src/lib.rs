pub mod node;
pub mod parser;
pub mod tokenizer;

#[cfg(test)]
mod tests {
    use super::tokenizer::tokenizer::{SrcInfo, Tokenizer};
    use crate::parser::api::Parser;

    #[test]
    fn main_declaration() {
        let src_info = SrcInfo {
            filename: "<test>".to_string(),
        };
        let tokenizer = Tokenizer::new(
            r#"
            func test(...args: Array<String>) { TODO("Not yet implemented"); }
            func main(args: Array<String>) -> Int {
                test(...args);
            }
            "#,
            src_info,
        );
        let mut parser = Parser::new(tokenizer);
        match parser.parse_program() {
            Ok(_) => {}
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}
