#![expect(unused_imports, reason = "Not what we're testing.")]
#![expect(unexpected_cfgs, reason = "Dunno how to generate them properly.")]
use nnn::nnn;

#[nnn(derive(PartialEq, Eq, PartialOrd, Ord))]
struct Float(f64);

fn main() {}

// Invalid because `finite` validator is gated behind a `cfg`
// While `derive(Eq, Ord)` isn't.
#[nnn(derive(PartialEq, Eq, PartialOrd, Ord), cfg(test, validators(finite)))]
struct FiniteFloat(f64);

// Invalid because `finite` validator is gated behind a `cfg`
// While `derive(Eq, Ord)` is behind a different one.
#[nnn(
    derive(PartialEq, PartialOrd),
    cfg(toto, validators(finite)),
    cfg(test, derive(Eq, Ord))
)]
struct FiniteFloat2(f64);

// Invalid because `not_nan` validator is gated behind a `cfg`
// While `derive(Eq)` isn't.
#[nnn(derive(PartialEq, Eq, PartialOrd, Ord), cfg(test, validators(not_nan)))]
struct FiniteFloat3(f64);

// Invalid because `not_nan` validator is gated behind a `cfg`
// While `derive(Eq)` is behind a different one.
#[nnn(
    derive(PartialEq, PartialOrd,
    cfg(toto, validators(not_nan)),
    cfg(test, derive(Eq, Ord)))
)]
struct FiniteFloat4(f64);
