/* Crate imports */
use nnn::nnn;

#[nnn(nnn_derive(Into))]
struct Float(f64);

#[nnn(nnn_derive(Into<u16, u32>))]
struct Num(u8);
