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
        for i in 0..(len - 1) {
            write!(f, "{:?}, ", self.children[i].tok)?;
        }
        write!(f, "{:?}])", self.children[len - 1].tok)
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
    let head = stream.next().unwrap_or(Token::Invalid);
    let head = match head {
        Token::Float(..) => Ast::new(head),
        Token::Int(..) => Ast::new(head),
        _ => panic!("dont know what to do"),
    };

    match stream.peek() {
        Token::End => Ok(head),
        Token::Op(c) => match c {
            '+' | '-' => Ok(Ast::from(
                stream.next().unwrap(),
                vec![head, parse_term(stream)?]
            )),
            '*' | '/' => Ok(Ast::from(
                stream.next().unwrap(),
                vec![head, parse_factor(stream)?]
            )),
            _ => Err(String::from("invalid operation")),
        },
        _ => parse_factor(stream)
    }
}

pub fn parse_term(stream: &mut Lexer) -> AstRes {
    let head = stream.next().unwrap_or(Token::End);
    match head {
        Token::Op(c) => match c {
            _ => panic!("no compound terms"),
        },
        _ => Ok(Ast::from(head, vec![parse_factor(stream)?])),
    }
}

pub fn parse_factor(stream: &mut Lexer) -> AstRes {
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
            _   => Err(String::from("invlaid operation")),
        },
        _ => Ok(Ast::new(head)),
    }
}

fn parse(text: &str) -> AstRes {
    let mut l = Lexer::new(text);
    parse_expr(&mut l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let ast = parse("1+1");

        match ast {
            Ok(ast) => {
                println!("{}", ast);
                // println!("{:?}", ast.tok);
            },
            Err(msg) => panic!(msg),
        }
    }
}