use std::iter::Peekable;
use std::str::Chars;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Op(char),
    Num(i64),
    End,
    Invalid,
}

#[allow(dead_code)]
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

fn parse_num<'a>(chars: &mut Peekable<Chars<'a>>, default: i64) -> i64 {
    let mut num = default;
    loop {
        let c = *chars.peek().unwrap_or(&'\0');
        if c < '0' || c > '9' {
            break num;
        }
        num = (num * 10) + (c as i64 - '0' as i64);
        chars.next();
    }
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self{
            chars: text.chars().peekable(),
        }
    }

    pub fn peek(&mut self) -> Token {
        let c = loop {
            let c = *self.chars.peek().unwrap_or(&'\0');
            if c != ' ' {
                break c;
            }
            self.chars.next();
        };

        match c {
            '\0' => Token::End,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '0'...'9' => {
                let mut chrs = self.chars.clone();
                Token::Num(parse_num(&mut chrs, 0))
            },
            '-' | '+' | '*' | '/' | '^' => Token::Op(c),
            _ => Token::Invalid,
        }
    }

    pub fn next(&mut self) -> Token {
        let c = loop {
            let c = self.chars.next().unwrap_or('\0');
            if c != ' ' {
                break c;
            }
        };

        match c {
            '\0' => Token::End,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '0'...'9' => {
                let num = c as i64 - '0' as i64;
                Token::Num(parse_num(&mut self.chars, num))
            },
            '-' | '+' | '*' | '/' | '^' => Token::Op(c),
            _ => Token::Invalid,
        }
    }
}

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
                let mut num = 0;

                toks.push(Token::Num(loop {
                    let c = *chars.peek().unwrap_or(&'\0');

                    if c < '0' || c > '9' {
                        break num;
                    }
                    num = (num * 10) + (c as i64 - '0' as i64);
                    chars.next();
                }));
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
        let s = "5 + 335 * (1+1)";
        let res = lex(s);
        match res[0] {
            Token::Num(x) => assert_eq!(x, 5),
            _ => panic!("should be a number token"),
        }
        match res[1] {
            Token::Op(x) => assert_eq!(x, '+'),
            _ => panic!("should be an op token"),
        }
        match res[2] {
            Token::Num(x) => assert_eq!(x, 335),
            _ => panic!("should be a number token"),
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
            Token::Num(x) => assert_eq!(x, 1),
            _ => panic!("should be num"),
        }
        match res[6] {
            Token::Op(c) => assert_eq!(c, '+'),
            Token::Num(x) => panic!("should not be number: {}", x),
            _ => panic!("should be op"),
        }
        match res[8] {
            Token::CloseParen => {},
            _ => panic!("should be closed paren"),
        }

        let mut l = Lexer::new(s);
        loop {
            let p = l.peek();
            println!("{:?}", l.peek());
            let n = l.next();
            match n {
                Token::End => break,
                _ => println!("{:?} {:?}", p, n),
            }
            // assert_eq!(p.unwrap(), n.unwrap());
        }
    }
}
