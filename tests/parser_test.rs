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
fn test_eval() {
    // match parse("-(1 + 1)") {
    //     Ok(ast) => assert_eq!(eval(&ast), -2.0),
    //     Err(msg) => panic!(msg),
    // }
}