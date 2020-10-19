use std::env;
use std::io::{self, Error, ErrorKind, Write};

use calc::exec;

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
        match exec(s.as_str()) {
            Ok(res) => println!("{}", res),
            Err(msg) => println!("Error: {}", msg),
        }
        s.clear()
    }
}

fn main() -> Result<(), Error> {
    let args = env::args();
    if args.len() == 1 {
        return interpreter();
    }
    let exp = args.last().unwrap();
    match exec(exp.as_str()) {
        Ok(res) => Ok(println!("{}", res)),
        Err(msg) => Err(Error::new(ErrorKind::Other, msg)),
    }
}
