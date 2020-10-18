#![allow(dead_code)]

use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

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

        // these are not supported yet
        '^' => Op::Invalid,
        '!' => Op::Invalid,
        _ => Op::Invalid,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
        if self.buf.len() > 0 {
            self.buf[0]
        } else {
            let next = next_token(&mut self.chars);
            self.buf.push(next);
            next
        }
    }

    pub fn look_ahead(&mut self, n: usize) -> Token {
        let len = self.buf.len();
        if len > 0 && len > n {
            return self.buf[n];
        }

        let mut next: Token;
        for _ in len..n {
            next = next_token(&mut self.chars);
            self.buf.push(next);

            if next == Token::End {
                return next;
            }
        }
        next = next_token(&mut self.chars);
        self.buf.push(next);
        next
    }

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

    pub fn copy_until_one_of(&self, toks: &[Token]) -> Vec<Token> {
        let mut chars = self.chars.clone();
        let mut v = vec![];
        loop {
            let t = next_token(&mut chars);
            if t == Token::End {
                break v;
            }
            for tok in toks {
                if t == *tok {
                    return v;
                }
            }
            v.push(t);
        }
    }

    pub fn copy_until(&self, tok: Token) -> Vec<Token> {
        let mut chars = self.chars.clone();
        let mut v = vec![];
        loop {
            let t = next_token(&mut chars);
            if t == tok || t == Token::End {
                break v;
            }
            v.push(t);
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() > 0 {
            return Some(self.buf.remove(0));
        }
        let tok = next_token(&mut self.chars);
        match tok {
            Token::End => None,
            _ => Some(tok),
        }
    }
}

impl<'a> fmt::Display for Lexer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer")
    }
}

fn next_token(chars: &mut Peekable<Chars>) -> Token {
    let c = eat_spaces(chars);

    let res = match c {
        '(' => Token::OpenParen,
        ')' => Token::CloseParen,
        '0'..='9' | '.' => return lex_num(chars),
        '-' | '+' | '*' | '/' | '^' => Token::Op(c),
        '\0' => Token::End,
        _ => Token::End,
    };
    chars.next();
    res
}

fn eat_spaces(chars: &mut Peekable<Chars>) -> char {
    loop {
        let c = *chars.peek().unwrap_or(&'\0');
        if c != ' ' && c != '\n' && c != '\t' {
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
        } else if c < '0' || c > '9' {
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

pub fn lex(s: &str) -> Vec<Token> {
    let mut toks = vec![];
    let mut chars = s.chars().peekable();

    loop {
        let next = next_token(&mut chars);
        match next {
            Token::End => break toks,
            _ => toks.push(next),
        }
    }
}
