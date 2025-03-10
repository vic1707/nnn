/* Crate imports */
use nnn::nnn;

#[nnn(nnn_derive(Borrow, Borrow<str>))]
struct MyString(String);

#[nnn(nnn_derive(Borrow<_, [u8]>))]
struct MyVec(Vec<u8>);
