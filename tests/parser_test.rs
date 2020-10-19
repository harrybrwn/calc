use calc::ast::eval;
use calc::parser::parse;

#[test]
fn test_eval() {
    assert_eq!(
        eval(&parse("3/(3/4/5)/6").unwrap()),
        3.0 / (3.0 / 4.0 / 5.0) / 6.0
    );
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
    let result = eval(&ast);
    assert_eq!(result as i32, 2 + (4 * 3 / 2));
}
