#![expect(unused_imports, reason = "_")]
use nnn::nnn;

#[nnn(derive(PartialEq, Eq), validators(finite))]
struct FiniteFloat(f64);

fn main() {}

#[nnn(cfg(test, derive(PartialEq, Eq), validators(finite)))]
struct FiniteFloat2(f64);