use std::io::Write;
use lambda_parser::parser::api::{Parser};
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
        let mut parser = Parser::new(tokenizer);
        match parser.parse_statement() {
            Ok(statement) => println!("{:#?}", statement),
            Err(e) => eprintln!("{e}")
        }
    }
}
