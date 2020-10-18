#![allow(dead_code)]

use crate::ast::Ast;
use crate::lex::{Lexer, Token};

type AstRes = Result<Ast, String>;

/// Parse a raw string and return the abstract syntax tree.
///
/// # Examples
///
/// ```
/// let ast = parse("2 + (4 * 3 / 2)");
/// let result = eval(&ast);
/// ```
pub fn parse(text: &str) -> AstRes {
    let mut l = Lexer::new(text);
    parse_expr(&mut l)
}

fn parse_expr_tokens(toks: Vec<Token>) -> AstRes {
    let mut trees: Vec<Ast> = vec![];
}

/**
 * Grammar: (see http://www.allisons.org/ll/ProgLang/Grammar/Top-Down/)
 *
 * expr   ::= expr + term |
 *            expr - term |
 *            term
 * term   ::= term * factor |
 *            term / factor |
 *            factor
 * factor ::= 0-9      |
 *            ( expr ) |
 *            - factor
 */

fn parse_expr(stream: &mut Lexer) -> AstRes {
    let mut left: Vec<Token> = vec![];
    let mut i = 0;
    let op = loop {
        let next = stream.look_ahead(i);
        match next {
            Token::End => {
                println!("only term");
                return parse_term(stream);
            }
            Token::Op('+') | Token::Op('-') => {
                stream.discard(i + 1);
                left.push(Token::End);
                println!("left vec {} {:?}", i, left);
                break next;
            }
            _ => {
                left.insert(0, next);
                i = i + 1;
            }
        }
    };

    let right = parse_term(stream)?;
    println!("op: {:?}, right term: {} {:?}", op, right, left);
    if left.len() == 0 {
        println!("no left term");
        return Ok(right);
    }
    let sub_expr = parse_expr(&mut Lexer::from(left))?;
    println!("left expr: {}", sub_expr);

    if sub_expr.tok == Token::End {
        Ok(Ast::from(op, vec![right]))
    } else {
        Ok(Ast::from(op, vec![sub_expr, right]))
    }
}

fn parse_term(stream: &mut Lexer) -> AstRes {
    let mut left = vec![];
    let mut i = 0;
    let op = loop {
        let next = stream.look_ahead(i);

        if next == Token::End {
            println!("only factor");
            return parse_factor(stream);
        } else if next == Token::Op('*') || next == Token::Op('/') {
            stream.discard(i + 1); // skip the operation
            break next;
        } else if next == Token::CloseParen {
            // stream.discard(i);
            // return parse_term(stream);
        }
        left.insert(0, next);
        i = i + 1;
    };
    let mut left_stream = Lexer::from(left);
    let sub_term = parse_term(&mut left_stream)?;
    let root = Ast::from(op, vec![sub_term, parse_factor(stream)?]);
    println!("term result: {}", root);
    Ok(root)
}

fn _parse_expr(stream: &mut Lexer) -> AstRes {
    let head = match stream.look_ahead(1) {
        Token::End => Ast::new(stream.next().unwrap()),
        // this is the case where there is only a term,
        // no op followed by a term.
        _ => parse_term(stream)?,
    };

    let op = stream.next().unwrap_or(Token::End);
    match op {
        Token::End => Ok(head),
        // Token::OpenParen | Token::CloseParen => {
        //     println!("paren in parse_expr ----------------------------");
        //     Ok(head)
        // }
        // this is the case where there is an operation
        // followed by a term.
        Token::Op(c) => match c {
            '/' | '*' | '+' | '-' => {
                let term = parse_term(stream)?;
                Ok(Ast::from(op, vec![head, term]))
            }
            _ => panic!("i dont know what to do with this"),
        },
        _ => Err(String::from("expected + or - operation")),
    }
}

// fn term_is_next(stream: &mut Lexer) -> bool {
//     return false;
// }

fn _parse_term(stream: &mut Lexer) -> AstRes {
    let head = match stream.look_ahead(1) {
        // we have reched the end of an expression, must return
        Token::CloseParen => return Ok(Ast::new(stream.next().unwrap())),
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
            vec![head, parse_factor(stream)?],
        )),
    }
}

fn parse_factor(stream: &mut Lexer) -> AstRes {
    let head = stream.next().unwrap_or(Token::End);

    match head {
        Token::End => Ok(Ast::new(head)),
        Token::Int(..) | Token::Float(..) => Ok(Ast::new(head)),
        Token::OpenParen => {
            let expr = parse_expr(stream)?;
            println!("( {} ) {:?}", expr, stream.peek());
            match stream.peek() {
                Token::CloseParen => {
                    stream.next();
                    Ok(expr)
                }
                Token::End => Ok(expr),
                _ => Err(format!("expected ')'")),
            }
            // match stream.next().unwrap_or(Token::End) {
            //     Token::CloseParen => expr,
            //     _ => {
            //         println!("open paren ?: {}", expr.unwrap());
            //         Err(String::from("expected ')'"))
            //     }
            // }
        }
        // only for negatives
        Token::Op(c) => match c {
            '-' => Ok(Ast::from(head, vec![parse_factor(stream)?])),
            _ => Err(format!("invlaid operation '{}'", c)),
        },
        _ => Ok(Ast::new(head)),
    }
}

#[cfg(test)]
mod test {
    use super::{parse, parse_expr, parse_factor};
    use crate::ast::eval;
    use crate::lex::{Lexer, Token};

    #[test]
    fn test_parse_factor() {
        let mut l = Lexer::new("-4");
        match parse_factor(&mut l) {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('-'));
                assert_eq!(ast.children[0].tok, Token::Int(4));
                assert_eq!(eval(&ast), -4.0);
            }
            Err(msg) => panic!(msg),
        }
        match parse("(-3)") {
            Ok(ast) => {
                println!("should be -3");
                println!("{}", ast);
                assert_eq!(eval(&ast), -3.0);
                // assert_eq!(ast.tok, Token::OpenParen);
            }
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn test_lex_buffer() {
        let mut l = Lexer::from(vec![
            Token::Int(5),
            Token::Op('+'),
            Token::Int(335),
            Token::Op('*'),
            Token::OpenParen,
            Token::Float(1.5),
            Token::Op('+'),
            Token::Int(1),
            Token::CloseParen,
        ]);
        match parse_expr(&mut l) {
            Ok(ast) => {
                /*
                 *   5.0 + 335.0 * (1.5 + 1.0)
                 *
                 *        +
                 *     /     \
                 *   5         *
                 *            /  \
                 *         335    +
                 *              /   \
                 *            1.5    1
                 */

                assert_eq!(eval(&ast), (5.0 + 335.0 * (1.5 + 1.0)));
                assert_eq!(ast.tok, Token::Op('+'));
                assert_eq!(ast.children[0].tok, Token::Int(5));
                let ast = &ast.children[1];
                assert_eq!(ast.tok, Token::Op('*'));
                assert_eq!(ast.children[0].tok, Token::Int(335));
                assert_eq!(ast.children[1].tok, Token::Op('+'));
                assert_eq!(ast.children[1].children[0].tok, Token::Float(1.5));
                assert_eq!(ast.children[1].children[1].tok, Token::Int(1));
            }
            Err(msg) => panic!(msg),
        }
    }
}
