use std::io::{self, Error, Write, ErrorKind};
use std::env;

pub mod parser;
pub mod lex;
use lex::Token;
use parser::{Ast, parse};


fn eval(ast: &Ast) -> f64 {
    match ast.tok {
        Token::Op(c) => match c {
            '+' => eval(&ast.children[0]) + eval(&ast.children[1]),
            '-' => eval(&ast.children[0]) - eval(&ast.children[1]),
            '*' => eval(&ast.children[0]) * eval(&ast.children[1]),
            '/' => {
                eval(&ast.children[0]) as f64 / eval(&ast.children[1]) as f64
            },
            _ => panic!("invalid op"),
        },
        Token::Int(n) => n as f64,
        Token::Float(n) => n,
        _ => 0.0,
    }
}

fn interpreter() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut s = String::new();
        print!(">>> ");

        stdout.flush()?;
        stdin.read_line(&mut s)?;

        if s.as_bytes()[0] as char == 'q' {
            return Ok(());
        }
        match parse(s.as_str()) {
            Ok(ast) => println!("{}", eval(&ast)),
            Err(msg) => println!("Error: {}", msg),
        }
    }
}

fn main() -> Result<(), Error>{
    let args = env::args();

    if args.len() >= 2 {
        let exp = args.last().unwrap();
        match parse(exp.as_str()) {
            Ok(ast) => Ok(println!("{}", eval(&ast))),
            Err(msg) => Err(Error::new(ErrorKind::Other, msg)),
        }
    } else {
        interpreter()
    }
}
