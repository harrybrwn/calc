use calc::lex::{lex, Lexer, Token};

// #[ignore]
#[test]
fn test_lexical() {
    let s = "1+2";
    let mut l = Lexer::new(s);
    assert_eq!(Token::End, l.look_ahead(5));
    assert_eq!(Token::End, l.look_ahead(10));
}

#[test]
fn test_lex() {
    let s = "5 + 335 * (1.5+1)";
    let expected = vec![
        Token::Int(5),
        Token::Op('+'),
        Token::Int(335),
        Token::Op('*'),
        Token::OpenParen,
        Token::Float(1.5),
        Token::Op('+'),
        Token::Int(1),
        Token::CloseParen,
    ];
    let res = lex(s);
    assert!(res.len() > 1);
    assert_eq!(res.len(), expected.len());

    for i in 0..expected.len() - 1 {
        assert_eq!(expected[i], res[i]);
    }

    let mut l = Lexer::new(s);
    loop {
        let p = l.peek().clone();
        let n = l.next().unwrap_or(Token::End);

        match (p, n) {
            (Token::Int(a), Token::Int(b)) => assert_eq!(a, b),
            (Token::Float(a), Token::Float(b)) => assert_eq!(a, b),
            (Token::Op(a), Token::Op(b)) => assert_eq!(a, b),
            (Token::OpenParen, Token::OpenParen) => (),
            (Token::CloseParen, Token::CloseParen) => (),
            (Token::End, Token::End) => break,
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
