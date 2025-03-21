#![allow(dead_code, unused_imports, reason = "Tests, dead_code is expected.")]
/* Modules */
#[rustfmt::skip] // wants to reorder modules
mod validators {
    // Containers
    mod not_empty;
    mod each;
    mod min_length;
    mod min_length_or_eq;
    mod length;
    mod max_length;
    mod max_length_or_eq;
    // Numerics
    mod min;
    mod min_or_eq;
    mod max;
    mod max_or_eq;
    mod positive;
    mod negative;
    // Float specifics
    mod finite;
    mod not_infinite;
    mod not_nan;
    // String specifics
    mod regex;
    // Common
    mod exactly;
    mod custom;
    mod predicate;
}

mod derives {
    mod as_ref;
    mod borrow;
    mod deref;
    mod from;
    mod from_str;
    mod into;
    mod into_iterator;
    mod serde;
    mod try_from;
}

#[rustfmt::skip] // wants to reorder modules
mod sanitizers {
    // Containers
    mod each;
    mod sort;
    mod dedup;
    // Strings
    mod trim;
    mod lowercase;
    mod uppercase;
    // Common
    mod custom;
}

#[cfg(test)]
mod ui {
    #[test]
    fn ui_pass() {
        trybuild::TestCases::new().pass("tests/ui/pass/*.rs");
        #[cfg(not(feature = "regex_validation"))]
        trybuild::TestCases::new()
            .pass("tests/ui/conditionals/invalid_compile_time_regex.rs");
    }

    #[test]
    fn ui_fail() {
        trybuild::TestCases::new().compile_fail("tests/ui/fail/*.rs");
        #[cfg(feature = "regex_validation")]
        trybuild::TestCases::new().compile_fail(
            "tests/ui/conditionals/invalid_compile_time_regex.rs",
        );
    }
}

#[doc(hidden)]
pub mod utils {
    macro_rules! sign_tests {
        ($sign_test:ident, $($ty:ty, valids = [$($valid:literal),*], invalids = [$($invalid:expr),*]),*) => {
            $(
                paste::paste! {
                    mod [< $sign_test _ $ty >] {
                        use rstest::rstest;
                        use nnn::{nnn, NNNewType as _};

                        #[nnn(validators($sign_test))]
                        struct NNN($ty);

                        #[rstest]
                        $(#[case($valid)])*
                        fn [< valid_ $sign_test _ $ty >](#[case] input: $ty) {
                            NNN::try_new(input).unwrap();
                        }

                        #[rstest]
                        $(#[case($invalid)])*
                        fn [< invalid_ $sign_test _ $ty >](#[case] input: $ty) {
                            assert!(NNN::try_new(input).is_err());
                        }
                    }
                }
            )*
        };
    }

    pub(crate) use sign_tests;
}
