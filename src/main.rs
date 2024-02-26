// use crate::equation::ExprParser;
use equations::{ast::Equation, *};
use lalrpop_util::lalrpop_mod;
use std::str::FromStr;

lalrpop_mod!(
    #[allow(unused)]
    equation
);

// TODO: allow writing `3*x` as `3x`

fn main() -> anyhow::Result<()> {
    let s = "x = (1 + 1) * 1 ";
    let eq = Equation::from_str(s)?;
    println!("{}", eq);
    Ok(())
}
