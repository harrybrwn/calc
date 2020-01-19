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
            '-' => if ast.children.len() == 1 {
                -1.0 * eval(&ast.children[0])
            } else {
                eval(&ast.children[0]) - eval(&ast.children[1])
            },
            '*' => eval(&ast.children[0]) * eval(&ast.children[1]),
            '/' => eval(&ast.children[0]) / eval(&ast.children[1]),
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
            _ => Ast::new(stream.next().unwrap()),
        },
        // this is the case where there is only a term,
        // no op followed by a term.
        _ => parse_term(stream)?,
    };

    match stream.peek() {
        Token::End => Ok(head),
        Token::Op(..) => {
            // this is the case where there is an operation
            // followed by a term.
            let op = stream.next().unwrap();
            let term = parse_term(stream)?;
            Ok(Ast::from(op, vec![head, term]))
        },
        _ => Err(String::from("expected + or - operation")),
    }
}

fn parse_term(stream: &mut Lexer) -> AstRes {
    let head = match stream.look_ahead(1) {
        // we have reched the end of an expression, must return
        Token::CloseParen => return Ok(
            Ast::new(stream.next().unwrap())
        ),
        Token::Op(c) => match c {
            '*' | '/' => Ast::new(stream.next().unwrap()),
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
    let head = stream.next().unwrap_or(Token::End);

    match head {
        Token::Int(..) | Token::Float(..) => Ok(
            Ast::new(head)
        ),
        Token::OpenParen => {
            let expr = parse_expr(stream);

            match stream.next().unwrap_or(Token::End) {
                Token::CloseParen => expr,
                _ => Err(String::from("expected ')'")),
            }
        },
        Token::Op(c) => match c {
            '-' => Ok(Ast::from(head, vec![parse_factor(stream)?])),
            _ => Err(format!("invlaid operation '{}'", c)),
        },
        _ => Ok(Ast::new(head)),
    }
}

pub fn parse(text: &str) -> AstRes {
    let mut l = Lexer::new(text);
    parse_expr(&mut l)
}