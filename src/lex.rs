#![allow(dead_code)]

use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

// extern crate radix_trie;
// use radix_trie::Trie;

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

    Negation, // TODO: use ¬ or ~

    Modulus,
    Factorial, // TODO:

    Of, // TODO: 10 % of 3

    // TODO:
    //      - sum
    //      - sqrt
    //      - log
    Func,   // TODO: add functions
    Assign, // TODO: add assignment support ("let name = ...")
    Equal,  // TODO: a single equal sign like real math f = 2*x

    Ident, // Identifiers

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
    Mod,
    Fac, // Factorial
    Percent,
    Not,
    Invalid,
}

pub fn lex(s: &str) -> Vec<Token> {
    let mut toks = vec![];
    let mut chars = s.chars().peekable();
    loop {
        let (next, _) = next_token(&mut chars);
        match next {
            Token::End | Token::Invalid => break toks,
            _ => toks.push(next),
        }
    }
}

fn next_token<'b>(chars: &mut Peekable<Chars>) -> (Token, usize) {
    let (tok, inc) = match eat_spaces(chars) {
        (Some(c), i) => match c {
            '(' => (Token::OpenParen, i),
            ')' => (Token::CloseParen, i),
            '0'..='9' | '.' => {
                let (n, adv) = lex_num(chars);
                return (n, adv + i);
            }
            '-' | '+' | '*' | '/' | '^' | '%' => (Token::Op(c), i + 1),
            '!' => (Token::Factorial, i),
            '¬' => (Token::Negation, i), // this might make things hard
            '~' => (Token::Negation, i),
            'a'..='z' => {
                // TODO: write a better way to tokenize keywords
                let mut s = String::new();
                let mut read = 0;
                while let Some(c) = chars.next() {
                    if c.is_digit(10) {
                        return (Token::Invalid, i + read);
                    }
                    s.push(c);
                    read += 1;
                    let keyword = match s.as_str() {
                        "mod" => (Token::Modulus, i + read),
                        "of" => (Token::Of, i + read + 1),
                        _ => continue,
                    };
                    if let Some(&c) = chars.peek() {
                        if c != ' ' {
                            return (Token::Invalid, keyword.1);
                        }
                    }
                    return keyword;
                }
                (Token::Invalid, i + read)
            }
            _ => return (Token::Invalid, 0),
        },
        (None, ..) => return (Token::End, 0),
    };
    chars.next(); // skip what we just peeked
    (tok, inc)
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    buf: Vec<Token>,
    index: usize,
    raw: String,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        let raw = String::from(text);
        Self {
            chars: text.chars().peekable(),
            buf: vec![],
            index: 0,
            raw: raw,
        }
    }

    pub fn from(toks: Vec<Token>) -> Self {
        return Self {
            chars: "".chars().peekable(),
            buf: toks,
            index: 0,
            raw: String::new(),
        };
    }

    fn new_empty() -> Self {
        Self {
            chars: "".chars().peekable(),
            buf: vec![],
            index: 0,
            raw: String::new(),
        }
    }

    pub fn get_index(self) -> usize {
        self.index
    }

    pub fn error_at_current(&self) -> Vec<String> {
        let mut v = vec![];
        let wrap = 3;
        let mut s = String::new();
        let code = &self.raw[self.index - wrap..self.index + wrap];
        s.extend(code.chars());
        v.push(s.clone());
        s.clear();

        s.extend((0..wrap).map(|_| " ").collect::<String>().chars());
        s.push('^');
        v.push(s);
        v
    }

    pub fn peek(&mut self) -> Token {
        if self.buf.len() == 0 {
            let (next, i) = next_token(&mut self.chars);

            // println!(
            //     "Lexer::peek => {} {:?}",
            //     &self.raw.clone()[self.index..self.index + 1],
            //     next
            // );

            // println!("Lexer::peek => {:?}", next);
            // if i < self.raw.len() {
            //     println!(
            //         "\"{}\" advance({}) peek()=>{:?} {}",
            //         self.raw,
            //         i,
            //         next,
            //         self.raw.as_bytes()[i] as char,
            //     );
            // }
            self.buf.push(next);
            self.index += i;
            // self.buf.push(next_token(&mut self.chars));
        }
        self.buf[0]
    }

    pub fn look_ahead(&mut self, n: usize) -> Token {
        let len = self.buf.len();
        if len > 0 && len > n {
            return self.buf[n];
        }

        // let (mut next, ix): (Token, usize);
        for _ in len..n {
            let (next, ix) = next_token(&mut self.chars);
            if next == Token::Invalid {
                return Token::End;
            }
            self.index += ix;
            self.buf.push(next);
            if next == Token::End {
                return next;
            }
        }
        let (next, ix) = next_token(&mut self.chars);
        self.index += ix;
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
            let (t, _) = next_token(&mut chars);
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
                Token::CloseParen => {}
                _ => return Err(format!("expected ')'")),
            },
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
        let (tok, i) = next_token(&mut self.chars);
        self.index += i;
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
            index: self.index,
            raw: self.raw.clone(),
        }
    }
}

fn eat_spaces<'a>(chars: &'a mut Peekable<Chars>) -> (Option<char>, usize) {
    let mut i = 0;
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' => {}
            _ => return (Some(c), i),
        }
        chars.next();
        i += 1;
    }
    (None, i)
}

fn lex_num(chars: &mut Peekable<Chars>) -> (Token, usize) {
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
        (Token::Float(s.parse::<f64>().unwrap()), s.len())
    } else {
        (Token::Int(s.parse::<i64>().unwrap()), s.len())
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
            Op::Add => write!(f, "Add(+)"),
            Op::Sub => write!(f, "Sub(-)"),
            Op::Mul => write!(f, "Mul(*)"),
            Op::Div => write!(f, "Div(/)"),
            Op::Exp => write!(f, "Exp({})", self.as_str()),
            Op::Mod => write!(f, "Mod({})", self.as_str()),
            Op::Not => write!(f, "Not({})", self.as_str()),
            Op::Fac => write!(f, "Fac(!)"),
            Op::Percent => write!(f, "Percent(%)"),
            Op::Invalid => write!(f, "Invalid"),
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
        '~' => Op::Not,
        'm' => Op::Mod,
        '!' => Op::Fac,
        _ => Op::Invalid,
    }
}

impl Op {
    fn as_char(&self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
            Op::Exp => '^',
            Op::Mod => 'm',
            Op::Not => '~',
            Op::Fac => '!',
            Op::Percent => '%',
            Op::Invalid => '0',
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Exp => "^",
            Op::Mod => "mod",
            Op::Not => "~",
            Op::Fac => "!",
            Op::Percent => "%",
            Op::Invalid => "<invalid>",
        }
    }

    fn presidence() -> i8 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::{eat_spaces, lex, lex_num, Lexer, Token};

    #[test]
    fn test_lex_num() {
        let mut ch = "123".chars().peekable();
        let res = match lex_num(&mut ch) {
            (Token::Int(i), ..) => i,
            (Token::Invalid, ..) => panic!("should not get invalid token for \"123\""),
            _ => 0,
        };
        assert_eq!(res, 123);
    }

    #[test]
    fn test_eat_spaces() {
        let mut chars = "    a".chars().peekable();
        match eat_spaces(&mut chars) {
            (Some(c), ..) => assert_eq!(c, 'a'),
            (None, ..) => panic!("expected 'a'"),
        }
        let mut ch = "a".chars().peekable();
        assert_eq!('a', eat_spaces(&mut ch).0.unwrap());
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
