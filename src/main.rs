mod parser;
mod lex;

use lex::*;
use std::env;

fn main() {
    let args = env::args();
    let exp = args.last().unwrap();
    let lexer = Lexer::new(exp.as_str());

    for tok in lexer {
        match tok {
            Token::End => break,
            Token::Invalid => panic!("invalid token"),
            _ => println!("{:?}", tok),
        }
    }
}
