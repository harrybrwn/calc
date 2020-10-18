use calc::ast::eval;
use calc::lex::Token;
use calc::parser::parse;

#[test]
fn parse_factor() {
    match parse("-3") {
        Ok(ast) => {
            assert_eq!(ast.tok, Token::Op('-'));
            assert_eq!(ast.children[0].tok, Token::Int(3));
        }
        Err(msg) => panic!(msg),
    };

    let ast = match parse("(3)") {
        Ok(ast) => ast,
        Err(msg) => panic!(msg),
    };
    assert_eq!(ast.tok, Token::Int(3));
}

#[test]
fn test_parser() {
    let ast = parse("1+1");
    match ast {
        Ok(ast) => {
            assert_eq!(ast.tok, Token::Op('+'));
            assert_eq!(ast.children[0].tok, Token::Int(1));
            assert_eq!(ast.children[1].tok, Token::Int(1));
        }
        Err(msg) => panic!(msg),
    }

    let ast = parse("9.4 / 5");
    match ast {
        Ok(ast) => {
            assert_eq!(ast.tok, Token::Op('/'));
            assert_eq!(ast.children[0].tok, Token::Float(9.4));
            assert_eq!(ast.children[1].tok, Token::Int(5));
        }
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
        }
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
        }
        Err(msg) => panic!(msg),
    }

    match parse("3 / 2 + 1") {
        Ok(ast) => {
            assert_eq!(ast.tok, Token::Op('+'));
            assert_eq!(ast.children[0].tok, Token::Op('/'));
        }
        Err(msg) => panic!(msg),
    }
}

#[test]
fn test_nested_div() {
    match parse("3/2/4/7") {
        Ok(ast) => {
            assert_eq!(ast.children[1].tok, Token::Int(7));
            assert_eq!(eval(&ast), (3.0 / 2.0 / 4.0 / 7.0));
        }
        Err(msg) => panic!(msg),
    }
    match parse("3/2/4") {
        Ok(ast) => {
            // println!("ast: {} => {}", ast, eval(&ast));
            assert_eq!(ast.tok, Token::Op('/'));
            assert_eq!(ast.children[1].tok, Token::Int(4));
            assert_eq!(ast.children[0].tok, Token::Op('/'));
            assert_eq!(ast.children[0].children[0].tok, Token::Int(3));
            assert_eq!(ast.children[0].children[1].tok, Token::Int(2));
            assert_eq!(eval(&ast), (3.0 / 2.0 / 4.0));
        }
        Err(msg) => panic!(msg),
    }
    match parse("4/5/6/7") {
        Ok(ast) => {
            assert_eq!(ast.tok, Token::Op('/'));
            assert_eq!(ast.children[0].tok, Token::Op('/'));
            assert_eq!(ast.children[0].children[0].tok, Token::Op('/'));
            assert_eq!(ast.children[0].children[0].children[0].tok, Token::Int(4));
            assert_eq!(ast.children[0].children[0].children[1].tok, Token::Int(5));
            assert_eq!(ast.children[0].children[1].tok, Token::Int(6));
            assert_eq!(ast.children[1].tok, Token::Int(7));
        }
        Err(msg) => panic!(msg),
    }
}

#[test]
fn test_paren_groups() {
    match parse("(1+2)") {
        Ok(ast) => {
            println!("result: {}", ast);
            println!("result inner: {}", ast.children[1]);
            assert_eq!(ast.tok, Token::Op('+'));
            assert_eq!(ast.children[0].tok, Token::Int(1));
            assert_eq!(ast.children[1].tok, Token::Int(2));
        }
        Err(msg) => panic!(msg),
    }

    // match parse("4 + (1 - 5)") {
    //     Ok(ast) => {
    //         assert_eq!(ast.tok, Token::Op('+'));
    //         assert_eq!(ast.children[0].tok, Token::Int(4));
    //         let ast = &ast.children[1];
    //         assert_eq!(ast.tok, Token::Op('-'));
    //         assert_eq!(ast.children[0].tok, Token::Int(1));
    //         assert_eq!(ast.children[1].tok, Token::Int(5));
    //     }
    //     Err(msg) => panic!(msg),
    // }
    // match parse("4 * (1 - 5)") {
    //     Ok(ast) => {
    //         assert_eq!(ast.tok, Token::Op('*'));
    //         assert_eq!(ast.children[0].tok, Token::Int(4));
    //         let ast = &ast.children[1];
    //         assert_eq!(ast.tok, Token::Op('-'));
    //         assert_eq!(ast.children[0].tok, Token::Int(1));
    //         assert_eq!(ast.children[1].tok, Token::Int(5));
    //     }
    //     Err(msg) => panic!(msg),
    // }
    // match parse("( 1-   5) *4") {
    //     Ok(ast) => {
    //         println!("{}", ast);
    //         assert_eq!(ast.tok, Token::Op('*'));
    //     }
    //     Err(msg) => panic!(msg),
    // }
    // match parse("(1+4*5)-5") {
    //     Ok(ast) => {
    //         assert_eq!(eval(&ast), ((1.0 + 4.0 * 5.0) - 5.0));
    //     }
    //     Err(msg) => panic!(msg),
    // }
}

