/* Built-in imports */
use core::{num, str::FromStr as _};
/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(
    validators(
        custom(with = f64::from_str, error = num::ParseFloatError)
    )
)]
struct ValidatedFnPathFloatString(String);

#[rstest]
#[case("4.0")]
fn validated_fn_path_float_string_valid(#[case] input: &str) {
    ValidatedFnPathFloatString::try_new(input.to_owned()).unwrap();
}

#[rstest]
#[case("not a float")]
fn validated_fn_path_float_string_valid_invalid(#[case] input: &str) {
    assert!(matches!(
        ValidatedFnPathFloatString::try_new(input.to_owned()),
        Err(ValidatedFnPathFloatStringError::NumParsefloaterror(_))
    ));
}

#[nnn(
    validators(
        custom(with = |str: &String| f64::from_str(str), error = num::ParseFloatError)
    )
)]
struct ValidatedClosureFloatString(String);

#[rstest]
#[case("4.0")]
fn validated_closure_float_string_valid(#[case] input: &str) {
    ValidatedClosureFloatString::try_new(input.to_owned()).unwrap();
}

#[rstest]
#[case("not a float")]
fn validated_closure_float_string_invalid(#[case] input: &str) {
    assert!(matches!(
        ValidatedClosureFloatString::try_new(input.to_owned()),
        Err(ValidatedClosureFloatStringError::NumParsefloaterror(_))
    ));
}

#[nnn(
    validators(
        custom(with = { f64::from_str(&value) }, error = num::ParseFloatError)
    )
)]
struct ValidatedBlockFloatString(String);

#[rstest]
#[case("4.0")]
fn validated_block_float_string_valid(#[case] input: &str) {
    ValidatedBlockFloatString::try_new(input.to_owned()).unwrap();
}

#[rstest]
#[case("not a float")]
fn validated_block_float_string_invalid(#[case] input: &str) {
    assert!(matches!(
        ValidatedBlockFloatString::try_new(input.to_owned()),
        Err(ValidatedBlockFloatStringError::NumParsefloaterror(_))
    ));
}
