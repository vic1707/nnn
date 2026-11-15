#![cfg(feature = "macros")]
use nnn::nnn;

#[nnn(default = String::from("Ada"))]
struct Name(String);
