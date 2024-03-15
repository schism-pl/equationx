// currently handles single variable equations
// TODO: seperate floating-point and fixed-point specific content into seperate files

use std::fmt::{self, Display};

/// A valid `Piecewise` function requires that all the intervals in cases are
/// non-overlapping and ordered, and terminate with an "otherwise" case.
#[derive(Clone)]
pub struct Piecewise<T> {
    pub(crate) lhs: String,
    // TODO: Make this into a struct instead of a tuple
    pub(crate) cases: Vec<(CaseCondition<T>, Box<Expr<T>>)>,
}

// TODO: Well-formedness check
// TODO: Ends of range for fixpoint (no explicit infinities)
impl<T> Piecewise<T> {
    pub fn new(lhs: String, cases: Vec<(CaseCondition<T>, Box<Expr<T>>)>) -> Self {
        Piecewise { lhs, cases }
    }
}

impl<T: Display> fmt::Display for Piecewise<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let eqns = self
            .cases
            .iter()
            .map(|(interval, expr)| format!("  {} if {}", expr, interval).to_string())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}{{\n{}\n}}", self.lhs, eqns)
    }
}

#[derive(Clone, PartialEq)]
pub enum CaseCondition<T> {
    Otherwise,
    Interval(Interval<T>),
}

impl<T: Display> fmt::Display for CaseCondition<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CaseCondition::Otherwise => write!(f, "otherwise"),

            CaseCondition::Interval(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Interval<T> {
    pub(crate) low_val: T,
    pub(crate) high_val: T,
    pub(crate) low_openness: Openness,
    pub(crate) high_openness: Openness,
}

impl<T> Interval<T> {
    pub fn new(low_val: T, high_val: T, low_openness: Openness, high_openness: Openness) -> Self {
        Interval {
            low_val,
            high_val,
            low_openness,
            high_openness,
        }
    }
}

impl<T: Display> fmt::Display for Interval<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (left_c, right_c) = match (self.low_openness, self.high_openness) {
            (Openness::Closed, Openness::Closed) => ("[", "]"),
            (Openness::Closed, Openness::Open) => ("[", ")"),
            (Openness::Open, Openness::Closed) => ("(", "]"),
            (Openness::Open, Openness::Open) => ("(", ")"),
        };
        write!(
            f,
            "{}{}, {}{}",
            left_c, self.low_val, self.high_val, right_c
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Openness {
    Open,
    Closed,
}

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

impl<T: Display> fmt::Display for Equation<T> {
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
