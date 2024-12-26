#![expect(unused_imports, reason = "Not what we're testing.")]
//! Is compile time error if feature `regex_validation` is enabled
//! else it will compile but fail generated tests.
use nnn::nnn;

#[nnn(validators(regex = r#"(\d+"#))]
struct InvalidRegex(String);

fn main() {}
