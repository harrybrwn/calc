use std::env;

mod parser;
mod lex;

use lex::Token;

fn main() {
    let args = env::args();
    let exp = args.last().unwrap();
    let mut lexer = lex::Lexer::new(exp.as_str());

    println!("{}", parser::parse_factor(&mut lexer));

    loop {
        let n = lexer.peek();
        match n {
            Token::End => break,
            Token::Invalid => panic!("invalid token"),
            _ => {
                println!("{:?}", n);
                lexer.skip();
            },
        }
    }
}