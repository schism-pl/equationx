pub mod ast;
pub mod fixed_point;
pub mod floating_point;
pub mod util;

pub use crate::ast::{Equation, Expr};
use ast::{CaseCondition, Interval, Piecewise};
use equation::*;

use lalrpop_util::lalrpop_mod;
use std::str::FromStr;

lalrpop_mod!(
    #[allow(unused)]
    equation
);

// TODO: use approx for tests

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

// TODO: Make all these trait implementations macros
impl Serialize for CaseCondition<f64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for CaseCondition<f64> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        CaseConditionParser::new()
            .parse(s)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}

impl<'de> Deserialize<'de> for CaseCondition<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl Serialize for Interval<f64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Interval<f64> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        IntervalParser::new()
            .parse(s)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}

impl<'de> Deserialize<'de> for Interval<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl Serialize for Piecewise<f64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Piecewise<f64> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        PiecewiseParser::new()
            .parse(s)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}

impl<'de> Deserialize<'de> for Piecewise<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl Serialize for Equation<f64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Equation<f64> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        EquationParser::new()
            .parse(s)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}

impl<'de> Deserialize<'de> for Equation<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl Serialize for Expr<f64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Expr<f64> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        ExprParser::new()
            .parse(s)
            .map_err(|e| anyhow::anyhow!("{}", e))
            .map(|b| *b) // Box<Expr> -> Expr
    }
}

impl<'de> Deserialize<'de> for Expr<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

#[macro_export]
macro_rules! eqn {
    ($e: expr) => {
        stringify!($e)
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{CaseCondition, Equation, Interval, Openness, Piecewise},
        Expr,
    };
    use std::str::FromStr;

    #[test]
    fn basic() {
        let s = "x = 3.0 + 2.0 + 1.0";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(0.0), 6.0)
    }

    #[test]
    fn var1() {
        let s = "x = y";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(0.0), 0.0)
    }

    #[test]
    fn var2() {
        let s = "x = y0";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(0.0), 0.0)
    }

    #[test]
    fn polynomial1() {
        let s = "y = 3 * x + 1";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(1.0), 4.0)
    }

    #[test]
    fn polynomial2() {
        let s = "y = 3 * x^2 + 2*x + 1";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(2.0), 17.0)
    }

    #[test]
    fn assoc() {
        let s = "x = 3.0 - 2.0 - 1.0";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(0.0), 0.0)
    }

    #[test]
    fn trig() {
        let s = "x = sin(0.0) + cos(0.0)";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(0.0), 1.0)
    }

    #[test]
    fn log() {
        let s = "x = log(x, 2.0)";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(4.0), 2.0)
    }

    #[test]
    fn paren1() {
        let s = "x = (2.0)";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(4.0), 2.0)
    }

    #[test]
    fn paren2() {
        let s = "x = 3.0 - (2.0 - 1.0)";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(4.0), 2.0)
    }

    #[test]
    fn paren3() {
        let s = "x = (y + 1) * (y + 2)";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(2.0), 12.0)
    }

    #[test]
    fn paren4() {
        let s = "x = ((y + 1) * y) + 3";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(2.0), 9.0)
    }

    #[test]
    fn paren5() {
        let s = "x = (y) * (y)";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(2.0), 4.0)
    }

    #[test]
    fn paren6() {
        let s = "x = (y * y)";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(2.0), 4.0)
    }

    #[test]
    fn neg1() {
        let s = "x = -1.0";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(4.0), -1.0)
    }

    #[test]
    fn neg2() {
        let s = "x = -(y + 1)";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(2.0), -3.0)
    }

    #[test]
    fn neg3() {
        let s = "x = -(y)";
        let eq = Equation::from_str(s).unwrap();
        assert_eq!(eq.eval(2.0), -2.0)
    }

    #[test]
    fn to_string() {
        let s = "x = (y + 1) * (y + 2)";
        let eq = Equation::from_str(s).unwrap();
        let s2 = eq.to_string();
        let eq2 = Equation::from_str(&s2).unwrap();
        assert_eq!(eq.eval(2.0), eq2.eval(2.0))
    }

    #[test]
    fn to_rust_string_1() {
        let s = "(y + 1) * (y + 2)";
        let eq = Expr::from_str(s).unwrap();
        let s2 = eq.to_rust();
        assert_eq!(s2, "((y + 1_f64) * (y + 2_f64))")
    }

    #[test]
    fn to_rust_string_2() {
        let s = "sin(cos(x))";
        let eq = Expr::from_str(s).unwrap();
        let s2 = eq.to_rust();
        assert_eq!(s2, "x.cos().sin()")
    }

    #[test]
    fn eqn_macro_1() {
        let s = eqn!(sin(cos(x)));
        let eq = Expr::from_str(s).unwrap();
        assert_eq!(s, "sin(cos(x))")
    }

    #[test]
    fn eqn_macro_2() {
        let s = eqn!(x ^ 2 ^ 3);
        let eq = Expr::from_str(s).unwrap();
        assert_eq!(s, "x ^ 2 ^ 3")
    }

    #[test]
    fn piecewise_single() {
        let s = "y = {x + 1 if [-5, 5), x + 2 if [5, 10)}";
        let peq = Piecewise::from_str(s).unwrap();
        let cases = vec![
            (
                CaseCondition::Interval(Interval::new(-5, 5, Openness::Closed, Openness::Open)),
                Box::new(Expr::Add(
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Const(1)),
                )),
            ),
            (
                CaseCondition::Interval(Interval::new(5, 10, Openness::Closed, Openness::Open)),
                Box::new(Expr::Add(
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Const(2)),
                )),
            ),
        ];
        let expected = Piecewise::new("y".to_string(), cases);
        assert_eq!(peq.to_string(), expected.to_string());
    }

    #[test]
    fn piecewise_eval() {
        let s = "y = {x + 1 if [-5, 5), x + 2 if [5, 10), x if otherwise}";
        let peq = Piecewise::from_str(s).unwrap();
        assert!(peq.well_formed().is_ok());
        assert_eq!(peq.eval(0.0), 1.0);
        assert_eq!(peq.eval(5.0), 7.0);
        assert_eq!(peq.eval(-5.0), -4.0);
        assert_eq!(peq.eval(-10.0), -10.0);
        assert_eq!(peq.eval(20.0), 20.0);
    }
}
