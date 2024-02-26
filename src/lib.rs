pub mod ast;

use crate::ast::Equation;
use equation::*;

use lalrpop_util::lalrpop_mod;
use std::str::FromStr;

lalrpop_mod!(
    #[allow(unused)]
    equation
);

// TODO: use approx for tests

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Equation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        EquationParser::new()
            .parse(s)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}

impl<'de> Deserialize<'de> for Equation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Equation;
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
}
