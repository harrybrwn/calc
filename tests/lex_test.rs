use calc::lex::{Token, Lexer, lex};

#[test]
fn test_lex() {
    let s = "5 + 335 * (1.5+1)";
    let res = lex(s);
    match res[0] {
        Token::Int(x) => assert_eq!(x, 5),
        _ => panic!("should be a int token"),
    }
    match res[1] {
        Token::Op(x) => assert_eq!(x, '+'),
        _ => panic!("should be an op token"),
    }
    match res[2] {
        Token::Int(x) => assert_eq!(x, 335),
        _ => panic!("should be a int token"),
    }
    match res[3] {
        Token::Op(x) => assert_eq!(x, '*'),
        _ => panic!("should be an op"),
    }
    match res[4] {
        Token::OpenParen => assert!(true),
        _ => assert!(false),
    }
    match res[5] {
        Token::Float(x) => assert_eq!(x, 1.5f64),
        _ => panic!("should be float"),
    }
    match res[6] {
        Token::Op(c) => assert_eq!(c, '+'),
        Token::Int(x) => panic!("should not be number: {}", x),
        _ => panic!("should be op"),
    }
    match res[8] {
        Token::CloseParen => {},
        _ => panic!("should be closed paren"),
    }

    let mut l = Lexer::new(s);
    loop {
        let p = l.peek().clone();
        let n = l.next().unwrap_or(Token::End);

        // println!("{:?} {:?}", p, n);
        match (p, n) {
            (Token::Int(a), Token::Int(b))     => assert_eq!(a, b),
            (Token::Float(a), Token::Float(b)) => assert_eq!(a, b),
            (Token::Op(a), Token::Op(b))       => assert_eq!(a, b),
            (Token::OpenParen, Token::OpenParen) => (),
            (Token::CloseParen, Token::CloseParen) => (),
            (Token::End, Token::End)           => break,
            _ => panic!("tokens should be the same"),
        }
    }
}

#[test]
fn test_iter() {
    let mut l = Lexer::new("1+1");
    let p = l.peek().clone();
    assert_eq!(p, Token::Int(1));
    assert_eq!(l.look_ahead(0), p);
    assert_eq!(l.look_ahead(1), Token::Op('+'));
    assert_eq!(l.look_ahead(2), Token::Int(1));

    match l.peek().clone() {
        Token::Int(n) => assert_eq!(n, 1),
        _ => panic!("expected the number one"),
    }
    l.next();
    match l.peek().clone() {
        Token::Op(c) => assert_eq!(c, '+'),
        _ => panic!("expected '+'"),
    }
    l.next();
    match l.peek().clone() {
        Token::Int(n) => assert_eq!(n, 1),
        _ => panic!("expected number one"),
    }
}

#[test]
fn test_both_lexers() {
    let s = "1 + (3 / 2) * 4";
    let mut toks1 = vec![];

    for t in Lexer::new(s) {
        toks1.push(t);
    }
    let toks2 = lex(s);

    assert_eq!(toks1, toks2);
}