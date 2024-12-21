/* Built-in imports */
use std::sync::LazyLock;
/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(regex = r"^\d{3}-\d{2}-\d{4}$"))]
struct RegexLiteralStr<'str>(&'str str);

#[rstest]
#[case("123-45-6789")]
fn valid_regex_literal_str(#[case] input: &str) {
    RegexLiteralStr::try_new(input).unwrap();
}

#[rstest]
#[case("1234-56-7890")]
#[case("123-45-678")]
fn invalid_regex_literal_str(#[case] input: &str) {
    assert!(matches!(
        RegexLiteralStr::try_new(input),
        Err(RegexLiteralStrError::Regex)
    ));
}

// Regex with a variable
#[expect(clippy::expect_used, reason = "Test, so this is acceptable.")]
static SSN_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::try_from(r"^\d{3}-\d{2}-\d{4}$").expect("Invalid test regex")
});

#[nnn(validators(regex = SSN_REGEX))]
struct RegexVariableStr<'str>(&'str str);

#[rstest]
#[case("123-45-6789")]
fn valid_regex_variable_str(#[case] input: &str) {
    RegexVariableStr::try_new(input).unwrap();
}

#[rstest]
#[case("abc-def-ghij")]
#[case("123-456-789")]
fn invalid_regex_variable_str(#[case] input: &str) {
    assert!(matches!(
        RegexVariableStr::try_new(input),
        Err(RegexVariableStrError::Regex)
    ));
}
