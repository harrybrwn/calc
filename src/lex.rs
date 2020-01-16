use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
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
}

impl<'a> Lexer<'a> {

    pub fn new(text: &'a str) -> Self {
        Self{
            chars: text.chars().peekable(),
        }
    }

    pub fn peek(&mut self) -> Token {
        let c = self.eat_spaces();

        match c {
            '\0' => Token::End,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '0'...'9' => {
                let mut chrs = self.chars.clone();
                lex_num(&mut chrs)
            },
            '-' | '+' | '*' | '/' | '^' => Token::Op(c),
            _ => Token::Invalid,
        }
    }

    /// pass will skip the current token.
    #[allow(dead_code)]
    pub fn pass(&mut self) {
        match self.eat_spaces() {
            '\0' | ' ' | '(' | ')' |
            '+'  | '-' | '*' | '/' |
            '^'  => { self.next_ch(); },
            '0'...'9' => {
                loop {
                    let c = self.peek_ch();
                    if (c < '0' || c > '9') && c != '.' {
                        break;
                    }
                    self.next_ch();
                }
            },
            _ => (),
        }
    }

    fn peek_ch(&mut self) -> char {
        *self.chars.peek().unwrap_or(&'\0')
    }

    fn next_ch(&mut self) -> char {
        self.chars.next().unwrap_or('\0')
    }

    fn eat_spaces(&mut self) -> char {
        loop {
            let c = self.peek_ch();
            if c != ' ' {
                break c
            }
            self.next_ch();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.eat_spaces();

        let res = match c {
            '\0' => None,
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            '0'...'9' | '.' => {
                return Some(lex_num(&mut self.chars))
            },
            '-' | '+' | '*' | '/' | '^' => Some(Token::Op(c)),
            _ => None,
        };
        self.next_ch();
        res
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

#[allow(dead_code)]
pub fn lex(s: &str) -> Vec<Token> {
    let mut toks = vec![];
    let mut chars = s.chars().peekable();

    loop {
        let c = chars.peek().unwrap_or(&'\0');

        match *c {
            '\0' => break toks,
            '(' => toks.push(Token::OpenParen),
            ')' => toks.push(Token::CloseParen),
            '0'...'9' => {
                toks.push(lex_num(&mut chars));
                continue // don't call chars.next
            },
            '-' | '+' | '*' | '/' | '^' => toks.push(Token::Op(*c)),
            _ => (),
        }
        chars.next();
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
            let p = l.peek();
            let n = l.next().unwrap_or(Token::End);

            // println!("{:?} {:?}", p, n);
            match (p, n) {
                (Token::Int(a), Token::Int(b))     => assert_eq!(a, b),
                (Token::Float(a), Token::Float(b)) => assert_eq!(a, b),
                (Token::Op(a), Token::Op(b))       => assert_eq!(a, b),
                (Token::OpenParen, Token::OpenParen) => (),
                (Token::CloseParen, Token::CloseParen) => (),
                (Token::End, Token::End)           => break,
                (Token::Invalid, Token::Invalid)   => panic!("should not be invalid"),
                _ => panic!("tokens should be the same"),
            }
        }
    }

    #[test]
    fn test_iter() {
        let mut l = Lexer::new("1+1");
        match l.peek() {
            Token::Int(n) => assert_eq!(n, 1),
            _ => panic!("expected the number one"),
        }
        l.pass();
        match l.peek() {
            Token::Op(c) => assert_eq!(c, '+'),
            _ => panic!("expected '+'"),
        }
        l.pass();
        match l.peek() {
            Token::Int(n) => assert_eq!(n, 1),
            _ => panic!("expected number one"),
        }
    }
}
