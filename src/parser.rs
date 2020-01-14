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


pub struct Ast {

}

struct Expr {

}

struct Term {

}

pub fn parse_factor(toks: &mut Lexer) -> i64 {
    0
}

#[allow(dead_code)]
fn parse_expr(toks: &mut Lexer) -> Expr {
    match toks.peek() {

    }
}
