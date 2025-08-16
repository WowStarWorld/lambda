pub mod tokenizer;
pub mod parser;
pub mod node;

#[cfg(test)]
mod tests {
    use crate::parser::api::TokenBuffer;
    use crate::parser::program::parse_program;
    use super::tokenizer::tokenizer::{SrcInfo, Tokenizer};

    #[test]
    fn main_declaration() {
        let src_info = SrcInfo {
            filename: "<test>".to_string()
        };
        let tokenizer = Tokenizer::new(
            r#"
            func test(...args: Array<String>) { TODO("Not yet implemented"); }
            func main(args: Array<String>) -> Int {
                test(...args);
            }
            "#,
            src_info
        );
        let mut token_buffer = TokenBuffer::new(tokenizer);
        match parse_program(&mut token_buffer) {
            Ok(_) => {},
            Err(err) => {
                panic!("{}", err);
            }
        }
    }


}