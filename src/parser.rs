#![allow(dead_code)]

use crate::ast::Ast;
use crate::lex::Token;

type AstRes = Result<Ast, String>;

/// Parse a raw string and return the abstract syntax tree.
///
/// # Examples
///
/// ```
/// // let ast = parse("2 + (4 * 3 / 2)");
/// // let result = eval(&ast);
/// ```
pub fn parse(text: &str) -> AstRes {
    // let mut l = Lexer::new(text);
    Err(format!("no implemented {}", text))
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

fn expr(toks: &mut Vec<Token>) -> AstRes {
    let head = match term(toks) {
        Ok(ast) if toks.len() == 0 => return Ok(ast),
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };
    let mut root = match toks.remove(0) {
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

fn term(toks: &mut Vec<Token>) -> AstRes {
    let res = match factor(toks) {
        Ok(ast) if toks.len() == 0 => return Ok(ast),
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };
    let mut root = match toks[0] {
        Token::Op(c) => match c {
            '/' | '*' => Ast::new(toks.remove(0)),
            '+' | '-' => return Ok(res),
            _ => return Err(format!("invalid operation")),
        },
        // _ if toks.len() == 1 => return Ok(res),
        _ => {
            toks.remove(0);
            if toks.len() < 1 {
                return Ok(res);
            }
            match toks[0] {
                // TODO: check too see what operations will break this
                //       for now we are allowing all ops
                Token::Op(..) => Ast::new(toks.remove(0)),
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

fn factor(toks: &mut Vec<Token>) -> AstRes {
    match toks[0] {
        Token::Int(..) | Token::Float(..) => Ok(Ast::new(toks.remove(0))),
        Token::OpenParen => {
            toks.remove(0); // shift left
            let mut exprtoks = vec![];
            let mut paren = 0;
            for t in toks.clone() {
                match t {
                    Token::CloseParen => {
                        if paren == 0 {
                            exprtoks.push(t);
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
                exprtoks.push(t);
            }
            let size = exprtoks.len() - 1;
            if exprtoks.pop().unwrap() != Token::CloseParen {
                return Err(format!("expected ')'"));
            }
            toks.drain(0..size + 0);
            expr(&mut exprtoks)
        }
        _ => Err(format!("invalid factor '{:?}'", toks[0])),
    }
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

#[cfg(test)]
mod test {
    // use super::{parse, parse_expr, parse_factor};
    use super::factor;
    use super::term;
    use crate::ast::eval;
    use crate::ast::Ast;
    use crate::lex::{Lexer, Token};
    use crate::parser::expr;
    use crate::parser::until_oneof;

    #[test]
    fn test_factor() {
        for s in vec!["1", "(1)", "((1))"] {
            match factor(&mut Lexer::new(s).as_vec()) {
                Ok(ast) => {
                    assert_eq!(ast.tok, Token::Int(1));
                    assert_eq!(ast.children.len(), 0);
                }
                Err(msg) => panic!(msg),
            }
        }
        let mut t = Lexer::new("1*1").as_vec();
        match factor(&mut t) {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Int(1));
            }
            Err(msg) => panic!(msg),
        }
        match factor(&mut t) {
            Err(..) => {}
            Ok(..) => panic!("expected an error"),
        }
    }

    #[test]
    fn test_term() {
        match term(&mut Lexer::new("(1)").as_vec()) {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Int(1));
            }
            Err(msg) => panic!(msg),
        }

        match term(&mut Lexer::new("1*1").as_vec()) {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('*'));
                assert_eq!(ast.children[0].tok, Token::Int(1));
                assert_eq!(ast.children[1].tok, Token::Int(1));
            }
            Err(msg) => panic!(msg),
        }
        let _exp = Ast::from(
            Token::Op('/'),
            vec![
                Ast::from(
                    Token::Op('/'),
                    vec![Ast::new(Token::Int(1)), Ast::new(Token::Int(1))],
                ),
                Ast::new(Token::Int(1)),
            ],
        );

        for s in vec![
            "1/2/3",
            "1/2/(3)",
            "(1)/2/3",
            "(1/2)/3",
            "1/(2)/3",
            "((1)/2)/3",
            "(((1/2/3)))",
            "(((((1/2)/3))))",
        ] {
            match term(&mut Lexer::new(s).as_vec()) {
                Ok(ast) => {
                    assert_eq!(ast.tok, Token::Op('/'));
                    assert_eq!(ast.children[0].tok, Token::Op('/'));
                    assert_eq!(ast.children[0].children[0].tok, Token::Int(1));
                    assert_eq!(ast.children[0].children[1].tok, Token::Int(2));
                    assert_eq!(ast.children[1].tok, Token::Int(3));
                    assert_eq!(eval(&ast), (1.0 / 2.0 / 3.0));
                }
                Err(msg) => panic!(msg),
            }
        }
        match term(&mut Lexer::new("3*2*5").as_vec()) {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('*'));
                assert_eq!(ast.children[0].tok, Token::Int(3));
                assert_eq!(ast.children[1].tok, Token::Op('*'));
                assert_eq!(ast.children[1].children[0].tok, Token::Int(2));
                assert_eq!(ast.children[1].children[1].tok, Token::Int(5));
            }
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn test_expr() {
        for s in vec![
            "5 + 3 * 3 / 6",
            "5+3*3/6",
            "5+(3*3/6)",
            "5+(3*(3/6))",
            "(5+(3*3/6))",
            "(5+(3*(3/6)))",
            "5             +3  * (3)     / (          6)",
        ] {
            match expr(&mut Lexer::new(s).as_vec()) {
                Ok(ast) => {
                    assert_eq!(eval(&ast), 5.0 + 3.0 * 3.0 / 6.0);
                    assert_eq!(eval(&ast), 5.0 + (3.0 * 3.0 / 6.0));
                    assert_eq!(eval(&ast), 5.0 + (3.0 * (3.0 / 6.0)));
                    assert_eq!(eval(&ast), 5.0 + 3.0 * (3.0 / 6.0));
                    assert_eq!(ast.tok, Token::Op('+'));
                    assert_eq!(ast.children[0].tok, Token::Int(5));
                    assert_eq!(ast.children[1].tok, Token::Op('*'));
                    assert_eq!(ast.children[1].children[0].tok, Token::Int(3));
                    assert_eq!(ast.children[1].children[1].tok, Token::Op('/'));
                    assert_eq!(ast.children[1].children[1].children[0].tok, Token::Int(3));
                    assert_eq!(ast.children[1].children[1].children[1].tok, Token::Int(6));
                }
                Err(msg) => panic!(msg),
            }
        }
    }

    #[test]
    fn test_tokens_until() {
        let mut v = vec![
            Token::OpenParen,
            Token::Int(4),
            Token::Op('+'),
            Token::Int(2),
            Token::CloseParen,
        ];
        let r = until_oneof(&mut v, &[Token::Op('+')]);
        assert_eq!(r[0], Token::OpenParen);
        assert_eq!(r[1], Token::Int(4));
        assert_eq!(r[2], Token::Op('+'));
    }
}
