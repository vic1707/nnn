/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

#[nnn(validators(one_of = [38_u8, 56, 57, 58]))]
struct OneOfInt(u8);

#[rstest]
#[case(38_u8)]
#[case(58_u8)]
fn valid_one_of_int(#[case] input: u8) {
    OneOfInt::try_new(input).unwrap();
}

#[rstest]
#[case(0_u8)]
#[case(55_u8)]
fn invalid_one_of_int(#[case] input: u8) {
    assert!(matches!(
        OneOfInt::try_new(input),
        Err(OneOfIntError::OneOf)
    ));
}

#[test]
fn one_of_error_names_the_type() {
    assert!(
        OneOfIntError::OneOf
            .to_string()
            .starts_with("[OneOfInt] Value should be one of [")
    );
}

#[nnn(validators(one_of = ["sd", "usb"]))]
struct OneOfString(String);

#[rstest]
#[case("sd")]
#[case("usb")]
fn valid_one_of_string(#[case] input: &str) {
    OneOfString::try_new(input.to_owned()).unwrap();
}

#[rstest]
#[case("")]
#[case("SD")]
fn invalid_one_of_string(#[case] input: &str) {
    assert!(matches!(
        OneOfString::try_new(input.to_owned()),
        Err(OneOfStringError::OneOf)
    ));
}

const ALLOWED: &[f64] = &[1.5_f64, 2.5_f64];

#[nnn(validators(one_of = ALLOWED))]
struct OneOfFromConst(f64);

#[test]
fn one_of_accepts_a_path() {
    OneOfFromConst::try_new(2.5_f64).unwrap();
    assert!(matches!(
        OneOfFromConst::try_new(2.0_f64),
        Err(OneOfFromConstError::OneOf)
    ));
}
