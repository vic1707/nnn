/* Crate imports */
use nnn::nnn;
/* Built-in imports */
use core::str::FromStr;
/* Dependencies */
use rstest::rstest;

#[nnn(nnn_derive(FromStr), validators(positive))]
struct PositiveFloat(f64);

#[rstest]
#[case("1")]
#[case("1.0")]
fn valid_not_nan_test(#[case] input: &str) {
    PositiveFloat::from_str(input).unwrap();
}

#[rstest]
#[case("toto")]
fn invalid_not_float_parse_test(#[case] input: &str) {
    assert!(matches!(
        PositiveFloat::from_str(input),
        Err(PositiveFloatParseError::InnerParse(_))
    ));
}

#[rstest]
#[case("-10.0")]
fn invalid_negative_float_parse_test(#[case] input: &str) {
    assert!(matches!(
        PositiveFloat::from_str(input),
        Err(PositiveFloatParseError::Validation(
            PositiveFloatError::Positive
        ))
    ));
}
