// currently handles single variable equations
// TODO: seperate floating-point and fixed-point specific content into seperate files

use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Equation<T> {
    lhs: String,
    rhs: Box<Expr<T>>,
}

impl<T> Equation<T> {
    pub fn new(lhs: String, rhs: Box<Expr<T>>) -> Self {
        Self { lhs, rhs }
    }

    pub fn lhs(&self) -> &str {
        &self.lhs
    }

    pub fn rhs(&self) -> &Expr<T> {
        &self.rhs
    }
}

impl<T:Display> fmt::Display for Equation<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)
    }
}

#[derive(Clone)]
pub enum Expr<T> {
    Const(T),
    Var(String),
    Add(Box<Expr<T>>, Box<Expr<T>>),
    Sub(Box<Expr<T>>, Box<Expr<T>>),
    Mul(Box<Expr<T>>, Box<Expr<T>>),
    Div(Box<Expr<T>>, Box<Expr<T>>),
    Pow(Box<Expr<T>>, Box<Expr<T>>),
    Log(Box<Expr<T>>, Box<Expr<T>>),
    Neg(Box<Expr<T>>),
    Sin(Box<Expr<T>>),
    Cos(Box<Expr<T>>),
}

// TODO: special constants pi & e (in parse)

impl<T: Display> fmt::Display for Expr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expr::*;
        let s = match self {
            Const(c) => c.to_string(),
            Var(s) => s.to_owned(),
            Add(op1, op2) => format!("({} + {})", op1, op2),
            Sub(op1, op2) => format!("({} - {})", op1, op2),
            Mul(op1, op2) => format!("({} * {})", op1, op2),
            Div(op1, op2) => format!("({} / {})", op1, op2),
            Pow(op1, op2) => format!("({}^{})", op1, op2),
            Log(op1, op2) => format!("log({}, {})", op1, op2),
            Neg(op) => format!("-{}", op),
            Sin(op) => format!("sin({})", op),
            Cos(op) => format!("cos({})", op),
        };

        write!(f, "{}", s)
    }
}
