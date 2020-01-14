use std::env;

mod parse;
mod lex;

use lex::Token;

fn main() {
    let args = env::args();
    let exp = args.last().unwrap();
    let mut lexer = lex::Lexer::new(exp.as_str());

    loop {
        let n = lexer.next();
        match n {
            Token::End => break,
            Token::Invalid => panic!("invalid token"),
            _ => {
                println!("{:?}", n);
                // lexer.skip();
            },
        }
    }
}