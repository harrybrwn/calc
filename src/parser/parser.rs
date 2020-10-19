#![allow(dead_code)]

use crate::ast::Ast;
use crate::lex::{Lexer, Token};

type AstRes = Result<Ast, String>;

/// Parse a raw string and return the abstract syntax tree.
pub fn parse(text: &str) -> AstRes {
    let mut l = Lexer::new(text);
    let result = expr(&mut l);
    result
}

/*
 * < expression > ::= < term > + < expression > |
 *                    < term > - < expression > |
 *                    < term >
 *
 * < term > ::= < factor > * < term > |
 *              < factor > / < term > |
 *              < factor >
 *
 * < factor > ::= (< expression >) |
 *                < float > |
 *                < int >   |
 *                < var >
 */

fn expr(toks: &mut Lexer) -> AstRes {
    let head = match term(toks) {
        Ok(ast) if toks.is_empty() => return Ok(ast),
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };

    let mut root = match toks.next().unwrap() {
        Token::Op(c) => match c {
            '+' | '-' => Ast::new(Token::Op(c)),
            _ => return Err(format!("invalid operation")),
        },
        _ => return Ok(head),
    };
    let sub_expr = match expr(toks) {
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };
    root.push(head);
    root.push(sub_expr);
    Ok(root)
}

fn term(toks: &mut Lexer) -> AstRes {
    let res = match factor(toks) {
        Ok(ast) if toks.is_empty() => return Ok(ast),
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };

    let mut root = match toks.peek() {
        Token::Op(c) => match c {
            '/' | '*' => Ast::new(match toks.next() {
                Some(tok) => tok,
                None => {
                    return Err(format!(
                        "no more stuff (this needs to be a better error message, sorry)"
                    ))
                }
            }),
            '+' | '-' => return Ok(res),
            _ => return Err(format!("invalid operation")),
        },
        _ => {
            match toks.next() {
                Some(t) => match t {
                    Token::End => {
                        println!("here");
                        return Ok(res);
                    }
                    _ => {}
                },
                None => return Ok(res),
            }
            match toks.peek() {
                Token::Op('/') | Token::Op('*') => Ast::new(toks.next().unwrap()),
                _ => return Ok(res),
            }
        }
    };

    let right = match term(toks) {
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };
    root.push(res);
    root.push(right);
    Ok(root)
}

fn factor(toks: &mut Lexer) -> AstRes {
    match toks.peek() {
        Token::Int(..) | Token::Float(..) => Ok(Ast::new(toks.next().unwrap())),
        Token::OpenParen => match expr(&mut toks.capture_group()?) {
            Ok(ast) => Ok(ast.as_grouped()),
            Err(msg) => Err(msg),
        },
        Token::Op('-') => Ok(Ast::from(toks.next().unwrap(), vec![factor(toks)?])),
        Token::Invalid => Err(format!("invalid input")),
        _ => Err(format!("invalid factor '{:?}'", toks.peek())),
    }
}

fn extract_from_parens(toks: &mut Vec<Token>) -> Result<Vec<Token>, String> {
    let mut expr = vec![];
    let mut paren = 0;
    for t in toks.clone() {
        expr.push(t);
        match t {
            Token::CloseParen => {
                if paren == 0 {
                    break;
                } else {
                    paren -= 1;
                }
            }
            Token::OpenParen => {
                paren += 1;
            }
            _ => {}
        }
    }
    Ok(expr)
}

fn until_oneof<'a>(tokens: &'a mut Vec<Token>, delim: &[Token]) -> Vec<Token> {
    let mut v = vec![];
    for token in tokens {
        v.push(*token);
        for t in delim {
            if token == t {
                return v;
            }
        }
    }
    v
}
