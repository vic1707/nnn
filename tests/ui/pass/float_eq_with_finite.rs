#![expect(unused_imports, reason = "_")]
use nnn::nnn;

#[nnn(derive(PartialEq, Eq), validators(finite))]
struct FiniteFloat(f64);

fn main() {}
