use nnn::nnn;

#[nnn(nnn_derive(Into<>, From<>, TryFrom<>, Borrow<>, AsRef<>, Deref<>))]
struct Float(f64);

fn main() {}
