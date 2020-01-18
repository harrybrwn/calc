#![allow(dead_code)]

use std::iter::Peekable;
use std::str::Chars;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Invalid,
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Op::Add => write!(f, "Op::Add(+)"),
            Op::Sub => write!(f, "Op::Sub(-)"),
            Op::Mul => write!(f, "Op::Mul(*)"),
            Op::Div => write!(f, "Op::Div(/)"),
            Op::Invalid => write!(f, "Op::Invalid"),
        }
    }
}

fn get_op(c: char) -> Op {
    match c {
        '+' => Op::Add,
        '-' => Op::Sub,
        '*' => Op::Mul,
        '/' => Op::Div,
        _ => Op::Invalid,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Op(char),
    Int(i64),
    Float(f64),
    End,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    toks: Vec<Token>,
    pos: usize,
}

impl<'a> Lexer<'a> {

    pub fn new(text: &'a str) -> Self {
        Self{
            chars: text.chars().peekable(),
            toks: lex(text),
            pos: 0usize,
        }
    }

    pub fn peek(&mut self) -> Token {
        self.look_ahead(0)
    }

    pub fn look_ahead(&mut self, n: usize) -> Token {
        let mut chars = self.chars.clone();
        for _ in 0..n {
            next_token(&mut chars);
        }
        next_token(&mut chars).unwrap_or(Token::End)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        next_token(&mut self.chars)
    }
}

fn next_token(chars: &mut Peekable<Chars>) -> Option<Token> {
    let c = eat_spaces(chars);

    let res = match c {
        '\0' => None,
        '(' => Some(Token::OpenParen),
        ')' => Some(Token::CloseParen),
        '0'...'9' | '.' => return Some(lex_num(chars)),
        '-' | '+' | '*' | '/' | '^' => Some(Token::Op(c)),
        _ => None,
    };
    chars.next();
    res
}

fn eat_spaces(chars: &mut Peekable<Chars>) -> char {
    loop {
        let c = *chars.peek().unwrap_or(&'\0');
        if c != ' ' {
            break c;
        }
        chars.next();
    }
}

fn lex_num<'a>(chars: &mut Peekable<Chars<'a>>) -> Token {
    let mut s = String::with_capacity(6);
    let mut isfloat = false;

    loop {
        let c = *chars.peek().unwrap_or(&'\0');
        if c == '.' {
            isfloat = true;
        } else if c < '0' || c > '9'{
            break
        }
        s.push(c);
        chars.next();
    }

    if isfloat {
        Token::Float(s.parse::<f64>().unwrap())
    } else {
        Token::Int(s.parse::<i64>().unwrap())
    }
}

pub fn lex(s: &str) -> Vec<Token> {
    let mut toks = vec![];
    let mut chars = s.chars().peekable();

    loop {
        let next = next_token(&mut chars);
        match next {
            Some(tok) => toks.push(tok),
            None => break toks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let s = "5 + 335 * (1.5+1)";
        let res = lex(s);
        match res[0] {
            Token::Int(x) => assert_eq!(x, 5),
            _ => panic!("should be a int token"),
        }
        match res[1] {
            Token::Op(x) => assert_eq!(x, '+'),
            _ => panic!("should be an op token"),
        }
        match res[2] {
            Token::Int(x) => assert_eq!(x, 335),
            _ => panic!("should be a int token"),
        }
        match res[3] {
            Token::Op(x) => assert_eq!(x, '*'),
            _ => panic!("should be an op"),
        }
        match res[4] {
            Token::OpenParen => assert!(true),
            _ => assert!(false),
        }
        match res[5] {
            Token::Float(x) => assert_eq!(x, 1.5f64),
            _ => panic!("should be float"),
        }
        match res[6] {
            Token::Op(c) => assert_eq!(c, '+'),
            Token::Int(x) => panic!("should not be number: {}", x),
            _ => panic!("should be op"),
        }
        match res[8] {
            Token::CloseParen => {},
            _ => panic!("should be closed paren"),
        }

        let mut l = Lexer::new(s);
        loop {
            let p = l.peek().clone();
            let n = l.next().unwrap_or(Token::End);

            // println!("{:?} {:?}", p, n);
            match (p, n) {
                (Token::Int(a), Token::Int(b))     => assert_eq!(a, b),
                (Token::Float(a), Token::Float(b)) => assert_eq!(a, b),
                (Token::Op(a), Token::Op(b))       => assert_eq!(a, b),
                (Token::OpenParen, Token::OpenParen) => (),
                (Token::CloseParen, Token::CloseParen) => (),
                (Token::End, Token::End)           => break,
                _ => panic!("tokens should be the same"),
            }
        }
    }

    #[test]
    fn test_iter() {
        let mut l = Lexer::new("1+1");
        let p = l.peek().clone();
        assert_eq!(p, Token::Int(1));
        assert_eq!(l.look_ahead(0), p);
        assert_eq!(l.look_ahead(1), Token::Op('+'));
        assert_eq!(l.look_ahead(2), Token::Int(1));

        match l.peek().clone() {
            Token::Int(n) => assert_eq!(n, 1),
            _ => panic!("expected the number one"),
        }
        l.next();
        match l.peek().clone() {
            Token::Op(c) => assert_eq!(c, '+'),
            _ => panic!("expected '+'"),
        }
        l.next();
        match l.peek().clone() {
            Token::Int(n) => assert_eq!(n, 1),
            _ => panic!("expected number one"),
        }
    }

    #[test]
    fn test_both_lexers() {
        let s = "1 + (3 / 2) * 4";
        let mut toks1 = vec![];

        for t in Lexer::new(s) {
            toks1.push(t);
        }
        let toks2 = lex(s);

        assert_eq!(toks1, toks2);
    }

    #[test]
    fn test_bad_source() {
        // let l = Lexer::new("hello");
    }
}
