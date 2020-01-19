use calc::parser::{parse, eval};
use calc::lex::Token;

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

#[test]
fn test_multi_mul() {
    match parse("1*2*3+1") {
        Ok(ast) => {
            println!("{}", ast);
            println!("{} {}", ast.children[0], ast.children[1]);
            assert_eq!(ast.tok, Token::Op('+'));
        },
        Err(msg) => panic!(msg),
    }
}

#[test]
fn test_eval() {
    assert_eq!(eval(&parse("-(1 + 1)").unwrap()), -2.0);
    assert_eq!(eval(&parse("-5").unwrap()), -5.0);
    assert_eq!(eval(&parse("(1+4*5)-5").unwrap()), ( (1+4*5)-5 ) as f64);
    assert_eq!(eval(&parse("4/(3-1)*5").unwrap()), ( 4/(3-1)*5 ) as f64);
    assert_eq!(eval(&parse("(3-1)*5+1").unwrap()), ( (3-1)*5+1 ) as f64);

    let testcase = "1*1*1 + 1";
    assert_eq!(eval(&parse(testcase).unwrap()), ( 1*1*1 + 1 ) as f64);
    assert_eq!(parse(testcase).unwrap().tok, Token::Op('+'));
    match parse(testcase) {
        Ok(ast) => {
            println!("{}", ast);
            println!("{} {}", ast.children[0], ast.children[1]);
            println!("eval: {}, want: {}", eval(&ast), 1*1*1 + 1);
        },
        Err(msg) => panic!(msg),
    }

    let testcase = "4/(3-1)*5+1";
    assert_eq!(eval(&parse(testcase).unwrap()), ( 4 / (3 - 1) * 5 + 1 ) as f64);
    assert_eq!(parse(testcase).unwrap().tok, Token::Op('+'));
    match parse(testcase) {
        Ok(ast) => {
            println!("{}", ast);
            println!("eval: {}, want: {}", eval(&ast), 4/(3-1)*5+1);
        },
        Err(msg) => panic!(msg),
    }
}
