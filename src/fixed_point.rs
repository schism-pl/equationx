use fixed::{types::extra::LeEqU64, FixedI64};
use crate::ast::{Equation, Expr};
use cordic::CordicNumber;

pub trait CordicMarker {}
impl<Frac> CordicMarker for fixed::FixedI8<Frac> {}
impl<Frac> CordicMarker for fixed::FixedI16<Frac> {}
impl<Frac> CordicMarker for fixed::FixedI32<Frac> {}
impl<Frac> CordicMarker for fixed::FixedI64<Frac> {}

impl<T: CordicNumber + CordicMarker> Expr<T> {
    pub fn eval(&self, arg: T) -> T {
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
            Sin(op) => cordic::sin(op.eval(arg)),
            Cos(op) => cordic::cos(op.eval(arg)),
        }
    }

    // pub fn to_rust(&self) -> String {
    //     use Expr::*;
    //     match self {
    //         Const(c) => format!("{}_f64", c),
    //         Var(s) => s.to_owned(),
    //         Add(op1, op2) => format!("({} + {})", op1.to_rust(), op2.to_rust()),
    //         Sub(op1, op2) => format!("({} - {})", op1.to_rust(), op2.to_rust()),
    //         Mul(op1, op2) => format!("({} * {})", op1.to_rust(), op2.to_rust()),
    //         Div(op1, op2) => format!("({} / {})", op1.to_rust(), op2.to_rust()),
    //         Pow(op1, op2) => format!("{}.powf({})", op1.to_rust(), op2.to_rust()),
    //         Log(op1, op2) => format!("{}.log({})", op1.to_rust(), op2.to_rust()),
    //         Neg(op) => format!("-{}", op.to_rust()),
    //         Sin(op) => format!("{}.sin()", op.to_rust()),
    //         Cos(op) => format!("{}.cos()", op.to_rust()),
    //     }
    // }
}