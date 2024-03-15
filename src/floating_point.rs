use crate::ast::{Equation, Expr};

impl Equation<f64> {
    pub fn eval(&self, arg: f64) -> f64 {
        self.rhs().eval(arg)
    }
}

impl Expr<f64> {
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

    pub fn to_rust(&self) -> String {
        use Expr::*;
        match self {
            Const(c) => format!("{}_f64", c),
            Var(s) => s.to_owned(),
            Add(op1, op2) => format!("({} + {})", op1.to_rust(), op2.to_rust()),
            Sub(op1, op2) => format!("({} - {})", op1.to_rust(), op2.to_rust()),
            Mul(op1, op2) => format!("({} * {})", op1.to_rust(), op2.to_rust()),
            Div(op1, op2) => format!("({} / {})", op1.to_rust(), op2.to_rust()),
            Pow(op1, op2) => format!("{}.powf({})", op1.to_rust(), op2.to_rust()),
            Log(op1, op2) => format!("{}.log({})", op1.to_rust(), op2.to_rust()),
            Neg(op) => format!("-{}", op.to_rust()),
            Sin(op) => format!("{}.sin()", op.to_rust()),
            Cos(op) => format!("{}.cos()", op.to_rust()),
        }
    }
}
