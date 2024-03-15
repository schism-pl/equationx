use cordic::CordicNumber;

use crate::fixed_point::CordicMarker;


pub fn cordic_powf<T: CordicNumber + CordicMarker>(a: T, b:T) -> T {
    cordic::exp() / cordic::exp()
}



pub fn cordic_log<T: CordicNumber + CordicMarker>(a: T, b:T) -> T {

}


/// Implement logarithm using tanh identity
/// impl<T: CordicNumber + CordicMarker> Expr<T> {

// const fn fact(x: usize) -> usize {
//     match x {
//         0 => 1,
//         1 => 1,
//         2 => 2,
//         3 => 6,
//         4 => 24,
//         5 => 120,
//         6 => 720,
//         7 => 5040,
//         8 => 40320,
//         9 => 362880,
//         10 => 3628800,
//         11 => 39916800,
//         12 => 479001600,
//         13 => 6227020800,
//         14 => 87178291200,
//     }
// }

// pub fn cordic_log<T: CordicNumber + CordicMarker>(a: T, b: T) -> T {

// }

// pub fn powi<T: CordicNumber + CordicMarker>(a: T, b: T) -> T {
//     (T::zero()..b).fold(T::zero(), |acc, x| acc * x)
// }


// // use terms up to 12 factorial

// pub fn taylor_powf<T: CordicNumber + CordicMarker>(a: T, b:T) -> T {

// }

// pub fn taylor_e_log<T: CordicNumber + CordicMarker>(a: T) -> T {
//     2 * taylor_atanh(x-1 / x+1)
// }

// // use change of base
// pub fn taylor_log<T: CordicNumber + CordicMarker>(a: T, b: T) -> T {
//     2 * taylor_atanh(x-1 / x+1)
// }




// pub fn taylor_sin<T: CordicNumber + CordicMarker>(a: T) -> T {
//     a - powi(a, 3) / fact(3) + powi(a, 5) / fact(5) - powi(a, 7) / fact(7) + powi(a, 9) / fact(9) - powi(a, 11) / fact(11)
// }

// pub fn taylor_cos<T: CordicNumber + CordicMarker>(a: T) -> T {
//     1 - powi(a, 2) / fact(2) + powi(a, 4) / fact(4) - powi(a, 6) / fact(6) + powi(a, 8) / fact(8) - powi(a, 10) / fact(10) + powi(a, 12) / fact(12)
// }

// pub fn taylor_tan<T: CordicNumber + CordicMarker>(a: T) -> T {
//     taylor_sin(a) / taylor_cos(a)
// }

// pub fn taylor_asin<T: CordicNumber + CordicMarker>(a: T) -> T {
//     a + powi(a, 3)/6 + 3*powi(a, 5)/40 
// }

// pub fn taylor_acos<T: CordicNumber + CordicMarker>(a: T) -> T {

// }

// pub fn taylor_atan<T: CordicNumber + CordicMarker>(a: T) -> T {
//     taylor_sin(a) / taylor_sin(a)
// }

// pub fn taylor_sinh<T: CordicNumber + CordicMarker>(a: T) -> T {
//     a + powi(a, 3) / fact(3) + powi(a, 5) / fact(5) + powi(a, 7) / fact(7) + powi(a, 9) / fact(9) + powi(a, 11) / fact(11)
// }

// pub fn taylor_cosh<T: CordicNumber + CordicMarker>(a: T) -> T {
//     1 + powi(a, 2) / fact(2) + powi(a, 4) / fact(4) + powi(a, 6) / fact(6) + powi(a, 8) / fact(8) + powi(a, 10) / fact(10) + powi(a, 12) / fact(12)
// }

// pub fn taylor_tanh<T: CordicNumber + CordicMarker>(a: T) -> T {
//     taylor_sinh(a) / taylor_cosh(a)
// }

// pub fn taylor_asinh<T: CordicNumber + CordicMarker>(a: T) -> T {

// }

// pub fn taylor_acosh<T: CordicNumber + CordicMarker>(a: T) -> T {

// }

// pub fn taylor_atanh<T: CordicNumber + CordicMarker>(a: T) -> T {
//     taylor_asinh(a) / taylor_acosh(a)
// }






