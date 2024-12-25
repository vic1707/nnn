#![expect(unused_imports, reason = "Not what we're testing.")]
use nnn::nnn;

#[nnn(derive(PartialEq, Eq))]
struct Float(f64);

fn main() {}
