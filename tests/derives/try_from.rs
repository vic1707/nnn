/* Crate imports */
use nnn::nnn;

#[nnn(nnn_derive(TryFrom, TryFrom<f32>))]
struct Float(f64);

#[nnn(nnn_derive(TryFrom<_, i32>))]
struct Num(i64);
