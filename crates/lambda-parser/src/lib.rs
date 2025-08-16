pub mod node;
pub mod parser;
pub mod tokenizer;

#[cfg(test)]
mod test {
    use crate::parser::api::Parser;
    use crate::tokenizer::tokenizer::{SrcInfo, Tokenizer};

    #[test]
    fn it_works() {
        let src = r#"
        package test;

        fn min(a: T, b: T) -> T = if (a > b) b else a;

        fn <T : Comparable> max(a: T, b: T) -> T {
            if (a > b) {
                return a;
            } else {
                return b;
            }
        }

        fn <T : Comparable> isEqual(a: T, b: T) -> Boolean {
            return !(a > b || a < b);
        }
        "#;
        let src_info = SrcInfo { filename: "test.ld".to_string() };
        let program = Parser::new(Tokenizer::new(src, src_info)).parse_program();
        match program {
            Ok(program) => {
                println!("{:#?}", program);
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
