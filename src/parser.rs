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
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };
    if toks.len() == 0 {
        return Ok(head);
    }
    let next = toks.remove(0);
    let mut root = match next {
        Token::Op(c) => match c {
            '+' | '-' => Ast::new(Token::Op(c)),
            _ => return Err(format!("invalid operation")),
        },
        _ => {
            println!("expr: {}", head);
            return Ok(head);
        }
    };
    let sub_expr = match expr(toks) {
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };
    root.push(head);
    root.push(sub_expr);
    Ok(root)
    // Ok(Ast::new(toks[0]))
}

fn term(toks: &mut Vec<Token>) -> AstRes {
    let res = match factor(toks) {
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };
    if toks.len() == 0 {
        return Ok(res);
    }
    let next = toks.remove(0);
    let mut root = match next {
        Token::Op(c) => match c {
            '/' | '*' => Ast::new(Token::Op(c)),
            _ => return Err(format!("invalid operation")),
        },
        // Token::End => return Ok(res),
        _ => {
            // println!("left: {}, next: {:?}", res, next);
            if toks.len() == 0 {
                println!("just returning {}", res);
                return Ok(res);
            }
            match toks[0] {
                // TODO: check too see what operations will break this
                //       for now we are allowing all ops
                Token::Op(..) => Ast::new(toks.remove(0)),
                _ => {
                    println!("other rem: {:?}", toks);
                    return Ok(res);
                }
            }
        }
    };
    let right = match term(toks) {
        Ok(ast) => ast,
        Err(msg) => return Err(msg),
    };
    println!("root:  {}", root);
    println!("left:  {}", res);
    println!("right: {}", right);
    println!("rem:   {:?}", toks);
    root.push(res);
    root.push(right);
    Ok(root)
}

fn factor(toks: &mut Vec<Token>) -> AstRes {
    match toks[0] {
        Token::Int(..) | Token::Float(..) => Ok(Ast::new(toks.remove(0))),
        Token::OpenParen => {
            toks.remove(0);
            let mut exprtoks = until_oneof(toks, &[Token::CloseParen]);
            print!("expression tokens: {:?}  ", exprtoks);
            if exprtoks.pop().unwrap() != Token::CloseParen {
                println!("\nneed closing paren in {:?}", toks);
                return Err(format!("expected ')'"));
            } else {
                println!("ok found closing paren");
            }
            // println!("expression tokens: {:?}", exprtoks);
            toks.drain(0..exprtoks.len());
            expr(&mut exprtoks)
        }
        _ => Err(format!("invalid factor")),
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
    use crate::ast::Ast;
    use crate::lex::{Lexer, Token};
    use crate::parser::until_oneof;

    #[test]
    fn test_factor() {
        // TODO: add "((1))" as a test-case
        for s in vec!["1", "(1)"] {
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
        // match term(&mut Lexer::new("(1)").as_vec()) {
        //     Ok(ast) => {
        //         assert_eq!(ast.tok, Token::Int(1));
        //     }
        //     Err(msg) => panic!(msg),
        // }

        // match term(&mut Lexer::new("1*1").as_vec()) {
        //     Ok(ast) => {
        //         assert_eq!(ast.tok, Token::Op('*'));
        //         assert_eq!(ast.children[0].tok, Token::Int(1));
        //         assert_eq!(ast.children[1].tok, Token::Int(1));
        //     }
        //     Err(msg) => panic!(msg),
        // }
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
            // "1/2/3",
            // "1/2/(3)",
            // "1/(2)/3",
            // "(1)/2/3",
            // "(1/2)/3",
            "((1)/2)/3",
        ] {
            match term(&mut Lexer::new(s).as_vec()) {
                Ok(ast) => {
                    println!("{} => {}", s, ast);
                    assert_eq!(ast.tok, Token::Op('/'));
                    assert_eq!(ast.children[0].tok, Token::Op('/'));
                    assert_eq!(ast.children[0].children[0].tok, Token::Int(1));
                    assert_eq!(ast.children[0].children[1].tok, Token::Int(2));
                    assert_eq!(ast.children[1].tok, Token::Int(3));
                }
                Err(msg) => panic!(msg),
            }
        }

        // match term(&mut Lexer::new("3*2").as_vec()) {
        //     Ok(ast) => {
        //         println!("{}", ast);
        //     }
        //     Err(msg) => panic!(msg),
        // }
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
