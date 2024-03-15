use crate::ast::{CaseCondition, Equation, Expr, Openness, Piecewise};
use anyhow::{anyhow, Result};

impl Piecewise<f64> {
    pub fn well_formed(&self) -> Result<()> {
        if self.cases.len() == 0 {
            return Err(anyhow!("Malformed piecewise function: missing any cases"));
        }

        let mut last_high = f64::NEG_INFINITY;
        let mut last_openness = Openness::Open;
        for cond in &self.cases[..self.cases.len() - 1] {
            if let CaseCondition::Interval(i) = &cond.0 {
                match i.low_openness {
                    Openness::Open => {
                        if i.low_val < last_high {
                            return Err(anyhow!(format!(
                                "Malformed piecewise function: overlapping interval {}",
                                i
                            )
                            .to_string()));
                        }
                    }
                    Openness::Closed => match last_openness {
                        Openness::Closed => {
                            if i.low_val <= last_high {
                                return Err(anyhow!(format!(
                                    "Malformed piecewise function: overlapping interval {}",
                                    i
                                )
                                .to_string()));
                            }
                        }
                        Openness::Open => {
                            if i.low_val < last_high {
                                return Err(anyhow!(format!(
                                    "Malformed piecewise function: overlapping interval {}",
                                    i
                                )
                                                   .to_string()));
                            }
                        }
                    },
                }

                last_high = i.high_val;
                last_openness = i.high_openness;
            } else {
                return Err(anyhow!("Malformed piecewise function: \"otherwise\" should only be the last piecewise case"));
            }
        }

        if self.cases[self.cases.len() - 1].0 != CaseCondition::Otherwise {
            return Err(anyhow!("Malformed piecewise function: the last csaes should always be \"otherwise\""));
        }

        Ok(())
    }

    pub fn eval(&self, arg: f64) -> f64 {
        for case in &self.cases[..self.cases.len() - 1] {
            if let CaseCondition::Interval(i) = &case.0 {
                match (i.low_openness, i.high_openness) {
                    (Openness::Closed, Openness::Closed) => {
                        if arg >= i.low_val && arg <= i.high_val {
                            return case.1.eval(arg);
                        }
                    }
                    (Openness::Closed, Openness::Open) => {
                        if arg >= i.low_val && arg < i.high_val {
                            return case.1.eval(arg);
                        }
                    }
                    (Openness::Open, Openness::Closed) => {
                        if arg > i.low_val && arg <= i.high_val {
                            return case.1.eval(arg);
                        }
                    }
                    (Openness::Open, Openness::Open) => {
                        if arg > i.low_val && arg < i.high_val {
                            return case.1.eval(arg);
                        }
                    }
                }
            }
        }

        // fall through to default case
        self.cases[self.cases.len() - 1].1.eval(arg)
    }
}

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
