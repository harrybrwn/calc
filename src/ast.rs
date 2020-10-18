use std::fmt;
use std::ops;

use crate::lex::Token;

pub struct Ast {
    pub tok: Token,
    pub children: Vec<Ast>,
}

pub fn eval(ast: &Ast) -> f64 {
    if ast.children.len() == 1 {
        return match ast.tok {
            Token::Op('-') => -1.0 * eval(&ast.children[0]),
            _ => panic!("not enough arguments"),
        };
    }

    match ast.tok {
        Token::Op(c) => {
            let left = ast.children[0].clone();
            let right = &ast.children[1];
            match c {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => panic!("invalid operation"),
            }
        }
        Token::Int(n) => n as f64,
        Token::Float(n) => n,
        _ => 0.0,
    }
}

impl Clone for Ast {
    fn clone(&self) -> Self {
        Self {
            tok: self.tok,
            children: self.children.clone(),
        }
    }
}

impl Ast {
    pub fn new(t: Token) -> Self {
        Self {
            tok: t,
            children: vec![],
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
        match (self.tok, ast.tok) {
            (Token::Op('/'), Token::Op('/')) => {
                self.children.push(ast);
                self.rotate_left();
            }
            _ => self.children.push(ast),
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

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
