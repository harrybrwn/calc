use std::fmt;
use std::ops;
use std::str;

use crate::lex::Token;
use crate::parser;

pub struct Ast {
    pub tok: Token,
    pub children: Vec<Ast>,
    grouped: bool,
}

#[doc(hidden)]
pub fn eval(ast: &Ast) -> f64 {
    match ast.children.len() {
        // numeric types
        0 => match ast.tok {
            Token::Int(n) => n as f64,
            Token::Float(f) => f,
            _ => panic!("zero children should be numeric"),
        },
        // unary operators
        1 => match ast.tok {
            Token::Op('-') => -1.0 * eval(&ast.children[0]),
            Token::Op('%') => eval(&ast.children[0]) / 100.0,
            _ => panic!("invalid unary operator"),
        },
        // binary operators
        2 => match ast.tok {
            Token::Op(c) => {
                let left = ast.children[0].clone();
                let right = &ast.children[1];
                match c {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    '^' => {
                        let base = eval(&ast.children[0]);
                        return base.powf(eval(right));
                    }
                    '%' => (eval(&ast.children[0]) / 100.0) * eval(&ast.children[1]),
                    _ => panic!("invalid binary operation"),
                }
            }
            Token::Modulus => eval(&ast.children[0]) % eval(&ast.children[1]),
            _ => panic!("invalid binary operator"),
        },
        // ternary operators
        3 => 0.0,
        _ => 0.0,
    }
}

impl str::FromStr for Ast {
    type Err = String;

    /// Constructs an expression by parsing a string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

impl<'a> Clone for Ast {
    fn clone(&self) -> Self {
        Self {
            tok: self.tok,
            children: self.children.clone(),
            grouped: self.grouped,
        }
    }
}

impl<'a> Ast {
    pub fn new(t: Token) -> Self {
        Self {
            tok: t,
            children: vec![],
            grouped: false,
        }
    }

    pub fn from(t: Token, children: Vec<Ast>) -> Self {
        let mut ast = Self::new(t);
        for c in children {
            ast.push(c);
        }
        ast
    }

    pub fn push(&mut self, ast: Ast) {
        use Token::{Modulus, Op};
        let grouped = ast.grouped;
        let tok = ast.tok;

        self.children.push(ast);

        // if the ast has been marked as
        // a group then we don't want to
        // break that group
        if grouped {
            return;
        }
        // we only do rotation if we
        // already have a left child
        if self.children.len() == 0 {
            return;
        }

        match (self.tok, tok) {
            (Op('+'), ..) | (Op('-'), ..) | (.., Op('+')) | (.., Op('-')) => {}
            (Op('/'), Op('/'))
            | (Op('/'), Modulus)
            | (Op('/'), Op('*'))
            | (Op('*'), Op('*'))
            | (Op('*'), Op('/'))
            | (Op('*'), Modulus)
            | (Op('^'), Op('*'))
            | (Op('^'), Op('/'))
            | (Op('^'), Modulus)
            | (Op('%'), Op('*'))
            | (Op('%'), Op('/'))
            | (Modulus, Modulus)
            | (Modulus, Op('*'))
            | (Modulus, Op('/')) => {
                self.rotate_left();
            }
            _ => {}
        }
    }

    fn rotate_left(&mut self) {
        if self.children.len() < 2 {
            return;
        }
        let newroot = self.children[1].tok;
        self.children[0] = Ast::from(
            self.tok,
            vec![
                self.children[0].clone(),
                self.children[1].children[0].clone(),
            ],
        );
        self.children[1] = self.children[1].children[1].clone();
        self.tok = newroot;
    }

    pub fn as_grouped(&self) -> Self {
        let mut ast = self.clone();
        ast.grouped = true;
        ast
    }
}

impl<'a> ops::Add<&'a Ast> for Ast {
    type Output = f64;
    fn add(self, rhs: &'a Ast) -> Self::Output {
        eval(&self) + eval(rhs)
    }
}

impl<'a> ops::Sub<&'a Ast> for Ast {
    type Output = f64;
    fn sub(self, rhs: &'a Ast) -> Self::Output {
        eval(&self) - eval(rhs)
    }
}

impl<'a> ops::Mul<&'a Ast> for Ast {
    type Output = f64;
    fn mul(self, rhs: &'a Ast) -> Self::Output {
        eval(&self) * eval(rhs)
    }
}

impl<'a> ops::Div<&'a Ast> for Ast {
    type Output = f64;
    fn div(self, rhs: &'a Ast) -> Self::Output {
        eval(&self) / eval(rhs)
    }
}

fn _format(ast: &Ast, f: &mut fmt::Formatter) -> fmt::Result {
    let len = ast.children.len();
    write!(f, "Ast({:?}: [", ast.tok)?;
    if len > 0 {
        for i in 0..(len - 1) {
            write!(f, "{:?}, ", ast.children[i].tok)?;
        }
        write!(f, "{:?}])", ast.children[len - 1].tok)
    } else {
        write!(f, "])")
    }
}

impl<'a> fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        _format(self, f)
    }
}

impl<'a> fmt::Debug for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let len = self.children.len();
        write!(f, "Ast({:?}: [", self.tok)?;
        if len > 0 {
            for i in 0..(len - 1) {
                write!(f, "{:?}, ", self.children[i].tok)?;
            }
            write!(f, "{:?}])", self.children[len - 1].tok)
        } else {
            write!(f, "])")
        }
    }
}
