#![allow(clippy::float_cmp, reason = "_")]
/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(exactly = 42_i32))]
struct ExactInt(i32);

#[rstest]
#[case(42_i32)]
fn valid_exact_signed_int(#[case] input: i32) {
    ExactInt::try_new(input).unwrap();
}

#[rstest]
#[case(41_i32)]
#[case(43_i32)]
fn invalid_exact_signed_int(#[case] input: i32) {
    assert!(matches!(
        ExactInt::try_new(input),
        Err(ExactIntError::Exactly)
    ));
}

#[nnn(validators(exactly = 3.00_f64))]
struct ExactFloat(f64);

#[rstest]
#[case(3.00_f64)]
fn valid_exact_float(#[case] input: f64) {
    ExactFloat::try_new(input).unwrap();
}

#[rstest]
#[case(3.01_f64)]
#[case(2.99_f64)]
fn invalid_exact_float(#[case] input: f64) {
    assert!(matches!(
        ExactFloat::try_new(input),
        Err(ExactFloatError::Exactly)
    ));
}

#[nnn(validators(exactly = (3.00_f64, 12_i32)))]
struct ExactTuple((f64, i32));

#[rstest]
#[case((3.00_f64, 12_i32))]
fn valid_exact_tuple(#[case] input: (f64, i32)) {
    ExactTuple::try_new(input).unwrap();
}

#[rstest]
#[case((3.00_f64, 13_i32))]
#[case((3.01_f64, 12_i32))]
fn invalid_exact_tuple(#[case] input: (f64, i32)) {
    assert!(matches!(
        ExactTuple::try_new(input),
        Err(ExactTupleError::Exactly)
    ));
}

#[nnn(validators(exactly = [3.00_f64, 2.00_f64]))]
struct ExactArray([f64; 2]);

#[rstest]
#[case([3.00_f64, 2.00_f64])]
fn valid_exact_array(#[case] input: [f64; 2]) {
    ExactArray::try_new(input).unwrap();
}

#[rstest]
#[case([3.00_f64, 2.01_f64])]
#[case([4.00_f64, 2.00_f64])]
fn invalid_exact_array(#[case] input: [f64; 2]) {
    assert!(matches!(
        ExactArray::try_new(input),
        Err(ExactArrayError::Exactly)
    ));
}

#[nnn(validators(exactly = "HEY"))]
struct ExactString<'str>(&'str str);

#[rstest]
#[case("HEY")]
fn valid_exact_string(#[case] input: &str) {
    ExactString::try_new(input).unwrap();
}

#[rstest]
#[case("HEY2")]
#[case("hey")]
fn invalid_exact_string(#[case] input: &'static str) {
    assert!(matches!(
        ExactString::try_new(input),
        Err(ExactStringError::Exactly)
    ));
}
