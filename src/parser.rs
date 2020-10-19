#![allow(dead_code)]

use crate::ast::Ast;
use crate::lex::{
    Lexer,
    Token,
    Token::{
        Op,
        Int,
        Float,
        Modulus,
        OpenParen,
        CloseParen,
        Invalid,
    },
};

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
 * < term > ::= < factor > *   < term > |
 *              < factor > /   < term > |
 *              < factor > ^   < term > |
 *              < factor > mod < term > |
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
        Op(c) => match c {
            '+' | '-' => Ast::new(Op(c)),
            _ => return Err(format!("invalid operation")),
        },
        _ => return Ok(head),
    };
    root.push(head);
    root.push(match expr(toks) {
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    });
    Ok(root)
}

fn term(toks: &mut Lexer) -> AstRes {
    let res = match factor(toks) {
        Ok(ast) if toks.is_empty() => return Ok(ast),
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };

    let mut root = match toks.peek() {
        Op(c) => match c {
            '^' | '/' | '*' => Ast::new(match toks.next() {
                Some(tok) => tok,
                None => {
                    return Err(format!(
                        "no more stuff (this needs to be a better error message, sorry)"
                    ))
                }
            }),
            '+' | '-' => return Ok(res),
            _ => return Err(format!("invalid operation '{}'", c)),
        },
        Modulus => Ast::new(toks.next().unwrap()),
        _ => return Err(format!("unexpected input")),
    };
    root.push(res);
    if let Ok(rhs) = term(toks) {
        root.push(rhs);
    }
    Ok(root)
}

fn factor(toks: &mut Lexer) -> AstRes {
    match toks.peek() {
        Int(..) | Float(..) => Ok(Ast::new(toks.next().unwrap())),
        OpenParen => match expr(&mut toks.capture_group()?) {
            Ok(ast) => Ok(ast.as_grouped()),
            Err(msg) => Err(msg),
        },
        Op('-') => Ok(Ast::from(toks.next().unwrap(), vec![factor(toks)?])),
        Invalid => Err(format!("invalid input")),
        _ => Err(format!("invalid factor '{:?}'", toks.peek())),
    }
}

