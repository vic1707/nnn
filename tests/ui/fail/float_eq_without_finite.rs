#![expect(unused_imports, reason = "Not what we're testing.")]
#![expect(unexpected_cfgs, reason = "Dunno how to generate them properly.")]
use nnn::nnn;

#[nnn(derive(PartialEq, Eq))]
struct Float(f64);

fn main() {}

// Invalid because `finite` validator is gated behind a `cfg`
// While `derive(Eq)` isn't.
#[nnn(derive(PartialEq, Eq), cfg(test, validators(finite)))]
struct FiniteFloat(f64);

// Invalid because `finite` validator is gated behind a `cfg`
// While `derive(Eq)` is behind a different one.
#[nnn(
    derive(PartialEq),
    cfg(toto, validators(finite)),
    cfg(test, derive(Eq))
)]
struct FiniteFloat2(f64);
