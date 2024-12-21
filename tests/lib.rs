#![allow(dead_code, reason = "_")]
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
    // Float specifics
    mod finite;
    mod not_infinite;
    mod not_nan;
    // String specifics
    mod regex;
    // Common
    mod exactly;
}
