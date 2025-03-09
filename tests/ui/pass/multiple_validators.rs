#![expect(unused_imports, reason = "_")]
use nnn::{nnn, NNNewType as _};

#[nnn(validators(each(finite, min = 0.0_f64)))]
struct FiniteFloatsVec(Vec<f64>);

#[nnn(validators(not_empty, each(finite)))]
struct NonEmptyFiniteFloatsVec(Vec<f64>);

#[nnn(validators(each(finite), not_empty))]
struct FiniteFloatsVecNonEmpty(Vec<f64>);

#[nnn(validators(finite))]
struct FiniteFloat(f64);

#[nnn(validators(length = 3, not_empty))]
struct AAA(String);

fn main() {}
