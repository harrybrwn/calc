use std::env;

mod parse;
mod lex;

fn main() {
    println!("{:?}", env::args());
    println!("hello!");
}