fn extract_from_parens(toks: &mut Vec<Token>) -> Result<Vec<Token>, String> {
    let mut expr = vec![];
    let mut paren = 0;
    for t in toks.clone() {
        expr.push(t);
        match t {
            CloseParen => {
                if paren == 0 {
                    break;
                } else {
                    paren -= 1;
                }
            }
            OpenParen => {
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

#[cfg(test)]
mod test {
    use super::parse;
    use super::{expr, factor, term, until_oneof};
    use crate::ast::eval;
    use crate::lex::{Lexer, Token, Token::{Int}};

    #[test]
    fn test_modulo() {
        match parse("4 mod 5") {
            Ok(ast) => {
                println!("{}", ast);
                assert_eq!(ast.tok, Token::Modulus);
                assert_eq!(ast.children[0].tok, Int(4));
                assert_eq!(ast.children[1].tok, Int(5));
                assert_eq!(eval(&ast), 4.0 % 5.0);
            },
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn test_exponentiate() {
        for s in vec!["2^2", "2^(2)", "(2)^2", "(2)^(2)"] {
            match parse(s) {
                Ok(ast) => {
                    assert_eq!(ast.tok, Token::Op('^'));
                    assert_eq!(ast.children[0].tok, Int(2));
                    assert_eq!(ast.children[1].tok, Int(2));
                }
                Err(msg) => panic!(msg),
            }
        }
        for s in vec!["2^3^2", "2^(3^2)"] {
            match parse(s) {
                Ok(ast) => {
                    assert_eq!(ast.tok, Token::Op('^'));
                    assert_eq!(ast.children[0].tok, Int(2));
                    assert_eq!(ast.children[1].tok, Token::Op('^'));
                    assert_eq!(ast.children[1].children[0].tok, Int(3));
                    assert_eq!(ast.children[1].children[1].tok, Int(2));
                    assert_eq!(eval(&ast), (2.0 as f64).powf((3.0 as f64).powf(2.0)));
                }
                Err(msg) => panic!(msg),
            }
        }
        match parse("(2^3)^2") {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('^'));
                assert_eq!(ast.children[1].tok, Int(2));
                assert_eq!(ast.children[0].tok, Token::Op('^'));
                assert_eq!(ast.children[0].children[0].tok, Int(2));
                assert_eq!(ast.children[0].children[1].tok, Int(3));
                assert_eq!(eval(&ast), (2.0 as f64).powf(3.0).powf(2.0));
            }
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn test_parse() {
        let t = match parse("3/(3/4/5)/6") {
            Ok(ast) => ast,
            Err(msg) => panic!(msg),
        };
        assert_eq!(t.children[0].tok, Token::Op('/'));
        assert_eq!(t.children[1].tok, Int(6));
        assert_eq!(t.children[0].children[0].tok, Int(3));
        assert_eq!(t.children[0].children[1].children[0].tok, Token::Op('/'));
        assert_eq!(t.children[0].children[1].children[1].tok, Int(5));
        let sub = &t.children[0].children[1].children[0];
        assert_eq!(sub.children[0].tok, Int(3));
        assert_eq!(sub.children[1].tok, Int(4));
        assert_eq!(eval(&t), 3.0 / (3.0 / 4.0 / 5.0) / 6.0);
        match parse("") {
            Err(..) => {}
            Ok(r) => panic!("expected an error from an empty string, got {}", r),
        }
    }

    #[test]
    fn test_factor() {
        // none of these should parse farther than the first number
        for s in vec!["1", "(1)", "((1))", "1*1", "1/1", "1+1", "1-1"] {
            let mut t = Lexer::new(s);
            match factor(&mut t) {
                Ok(ast) => {
                    assert_eq!(ast.tok, Int(1));
                    assert_eq!(ast.children.len(), 0);
                }
                Err(msg) => panic!(msg),
            }
        }
        for s in vec![
            "-3.1", "(-3.1)", "-(3.1)", "-((3.1))", "(-(3.1))", "((-3.1))",
        ] {
            match factor(&mut Lexer::new(s)) {
                Ok(ast) => {
                    assert_eq!(ast.tok, Token::Op('-'));
                    assert_eq!(ast.children.len(), 1);
                    assert_eq!(ast.children[0].tok, Token::Float(3.1));
                }
                Err(msg) => panic!(msg),
            }
        }
    }

    #[test]
    fn test_term() {
        match term(&mut Lexer::new("(1)")) {
            Ok(ast) => assert_eq!(ast.tok, Int(1)),
            Err(msg) => panic!(msg),
        }
        match term(&mut Lexer::new("1*1")) {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('*'));
                assert_eq!(ast.children[0].tok, Int(1));
                assert_eq!(ast.children[1].tok, Int(1));
            }
            Err(msg) => panic!(msg),
        }

        for s in vec![
            ("1/2/3", '/'),
            ("1/2/(3)", '/'),
            ("(1)/2/3", '/'),
            ("(1/2)/3", '/'),
            ("1/(2)/3", '/'),
            ("((1)/2)/3", '/'),
            ("(((1/2/3)))", '/'),
            ("(((((1/2)/3))))", '/'),
            ("1*2*3", '*'),
            ("1*2*(3)", '*'),
            ("(1)*2*3", '*'),
            ("(1*2)*3", '*'),
            ("1*(2)*3", '*'),
            ("((1)*2)*3", '*'),
            ("(((1*2*3)))", '*'),
            ("(((((1*2)*3))))", '*'),
        ] {
            match term(&mut Lexer::new(s.0)) {
                Ok(ast) => {
                    assert_eq!(ast.tok, Token::Op(s.1));
                    assert_eq!(ast.children[0].tok, Token::Op(s.1));
                    assert_eq!(ast.children[0].children[0].tok, Int(1));
                    assert_eq!(ast.children[0].children[1].tok, Int(2));
                    assert_eq!(ast.children[1].tok, Int(3));
                    assert_eq!(
                        eval(&ast),
                        match s.1 {
                            '/' => (1.0 / 2.0 / 3.0),
                            '*' => 1.0 * 2.0 * 3.0,
                            _ => panic!("this test should be used for '*' and '/' ops only"),
                        }
                    );
                }
                Err(msg) => panic!(msg),
            }
        }
    }

    #[test]
    fn test_expr() {
        let answers = vec![
            5.0 + 3.0 * 3.0 / 6.0,
            5.0 + 3.0 * 3.0 / 6.0,
            5.0 + (3.0 * 3.0 / 6.0),
            5.0 + 3.0 * (3.0 / 6.0),
            5.0 + (3.0 * 3.0) / 6.0,
            5.0 + ((3.0 * 3.0) / 6.0),
            ((5.0) + (((3.0) * (3.0)) / (6.0))),
        ];
        for s in vec![
            "5 + 3 * 3 / 6",
            "5+3*3/6",
            "5+(3*3/6)",
            "5+((3*3)/6)",
            "(5+(3*3/6))",
            "5           +3  * (3)     / (          6)",
            "5+(3*3)/6",
            "((5) + (((3) * (3)) / (6)))",
        ] {
            match expr(&mut Lexer::new(s)) {
                Ok(ast) => {
                    let r = eval(&ast);
                    for a in &answers {
                        assert_eq!(r, *a);
                    }
                    assert_eq!(ast.tok, Token::Op('+'));
                    assert_eq!(ast.children[0].tok, Int(5));
                    assert_eq!(ast.children[1].tok, Token::Op('/'));
                    assert_eq!(ast.children[1].children[0].tok, Token::Op('*'));
                    assert_eq!(ast.children[1].children[0].children[0].tok, Token::Int(3));
                    assert_eq!(ast.children[1].children[0].children[1].tok, Token::Int(3));
                    assert_eq!(ast.children[1].children[1].tok, Token::Int(6));
                }
                Err(msg) => panic!(msg),
            }
        }
        match expr(&mut Lexer::new("1+1+1+1")) {
            Ok(ast) => {
                assert_eq!(eval(&ast), 4.0);
            }
            Err(msg) => panic!(msg),
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
