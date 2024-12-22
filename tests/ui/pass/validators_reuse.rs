#![expect(unused_imports, reason = "_")]
use nnn::nnn;

#[nnn(validators(not_empty, each(not_empty)))]
struct StringsVec(Vec<String>);

#[nnn(validators(not_empty, each(not_empty, each(finite))))]
struct VecFiniteFloatVec(Vec<Vec<f64>>);

fn main() {}
