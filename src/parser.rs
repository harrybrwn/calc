use crate::lex::Lexer;

// Grammar:
//
// expr   ::= expr + term |
//            expr - term |
//            term
// term   ::= term * factor |
//            term / factor |
//            factor
// factor ::= 0-9       |
//            ( expr ) |
//            - factor

#[allow(dead_code)]
pub struct Ast {

}

struct Expr {

}

#[allow(dead_code)]
impl Expr {
    pub fn new() -> Self {
        Self{}
    }
}

struct Term {

}

pub fn parse_factor(toks: &mut Lexer) -> i64 {
    0
}

#[allow(dead_code)]
fn parse_expr(toks: &mut Lexer) -> Expr {
    match toks.peek() {
        _ => Expr::new(),
    }
}
