/* Crate imports */
use nnn::nnn;

#[nnn(nnn_derive(IntoIterator))]
struct FloatVec(Vec<f64>);
