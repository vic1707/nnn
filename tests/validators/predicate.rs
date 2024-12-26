/* Built-in imports */
use core::{num, str::FromStr as _};
/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(
    validators(
        predicate(with = Option::is_some, error_name = NotSome)
    )
)]
struct ValidatedFnPathOption(Option<()>);

#[rstest]
#[case(Some(()))]
fn validated_fn_path_option_valid(#[case] input: Option<()>) {
    ValidatedFnPathOption::try_new(input).unwrap();
}

#[rstest]
#[case(None)]
fn validated_fn_path_option_invalid(#[case] input: Option<()>) {
    assert!(matches!(
        ValidatedFnPathOption::try_new(input),
        Err(ValidatedFnPathOptionError::NotSome)
    ));
}

#[nnn(
    validators(
        predicate(with = |opt: &Option<()>| opt.is_some())
    )
)]
struct ValidatedClosureOption(Option<()>);

#[rstest]
#[case(Some(()))]
fn validated_closure_option_valid(#[case] input: Option<()>) {
    ValidatedClosureOption::try_new(input).unwrap();
}

#[rstest]
#[case(None)]
fn validated_closure_option_invalid(#[case] input: Option<()>) {
    assert!(matches!(
        ValidatedClosureOption::try_new(input),
        Err(ValidatedClosureOptionError::Predicate)
    ));
}

#[nnn(
    validators(
        predicate(with = { value.is_some() })
    )
)]
struct ValidatedBlockOption(Option<()>);

#[rstest]
#[case(Some(()))]
fn validated_block_float_string_valid(#[case] input: Option<()>) {
    ValidatedBlockOption::try_new(input).unwrap();
}

#[rstest]
#[case(None)]
fn validated_block_float_string_invalid(#[case] input: Option<()>) {
    assert!(matches!(
        ValidatedBlockOption::try_new(input),
        Err(ValidatedBlockOptionError::Predicate)
    ));
}
