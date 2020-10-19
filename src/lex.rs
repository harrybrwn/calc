#![allow(dead_code)]

use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    Op(char),
    Int(i64),
    Float(f64),

    // TODO: add a counter to the paren tokens
    // i.e. 'OpenParen(i32)' so when I need to
    // match them up I can just match them by
    // their internal counter
    //
    // I would need the `next_token` function to be
    // a lot smarter (maybe just get rid of it and put
    // it on the Lexer).
    OpenParen,
    CloseParen,

    Modulus, // TODO: implement modulus as a keywork 4 mod 3
    Func,   // TODO: add functions
    Assign, // TODO: add assignment support ("let name = ...")
    Equal,  // TODO: a single equal sign like real math f = 2*x

    End,
    Invalid,
}

#[derive(Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Invalid,
}

pub fn lex(s: &str) -> Vec<Token> {
    let mut toks = vec![];
    let mut chars = s.chars().peekable();
    loop {
        let next = next_token(&mut chars);
        match next {
            Token::End | Token::Invalid => break toks,
            _ => toks.push(next),
        }
    }
}

fn next_token<'b>(chars: &'b mut Peekable<Chars>) -> Token {
    let tok = match eat_spaces(chars) {
        Some(c) => match c {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '0'..='9' | '.' => return lex_num(chars),
            '-' | '+' | '*' | '/' | '^' => Token::Op(c),
            // '!' => Token::Invalid, // TODO: factorial
            'a'..='z' => {
                let key = "mod".chars();
                for ch in key {
                    if let Some(&peeked) = chars.peek() {
                        if ch != peeked {
                            return Token::Invalid;
                        }
                        chars.next();
                    }
                }
                Token::Modulus
            },
            _ => return Token::Invalid,
        },
        None => return Token::Invalid,
    };
    chars.next();
    tok
}
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    buf: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            chars: text.chars().peekable(),
            buf: vec![],
        }
    }

    pub fn from(toks: Vec<Token>) -> Self {
        return Self {
            chars: "".chars().peekable(),
            buf: toks,
        };
    }

    pub fn peek(&mut self) -> Token {
        if self.buf.len() == 0 {
            self.buf.push(next_token(&mut self.chars));
        }
        self.buf[0]
    }

    pub fn look_ahead(&mut self, n: usize) -> Token {
        let len = self.buf.len();
        if len > 0 && len > n {
            return self.buf[n];
        }

        let mut next: Token;
        for _ in len..n {
            next = next_token(&mut self.chars);
            if next == Token::Invalid {
                return Token::End;
            }
            self.buf.push(next);
            if next == Token::End {
                return next;
            }
        }
        next = next_token(&mut self.chars);
        self.buf.push(next);
        next
    }

    /// Discard n tokens from the tokenizer.
    pub fn discard(&mut self, n: usize) {
        let len = self.buf.len();

        if len < n {
            self.buf.drain(0..len);
            for _ in len..n {
                next_token(&mut self.chars);
            }
        } else if len > n {
            self.buf.drain(0..n);
        }
    }

    pub fn as_vec(&self) -> Vec<Token> {
        let mut chars = self.chars.clone();

        let mut v = vec![];
        v.extend(&self.buf);
        loop {
            let t = next_token(&mut chars);
            if t == Token::Invalid {
                break v;
            }
            v.push(t);
            if t == Token::End {
                break v;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        if !self.buf.is_empty() {
            false
        } else {
            match self.clone().next() {
                Some(t) => t == Token::End,
                None => true,
            }
        }
    }

    /// Capture a group within parenthesis tokens
    pub fn capture_group(&mut self) -> Result<Lexer, String> {
        let mut toks = self.clone();
        match toks.next() {
            Some(t) => match t {
                Token::OpenParen => {}
                _ => return Err(format!("expected '('")),
            },
            None => return Err(format!("stream ended early")),
        }
        self.next(); // skip the open paren

        let mut expr = vec![];
        let mut paren = 0;
        for t in toks {
            expr.push(t);
            match t {
                Token::CloseParen => {
                    if paren == 0 {
                        break;
                    } else {
                        paren -= 1;
                    }
                }
                Token::OpenParen => paren += 1,
                _ => {}
            }
        }
        let size = expr.len() - 1;
        if size == 0 {
            return Err(format!("empty parenthesis expression"));
        }
        self.discard(size);

        // check for the closing parenthesis in the expression
        if expr.pop().unwrap() != Token::CloseParen {
            return Err(format!("expected ')'"));
        }
        // check for a closing parenthesis in the main token stream
        match self.next() {
            Some(tok) => match tok {
                Token::CloseParen => {},
                _ => return Err(format!("expected ')'")),
            }
            None => {}
        }

        Ok(Lexer::from(expr))
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() > 0 {
            return Some(self.buf.remove(0));
        }
        let tok = next_token(&mut self.chars);
        match tok {
            Token::End | Token::Invalid => None,
            _ => Some(tok),
        }
    }
}

impl Clone for Lexer<'_> {
    fn clone(&self) -> Self {
        Self {
            buf: self.buf.clone(),
            chars: self.chars.clone(),
        }
    }
}

