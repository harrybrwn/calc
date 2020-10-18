use std::env;
use std::io::{self, Error, ErrorKind, Write};

use calc;

fn interpreter() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut s = String::new();
    loop {
        print!(">>> ");

        stdout.flush()?;
        stdin.read_line(&mut s)?;

        if s.as_bytes()[0] as char == 'q' || s == "q" || s == "quit" || s == "exit" {
            return Ok(());
        }
        match calc::parser::parse(s.as_str()) {
            Ok(ast) => println!("{}", calc::ast::eval(&ast)),
            Err(msg) => println!("Error: {}", msg),
        }
        s.clear()
    }
}

fn main() -> Result<(), Error> {
    let args = env::args();

    if args.len() >= 2 {
        let exp = args.last().unwrap();
        match calc::parser::parse(exp.as_str()) {
            Ok(ast) => Ok(println!("{}", calc::ast::eval(&ast))),
            Err(msg) => Err(Error::new(ErrorKind::Other, msg)),
        }
    } else {
        interpreter()
    }
}
