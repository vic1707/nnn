/* Crate imports */
use nnn::nnn;

#[nnn(nnn_derive(Deref<str>))]
struct MyString(String);

#[nnn(nnn_derive(Deref))]
struct MyVec(Vec<u8>);

#[nnn(nnn_derive(Deref<_>))]
struct MyVec2(Vec<u8>);