impl<'a> fmt::Display for Lexer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.buf.len() == 0 {
            write!(f, "Lexer{{..}}")
        } else {
            write!(f, "Lexer{{{:?}}}", self.buf)
        }
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Op::Add => write!(f, "Op::Add(+)"),
            Op::Sub => write!(f, "Op::Sub(-)"),
            Op::Mul => write!(f, "Op::Mul(*)"),
            Op::Div => write!(f, "Op::Div(/)"),
            Op::Exp => write!(f, "Op::Exp(^)"),
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
        '^' => Op::Exp,

        // these are not supported yet
        '!' => Op::Invalid,
        _ => Op::Invalid,
    }
}

impl Op {
    fn as_char(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
            Op::Exp => '^',
            Op::Invalid => '0',
        }
    }

    fn presidence() -> i8 {
        0
    }
}

fn eat_spaces<'a>(chars: &'a mut Peekable<Chars>) -> Option<char> {
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' => {}
            _ => return Some(c),
        }
        chars.next();
    }
    None
}

fn lex_num(chars: &mut Peekable<Chars>) -> Token {
    let mut s = String::with_capacity(16);
    let mut isfloat = false;

    while let Some(&c) = chars.peek() {
        if c == '.' {
            isfloat = true;
        } else if !c.is_digit(10) {
            break;
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


#[cfg(test)]
mod test {
    use super::{eat_spaces, lex, lex_num, Lexer, Token};

    #[test]
    fn test_lex_num() {
        let mut ch = "123".chars().peekable();
        let res = match lex_num(&mut ch) {
            Token::Int(i) => i,
            Token::Invalid => panic!("should not get invalid token for \"123\""),
            _ => 0,
        };
        assert_eq!(res, 123);
    }

    #[test]
    fn test_eat_spaces() {
        let mut chars = "    a".chars().peekable();
        match eat_spaces(&mut chars) {
            Some(c) => assert_eq!(c, 'a'),
            None => panic!("expected 'a'"),
        }
        let mut ch = "a".chars().peekable();
        assert_eq!('a', eat_spaces(&mut ch).unwrap());
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
    fn test_look_ahead() {
        let s = "1+2";
        let mut l = Lexer::new(s);
        assert_eq!(Token::End, l.look_ahead(5));
        assert_eq!(Token::End, l.look_ahead(10));
    }

    #[test]
    fn test_lex() {
        let s = "5 + 335 * (1.5+1)";
        let expected = vec![
            Token::Int(5),
            Token::Op('+'),
            Token::Int(335),
            Token::Op('*'),
            Token::OpenParen,
            Token::Float(1.5),
            Token::Op('+'),
            Token::Int(1),
            Token::CloseParen,
        ];
        let res = lex(s);
        assert!(res.len() > 1);
        assert_eq!(res.len(), expected.len());

        for i in 0..expected.len() - 1 {
            assert_eq!(expected[i], res[i]);
        }

        let mut l = Lexer::new(s);
        loop {
            let p = l.peek().clone();
            let n = l.next().unwrap_or(Token::End);

            match (p, n) {
                (Token::Int(a), Token::Int(b)) => assert_eq!(a, b),
                (Token::Float(a), Token::Float(b)) => assert_eq!(a, b),
                (Token::Op(a), Token::Op(b)) => assert_eq!(a, b),
                (Token::OpenParen, Token::OpenParen) => (),
                (Token::CloseParen, Token::CloseParen) => (),
                (Token::End, Token::End) => break,
                (Token::Invalid, Token::Invalid) => break,
                _ => panic!("tokens should be the same"),
            }
        }
    }

    #[test]
    fn test_lex_iter() {
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
        let l = Lexer::new("1 + 1 + 1");
        for t in l {
            match t {
                Token::Int(1) | Token::Op('+') | Token::End => {}
                Token::Invalid => {
                    println!("invalid");
                    break;
                }
                _ => panic!("got wrong token, {:?}", t),
            }
        }
    }
}
