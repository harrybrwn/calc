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
    let head = match stream.look_ahead(1) {
        &Token::Op(c) => match c {
            '*' | '/' => parse_term(stream)?,
            _ => Ast::new(stream.next().unwrap_or(Token::End)),
        },
        _ => return parse_term(stream),
    };

    match stream.peek() {
        Token::End => Ok(head),
        Token::Op(c) => match c {
            '+' | '-' => {
                let op = stream.next().unwrap_or(Token::Invalid);
                Ok(Ast::from(op, vec![head, parse_term(stream)?]))
            },
            _ => Err(format!("invalid operation: '{}'", c))
        },
        _ => Err(String::from("expected + or - operation")),
    }
}

pub fn parse_term(stream: &mut Lexer) -> AstRes {
    let head = stream.next().unwrap_or(Token::End);

    let left = match head {
        Token::Op(c) => {
            println!("term op: {}", c);
            panic!("aaaaahhhhhh");
        },
        Token::Int(..) | Token::Float(..) => Ast::new(head),
        _ => parse_term(stream)?,
    };

    let op = stream.next().unwrap_or(Token::End);
    match op {
        Token::End => Ok(left),
        Token::CloseParen => Ok(left),
        _ => Ok(Ast::from(op, vec![left, parse_factor(stream)?])),
    }
}

pub fn parse_factor(stream: &mut Lexer) -> AstRes {
    let head = stream.peek().clone();

    match head {
        Token::Int(..) | Token::Float(..) => Ok(Ast::new(stream.next().unwrap())),
        Token::OpenParen => {
            stream.pass();
            parse_expr(stream)
        },
        Token::Op(c) => match c {
            '-' => {
                panic!("not finished with negatives");
            },
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

    #[test]
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
                assert_eq!(ast.tok, Token::Op('+'));
                assert_eq!(ast.children[0].tok, Token::Int(1));
                assert_eq!(ast.children[1].tok, Token::Op('*'));
                assert_eq!(ast.children[1].children[0].tok, Token::Int(2));
                assert_eq!(ast.children[1].children[1].tok, Token::Int(3));
            },
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn test_another_pemdas() {
        let ast = parse("3 * 2 + 1");
        match ast {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('+'));
                assert_eq!(ast.children[1].tok, Token::Int(1));
                assert_eq!(ast.children[0].tok, Token::Op('*'));
                assert_eq!(ast.children[0].children[0].tok, Token::Int(3));
                assert_eq!(ast.children[0].children[1].tok, Token::Int(2))
            },
            Err(msg) => panic!(msg),
        }

        match parse("3 / 2 + 1") {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('+'));
                assert_eq!(ast.children[0].tok, Token::Op('/'));
            },
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn test_paren_groups() {
        println!();
        match parse("(1+2)") {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('+'));
                assert_eq!(ast.children[0].tok, Token::Int(1));
                assert_eq!(ast.children[1].tok, Token::Int(2));
            },
            Err(msg) => panic!(msg),
        }

        match parse("4 + (1 - 5)") {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('+'));
                assert_eq!(ast.children[0].tok, Token::Int(4));
                let ast = &ast.children[1];
                assert_eq!(ast.tok, Token::Op('-'));
                assert_eq!(ast.children[0].tok, Token::Int(1));
                assert_eq!(ast.children[1].tok, Token::Int(5));
            },
            Err(msg) => panic!(msg),
        }
        match parse("4 * (1 - 5)") {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('*'));
                assert_eq!(ast.children[0].tok, Token::Int(4));
                let ast = &ast.children[1];
                assert_eq!(ast.tok, Token::Op('-'));
                assert_eq!(ast.children[0].tok, Token::Int(1));
                assert_eq!(ast.children[1].tok, Token::Int(5));
            },
            Err(msg) => panic!(msg),
        }
        println!();
    }
}