use crate::equation::ExprParser;
use equations::*;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(unused)]
    equation
);

// TODO: allow writing `3*x` as `3x`

fn main() {
    let s = "(1 + 1) * 1 ";
    let eq = ExprParser::new().parse(s).unwrap();
    println!("{}", eq);
}
