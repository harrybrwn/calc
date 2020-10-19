#[cfg(test)]
mod test {
    use super::parse;
    use super::{expr, factor, term, until_oneof};
    use crate::ast::eval;
    use crate::lex::{Lexer, Token};

    #[test]
    fn test_eval() {
        let t = match parse("3/(3/4/5)/6") {
            Ok(ast) => ast,
            Err(msg) => panic!(msg),
        };
        assert_eq!(eval(&t), 3.0 / (3.0 / 4.0 / 5.0) / 6.0);
        assert_eq!(eval(&parse("-(1 + 1)").unwrap()), -2.0);
        assert_eq!(eval(&parse("-5").unwrap()), -5.0);
        assert_eq!(eval(&parse("(1+4*5)-5").unwrap()), ((1 + 4 * 5) - 5) as f64);
        assert_eq!(eval(&parse("4/(3-1)").unwrap()), 4.0 / (3.0 - 1.0));
        assert_eq!(eval(&parse("(3-1)*5").unwrap()), (3.0 - 1.0) * 5.0);
        assert_eq!(eval(&parse("4/(3-1)*5").unwrap()), 4.0 / (3.0 - 1.0) * 5.0);
        assert_eq!(eval(&parse("(3-1)*5+1").unwrap()), (3.0 - 1.0) * 5.0 + 1.0);
        assert_eq!(eval(&parse("2/2").unwrap()), 2.0 / 2.0);
        assert_eq!(eval(&parse("1/3").unwrap()), 1.0 / 3.0);
        assert_eq!(eval(&parse("2/2/3").unwrap()), 2.0 / 2.0 / 3.0);
        assert_eq!(eval(&parse("2/2/3").unwrap()), 2.0 / 2.0 / 3.0);
        assert_eq!(eval(&parse("4/5/6/7").unwrap()), 4.0 / 5.0 / 6.0 / 7.0);
        assert_eq!(
            eval(&parse("3/3/4/5/6").unwrap()),
            3.0 / 3.0 / 4.0 / 5.0 / 6.0
        );
        let ast = match parse("2 + (4 * 3 / 2)") {
            Ok(ast) => ast,
            Err(msg) => panic!(msg),
        };
        assert_eq!(eval(&ast), 2.0 + (4.0 * 3.0 / 2.0));
    }

    #[test]
    fn test_parse() {
        let t = match parse("3/(3/4/5)/6") {
            Ok(ast) => ast,
            Err(msg) => panic!(msg),
        };
        assert_eq!(t.children[0].tok, Token::Op('/'));
        assert_eq!(t.children[1].tok, Token::Int(6));
        assert_eq!(t.children[0].children[0].tok, Token::Int(3));
        assert_eq!(t.children[0].children[1].children[0].tok, Token::Op('/'));
        assert_eq!(t.children[0].children[1].children[1].tok, Token::Int(5));
        let sub = &t.children[0].children[1].children[0];
        assert_eq!(sub.children[0].tok, Token::Int(3));
        assert_eq!(sub.children[1].tok, Token::Int(4));
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
                    assert_eq!(ast.tok, Token::Int(1));
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
            Ok(ast) => assert_eq!(ast.tok, Token::Int(1)),
            Err(msg) => panic!(msg),
        }
        match term(&mut Lexer::new("1*1")) {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('*'));
                assert_eq!(ast.children[0].tok, Token::Int(1));
                assert_eq!(ast.children[1].tok, Token::Int(1));
            }
            Err(msg) => panic!(msg),
        }

        for s in vec![
            // division
            ("1/2/3", '/'),
            ("1/2/(3)", '/'),
            ("(1)/2/3", '/'),
            ("(1/2)/3", '/'),
            ("1/(2)/3", '/'),
            ("((1)/2)/3", '/'),
            ("(((1/2/3)))", '/'),
            ("(((((1/2)/3))))", '/'),
            // multiplication
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
                    assert_eq!(ast.children[0].children[0].tok, Token::Int(1));
                    assert_eq!(ast.children[0].children[1].tok, Token::Int(2));
                    assert_eq!(ast.children[1].tok, Token::Int(3));
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
        match term(&mut Lexer::new("3*2*5")) {
            Ok(ast) => {
                assert_eq!(ast.tok, Token::Op('*'));
                assert_eq!(ast.children[0].tok, Token::Op('*'));
                assert_eq!(ast.children[0].children[0].tok, Token::Int(3));
                assert_eq!(ast.children[0].children[1].tok, Token::Int(2));
                assert_eq!(ast.children[1].tok, Token::Int(5));
            }
            Err(msg) => panic!(msg),
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
                    assert_eq!(ast.children[0].tok, Token::Int(5));
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
