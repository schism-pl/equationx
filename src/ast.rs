// currently handles single variable equations
// TODO: minimize dependencies

use std::fmt;

pub struct Equation {
    lhs: String,
    rhs: Box<Expr>,
}

impl Equation {
    pub fn new(lhs: String, rhs: Box<Expr>) -> Self {
        Self { lhs, rhs }
    }

    pub fn eval(&self, arg: f64) -> f64 {
        self.rhs.eval(arg)
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)
    }
}

pub enum Expr {
    Const(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Log(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
}

// TODO: special constants pi & e (in parse)

impl Expr {
    pub fn eval(&self, arg: f64) -> f64 {
        use Expr::*;
        match self {
            Const(c) => *c,
            Var(_s) => arg, // TODO: actually look up variable
            Add(op1, op2) => op1.eval(arg) + op2.eval(arg),
            Sub(op1, op2) => op1.eval(arg) - op2.eval(arg),
            Mul(op1, op2) => op1.eval(arg) * op2.eval(arg),
            Div(op1, op2) => op1.eval(arg) / op2.eval(arg),
            Pow(op1, op2) => op1.eval(arg).powf(op2.eval(arg)),
            Log(op1, op2) => op1.eval(arg).log(op2.eval(arg)),
            Neg(op) => -op.eval(arg),
            Sin(op) => op.eval(arg).sin(),
            Cos(op) => op.eval(arg).cos(),
        }
    }
}

impl fmt::Display for Expr {
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
