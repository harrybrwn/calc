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

pub fn eval(ast: &Ast) -> f64 {
    match ast.tok {
        Token::Op(c) => match c {
            '+' => eval(&ast.children[0]) + eval(&ast.children[1]),
            '-' => eval(&ast.children[0]) - eval(&ast.children[1]),
            '*' => eval(&ast.children[0]) * eval(&ast.children[1]),
            '/' => {
                eval(&ast.children[0]) as f64 / eval(&ast.children[1]) as f64
            },
            _ => panic!("invalid op"),
        },
        Token::Int(n) => n as f64,
        Token::Float(n) => n,
        _ => 0.0,
    }
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

fn parse_expr(stream: &mut Lexer) -> AstRes {
    let head = match stream.look_ahead(1) {
        Token::Op(c) => match c {
            '*' | '/' => parse_term(stream)?,
            _ => Ast::new(stream.next().unwrap_or(Token::End)),
        },
        // this is the case where there is only a term,
        // no op followed by a term.
        _ => parse_term(stream)?,
    };

    match stream.peek() {
        Token::End => Ok(head),
        Token::Op(..) => {
            // this is the case where there is an operation followed by a term.
            let op = stream.next().unwrap_or(Token::End);
            let term = parse_term(stream)?;
            Ok(Ast::from(op, vec![head, term]))
        },
        _ => Err(String::from("expected + or - operation")),
    }
}

fn parse_term(stream: &mut Lexer) -> AstRes {
    let head = match stream.look_ahead(1) {
        Token::CloseParen => return Ok(
            Ast::new(stream.next().unwrap_or(Token::End))
        ),
        Token::Op(c) => match c {
            '*' | '/' => Ast::new(stream.next().unwrap_or(Token::End)),
            _ => return parse_factor(stream),
        },
        _ => parse_factor(stream)?,
    };

    match stream.peek() {
        Token::End => Ok(head),
        _ => Ok(Ast::from(
            stream.next().unwrap_or(Token::End),
            vec![head, parse_factor(stream)?]
        )),
    }
}

fn parse_factor(stream: &mut Lexer) -> AstRes {
    let head = stream.peek();

    match head {
        Token::Int(..) | Token::Float(..) => Ok(Ast::new(stream.next().unwrap())),
        Token::OpenParen => {
            stream.next();
            let res = parse_expr(stream);

            match stream.next().unwrap_or(Token::End) {
                Token::CloseParen => res,
                _ => Err(String::from("expected ')'")),
            }
        },
        Token::Op(c) => match c {
            '-' => {
                panic!("not finished with negatives");
            },
            _ => Err(format!("invlaid operation '{}' (parse_factor)", c)),
        },
        _ => Ok(Ast::new(stream.next().unwrap_or(Token::End))),
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
    fn test_nested_div() {
        match parse("3/2/4") {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('/'));
                assert_eq!(ast.children[1].tok, Token::Int(4));
                let ast = &ast.children[0];
                assert_eq!(ast.tok, Token::Op('/'));
                assert_eq!(ast.children[0].tok, Token::Int(3));
                assert_eq!(ast.children[1].tok, Token::Int(2));
            },
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn test_paren_groups() {
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
    }

    #[test]
    fn test_complex_expr() {
        match parse("(1+4*5)") {
            Ok(ast) => assert_eq!(ast.tok, Token::Op('+')),
            Err(msg) => panic!(msg),
        }

        match parse("(1+4*5)-5") {
            Ok(ast) => {
                assert_eq!(eval(&ast), ( (1+4*5)-5 ) as f64);

                assert_eq!(ast.tok, Token::Op('-'));
                assert_eq!(ast.children[1].tok, Token::Int(5));
                let ast = &ast.children[0];
                assert_eq!(ast.tok, Token::Op('+'));
                assert_eq!(ast.children[0].tok, Token::Int(1));
                assert_eq!(ast.children[1].children[0].tok, Token::Int(4));
                assert_eq!(ast.children[1].children[1].tok, Token::Int(5));
            },
            Err(msg) => panic!(msg),
        }

        match parse("5-(1+4*5") {
            Ok(..) => panic!("expected an error here"),
            Err(..) => (),
        }
    }
}