/* Crate imports */
use nnn::nnn;

#[nnn(nnn_derive(From, From<f64>))]
struct Float(f32);
