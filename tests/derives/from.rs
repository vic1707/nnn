/* Crate imports */
use nnn::nnn;

#[nnn(nnn_derive(From, From<f64>))]
struct Float(f32);

#[nnn(nnn_derive(From<_, i64>))]
struct Num(i32);
