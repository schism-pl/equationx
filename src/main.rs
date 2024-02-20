use crate::equation::EquationParser;
use equations::*;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(unused)]
    equation
);

// TODO: allow writing `3*x` as `3x`

fn main() {
    let s = "x = 3.0 - (2.0 - 1.0)";
    let eq = EquationParser::new().parse(s).unwrap();
    println!("{} = {}", eq, eq.eval(2.0));
}
