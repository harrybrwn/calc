#![allow(dead_code)]

use std::fmt;

use crate::lex::Lexer;
use crate::lex::Token;

type AstRes = Result<Ast, String>;

/*
Grammar:

expr   ::= expr + term |
           expr - term |
           term
term   ::= term * factor |
           term / factor |
           factor
factor ::= 0-9      |
           ( expr ) |
           - factor
*/

pub struct Ast {
    pub tok: Token,
    pub children: Vec<Ast>,
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.children.len();

        write!(f, "Ast({:?}: [", self.tok)?;
        if len > 0 {
            for i in 0..(len - 1) {
                write!(f, "{:?}, ", self.children[i].tok)?;
            }
            write!(f, "{:?}])", self.children[len - 1].tok)
        } else {
            write!(f, "])")
        }
    }
}

impl Ast {
    pub fn new(t: Token) -> Self {
        Self{
            tok: t,
            children: vec![],
        }
    }

    pub fn from(t: Token, children: Vec<Ast>) -> Self {
        let mut ast = Self::new(t);
        ast.children = children;
        ast
    }
}

pub fn parse_expr(stream: &mut Lexer) -> AstRes {
    println!("parse_expr");
    let head = stream.next().unwrap_or(Token::Invalid);
    let head = match head {
        Token::Int(..) | Token::Float(..) => Ast::new(head),
        _ => panic!("dont know what to do"),
    };

    println!("peek: {:?}, head: {}", stream.peek(), head);
    match stream.peek() {
        Token::End => Ok(head),
        Token::Op(c) => match c {
            '+' | '-' => {
                let op = stream.next().unwrap_or(Token::Invalid);
                Ok(Ast::from(op, vec![head, parse_term(stream)?]))
            },
            '*' | '/' => {
                let op = stream.next().unwrap_or(Token::Invalid);
                Ok(Ast::from(op, vec![head, parse_factor(stream)?]))
            },
            _ => Err(format!("invalid operation: '{}'", c))
        },
        _ => parse_term(stream)
    }
}

pub fn parse_term(stream: &mut Lexer) -> AstRes {
    println!("parse_term");
    let head = stream.next().unwrap_or(Token::End);
    match head {
        Token::Op(c) => match c {
            _ => panic!("no compound terms yet"),
        },
        Token::Int(..) | Token::Float(..) => {
            let op = stream.next().unwrap_or(Token::Invalid);
            let factor = parse_factor(stream);
            Ok(Ast::from(op, vec![Ast::new(head), factor?]))
        },
        _ => parse_factor(stream),
    }
}

pub fn parse_factor(stream: &mut Lexer) -> AstRes {
    println!("parse_factor");
    let head = stream.peek();

    match head {
        Token::Int(_) | Token::Float(_) => Ok(Ast::new(stream.next().unwrap())),
        Token::OpenParen => {
            stream.pass();
            parse_expr(stream)
        },
        Token::Op(c) => match c {
            '-' => {
                panic!("not finished with negatives");
                stream.pass();
                Ok(Ast::from(head, vec![parse_factor(stream)?]))
            },
            // _   => Err(String::from("invlaid operation '{}' (parse_factor)", c)),
            _ => Err(format!("invlaid operation '{}' (parse_factor)", c)),
        },
        _ => Ok(Ast::new(head)),
    }
}

pub fn parse(text: &str) -> AstRes {
    let mut l = Lexer::new(text);
    parse_expr(&mut l)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_parser() {
        let ast = parse("1+1");
        match ast {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('+'));
                assert_eq!(ast.children[0].tok, Token::Int(1));
                assert_eq!(ast.children[1].tok, Token::Int(1));
            },
            Err(msg) => panic!(msg),
        }

        let ast = parse("9.4 / 5");
        match ast {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('/'));
                assert_eq!(ast.children[0].tok, Token::Float(9.4));
                assert_eq!(ast.children[1].tok, Token::Int(5));
            },
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn test_complex_str() {
        let ast = parse("1 + 2 * 3");
        match ast {
            Ok(ast) => {
                println!("{}", ast);
                assert_eq!(ast.tok, Token::Op('+'));
            },
            Err(msg) => panic!(msg),
            // _ => (),
        }
    }
}