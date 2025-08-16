use std::io::Write;
use lambda_parser::parser::api::TokenBuffer;
use lambda_parser::parser::statement::parse_statement;
use lambda_parser::tokenizer::tokenizer::{SrcInfo, Tokenizer};

fn main() {
    println!("Welcome to the Lambda REPL!");
    println!("Type ':exit' to quit.");
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == ":exit" {
            break;
        }

        let src_info = SrcInfo {
            filename: "<stdin>".to_string()
        };
        let tokenizer = Tokenizer::new(&*input, src_info);
        let mut token_buffer = TokenBuffer::new(tokenizer);
        match parse_statement(&mut token_buffer) {
            Ok(statement) => println!("{:#?}", statement),
            Err(e) => eprintln!("{e}")
        }
    }
}
