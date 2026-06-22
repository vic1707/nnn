/* Built-in imports */
use core::{num, str::FromStr as _};
/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

#[nnn(
    validators(
        custom(with = f64::from_str, error = ParseFloat(num::ParseFloatError))
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
        Err(ValidatedFnPathFloatStringError::ParseFloat(_))
    ));
}

#[nnn(
    validators(
        custom(with = |str: &String| f64::from_str(str), error = ParseFloat(num::ParseFloatError))
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
        Err(ValidatedClosureFloatStringError::ParseFloat(_))
    ));
}

#[nnn(
    validators(
        custom(with = { f64::from_str(&value) }, error = ParseFloat(num::ParseFloatError))
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
        Err(ValidatedBlockFloatStringError::ParseFloat(_))
    ));
}

#[nnn(
    validators(
        custom(
            with = |bytes: &[u8]| matches!(bytes[0] & 0x0F, 0x02 | 0x06 | 0x0A | 0x0E),
            error = NotLocallyAdministered
        )
    )
)]
struct MacAddressClosure([u8; 6]);

#[rstest]
#[case([0x02, 0x48, 0x65, 0x6E, 0x72, 0x69])]
fn validated_boolean_custom_closure_valid(#[case] input: [u8; 6]) {
    MacAddressClosure::try_new(input).unwrap();
}

#[rstest]
#[case([0x01, 0x48, 0x65, 0x6E, 0x72, 0x69])]
fn validated_boolean_custom_closure_invalid(#[case] input: [u8; 6]) {
    assert!(matches!(
        MacAddressClosure::try_new(input),
        Err(MacAddressClosureError::NotLocallyAdministered)
    ));
}

#[nnn(
    validators(
        custom(
            with = { value[0] & 0x0F == 0x02 || value[0] & 0x0F == 0x06 || value[0] & 0x0F == 0x0A || value[0] & 0x0F == 0x0E },
            error = NotLocallyAdministered
        )
    )
)]
struct MacAddressBlock([u8; 6]);

#[rstest]
#[case([0x02, 0x48, 0x65, 0x6E, 0x72, 0x69])]
fn validated_boolean_custom_block_valid(#[case] input: [u8; 6]) {
    MacAddressBlock::try_new(input).unwrap();
}

#[rstest]
#[case([0x01, 0x48, 0x65, 0x6E, 0x72, 0x69])]
fn validated_boolean_custom_block_invalid(#[case] input: [u8; 6]) {
    assert!(matches!(
        MacAddressBlock::try_new(input),
        Err(MacAddressBlockError::NotLocallyAdministered)
    ));
}

fn is_locally_administered(bytes: &[u8]) -> bool {
    matches!(bytes[0] & 0x0F, 0x02 | 0x06 | 0x0A | 0x0E)
}

#[nnn(
    validators(
        custom(
            with = is_locally_administered,
            error = NotLocallyAdministered
        )
    )
)]
struct MacAddressFnPath([u8; 6]);

#[rstest]
#[case([0x02, 0x48, 0x65, 0x6E, 0x72, 0x69])]
fn validated_boolean_custom_fn_path_valid(#[case] input: [u8; 6]) {
    MacAddressFnPath::try_new(input).unwrap();
}

#[rstest]
#[case([0x01, 0x48, 0x65, 0x6E, 0x72, 0x69])]
fn validated_boolean_custom_fn_path_invalid(#[case] input: [u8; 6]) {
    assert!(matches!(
        MacAddressFnPath::try_new(input),
        Err(MacAddressFnPathError::NotLocallyAdministered)
    ));
}
