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
}
#[rustfmt::skip] // wants to reorder modules
mod derives {
    mod serde;
}

#[cfg(test)]
mod ui {
    #[test]
    fn ui_pass() {
        trybuild::TestCases::new().pass("tests/ui/pass/*.rs");
    }

    #[test]
    fn ui_fail() {
        trybuild::TestCases::new().compile_fail("tests/ui/fail/*.rs");
    }
}
