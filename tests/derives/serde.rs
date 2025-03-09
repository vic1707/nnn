/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;
use serde::{Deserialize, Serialize};

#[nnn(
    derive(Debug, Serialize, Deserialize, PartialEq),
    nnn_derive(TryFrom),
    validators(positive)
)]
struct Foo(f64);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Data {
    pub foo: Foo,
}

#[rstest]
#[case(r#"{ "foo": 0.1 }"#, 0.1_f64)]
#[case(r#"{ "foo": 3.0 }"#, 3.0_f64)]
fn data_deserialization_valid(#[case] input: &str, #[case] expected: f64) {
    let deserialized =
        serde_json::from_str::<Data>(input).expect("Deserialization failed.");
    let expected_data = Data {
        foo: Foo::try_new(expected).expect("Should have a valid input."),
    };
    assert_eq!(deserialized, expected_data);
}

#[rstest]
#[case(r#"{ "foo": -3.0 }"#)]
#[case(r#"{ "foo": -0.0 }"#)]
#[case(r#"{ "foo": "0.0" }"#)]
#[case(r#"{ "foo": "coucou" }"#)]
fn data_deserialization_invalid(#[case] input: &str) {
    serde_json::from_str::<Data>(input).unwrap_err();
}