#[test]
fn test_complex_expr() {
    match parse("(1+4*5)") {
        Ok(ast) => assert_eq!(ast.tok, Token::Op('+')),
        Err(msg) => panic!(msg),
    }

    match parse("(1+4*5)-5") {
        Ok(ast) => {
            assert_eq!(eval(&ast), ((1 + 4 * 5) - 5) as f64);

            assert_eq!(ast.tok, Token::Op('-'));
            assert_eq!(ast.children[1].tok, Token::Int(5));
            let ast = &ast.children[0];
            assert_eq!(ast.tok, Token::Op('+'));
            assert_eq!(ast.children[0].tok, Token::Int(1));
            assert_eq!(ast.children[1].children[0].tok, Token::Int(4));
            assert_eq!(ast.children[1].children[1].tok, Token::Int(5));
        }
        Err(msg) => panic!(msg),
    }

    match parse("5-(1+4*5") {
        Ok(..) => panic!("expected an error here"),
        Err(..) => (),
    }
}

#[test]
fn test_multi_mul() {
    match parse("1*2*3*4") {
        Ok(ast) => {
            assert_eq!(ast.tok, Token::Op('*'));
            assert_eq!(ast.children.len(), 2);
            assert_eq!(ast.children[0].tok, Token::Op('*'));
            assert_eq!(ast.children[0].children.len(), 2);
            assert_eq!(ast.children[0].children[0].tok, Token::Int(1));
            assert_eq!(ast.children[0].children[1].tok, Token::Int(2));
            assert_eq!(ast.children[1].tok, Token::Op('*'));
            assert_eq!(ast.children[1].children.len(), 2);
            assert_eq!(ast.children[1].children[0].tok, Token::Int(3));
            assert_eq!(ast.children[1].children[1].tok, Token::Int(4));
            assert_eq!(eval(&ast), 24 as f64);
        }
        Err(msg) => panic!(msg),
    }
}

#[test]
fn test_eval() {
    assert_eq!(eval(&parse("-(1 + 1)").unwrap()), -2.0);
    assert_eq!(eval(&parse("-5").unwrap()), -5.0);

    assert_eq!(eval(&parse("(1+4*5)-5").unwrap()), ((1 + 4 * 5) - 5) as f64);

    assert_eq!(eval(&parse("4/(3-1)*5").unwrap()), (4 / (3 - 1) * 5) as f64);
    assert_eq!(eval(&parse("(3-1)*5+1").unwrap()), ((3 - 1) * 5 + 1) as f64);
    assert_eq!(eval(&parse("2/2").unwrap()), (2.0 / 2.0) as f64);
    assert_eq!(eval(&parse("1/3").unwrap()), (1.0 / 3.0) as f64);
    assert_eq!(eval(&parse("2/2/3").unwrap()), (2.0 / 2.0 / 3.0) as f64);
    assert_eq!(eval(&parse("2/2/3").unwrap()), (2.0 / 2.0 / 3.0) as f64);
    assert_eq!(eval(&parse("4/5/6/7").unwrap()), 4.0 / 5.0 / 6.0 / 7.0);
    assert_eq!(
        eval(&parse("3/3/4/5/6").unwrap()),
        (3.0 / 3.0 / 4.0 / 5.0 / 6.0)
    );

    // let testcase = "1*1*1 + 1";
    // assert_eq!(eval(&parse(testcase).unwrap()), ( 1*1*1 + 1 ) as f64);
    // assert_eq!(parse(testcase).unwrap().tok, Token::Op('+'));
    // match parse(testcase) {
    //     Ok(ast) => {
    //         assert_eq!(eval(&parse("1*1*1 + 1").unwrap()), 1.0*1.0*1.0 + 1.0);
    //     },
    //     Err(msg) => panic!(msg),
    // }
    // let testcase = "4/(3-1)*5+1";
    // assert_eq!(eval(&parse(testcase).unwrap()), ( 4 / (3 - 1) * 5 + 1 ) as f64);
    // assert_eq!(parse(testcase).unwrap().tok, Token::Op('+'));
    // match parse(testcase) {
    //     Ok(ast) => {
    //         println!("{}", ast);
    //         println!("eval: {}, want: {}", eval(&ast), 4/(3-1)*5+1);
    //     },
    //     Err(msg) => panic!(msg),
    // }
}
