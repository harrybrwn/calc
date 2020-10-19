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
    Invalid,
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
            self.buf.push(next);
            match next {
                Token::End | Token::Invalid => return Token::End,
                _ => {}
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
            v.push(t);
            if t == Token::End || t == Token::Invalid {
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
                None => false,
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
        if expr.pop().unwrap() != Token::CloseParen {
            return Err(format!("expected ')'"));
        }
        self.discard(size);
        Ok(Lexer::from(expr))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() > 0 {
            return Some(self.buf.remove(0));
        }
        let tok = next_token(&mut self.chars);
        // Some(tok)

        match tok {
            Token::Invalid => None,
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

fn next_token<'b>(chars: &'b mut Peekable<Chars>) -> Token {
    let tok = match eat_spaces(chars) {
        Some(c) => match c {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '0'..='9' | '.' => return lex_num(chars),
            '-' | '+' | '*' | '/' | '^' => Token::Op(c),
            '\0' => Token::End,
            _ => Token::Invalid,
        },
        None => return Token::Invalid,
    };
    chars.next();
    tok
}

fn eat_spaces<'a>(chars: &'a mut Peekable<Chars>) -> Option<char> {
    loop {
        match chars.peek() {
            Some(c) if *c == '\0' => break Some(*c),
            Some(c) if *c != ' ' && *c != '\n' && *c != '\t' => break Some(*c),
            Some(..) => chars.next(),
            None => break None,
        };
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

#[cfg(test)]
mod test {
    use super::{eat_spaces, lex, Lexer, Token};

    #[test]
    fn test_eat_spaces() {
        let mut chars = "    a".chars().peekable();
        match eat_spaces(&mut chars) {
            Some(c) => assert_eq!(c, 'a'),
            None => panic!("expected 'a'"),
        }
    }

    // #[ignore]
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
    }
}
