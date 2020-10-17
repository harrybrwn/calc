use std::io::{self, Error, Write, ErrorKind};
use std::env;

use calc::parser::{parse, eval};

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
        match parse(s.as_str()) {
            Ok(ast) => println!("{}", eval(&ast)),
            Err(msg) => println!("Error: {}", msg),
        }
        s.clear()
    }
}

fn main() -> Result<(), Error> {
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
