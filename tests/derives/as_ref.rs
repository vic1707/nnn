/* Crate imports */
use nnn::nnn;

#[nnn(nnn_derive(AsRef, AsRef<str>))]
struct MyString(String);

#[nnn(nnn_derive(AsRef<_, [u8]>))]
struct MyVec(Vec<u8>);
