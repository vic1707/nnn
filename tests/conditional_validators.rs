#![cfg(feature = "macros")]
#![allow(dead_code)]
use nnn::{NNNewType as _, nnn};

#[nnn(
    cfg(all(), validators(min_or_eq = 2)),
    cfg(any(), validators(min_or_eq = 3))
)]
struct ActiveThenInactive(u32);

#[nnn(
    cfg(any(), validators(min_or_eq = 3)),
    cfg(all(), validators(min_or_eq = 2))
)]
struct InactiveThenActive(u32);

#[nnn(validators(min_or_eq = 2), cfg(any(), validators(min_or_eq = 3)))]
struct UnconditionalThenInactive(u32);

#[nnn(cfg(all(), validators(min_or_eq = 2)), validators(min_or_eq = 3))]
struct ConditionalThenUnconditional(u32);

#[nnn(
    cfg(all(), cfg(all(), validators(min_or_eq = 2))),
    cfg(all(), cfg(any(), validators(min_or_eq = 3)))
)]
struct NestedActive(u32);

#[nnn(
    validators(positive),
    cfg(all(), cfg(any(), validators(min_or_eq = 2))),
    cfg(any(), cfg(all(), validators(min_or_eq = 3)))
)]
struct NestedInactive(u32);

#[test]
fn shared_variants_keep_every_enabling_condition() {
    assert!(matches!(
        ActiveThenInactive::try_new(1),
        Err(ActiveThenInactiveError::MinOrEq)
    ));
    assert!(ActiveThenInactive::try_new(2).is_ok());
    assert!(matches!(
        InactiveThenActive::try_new(1),
        Err(InactiveThenActiveError::MinOrEq)
    ));
    assert!(InactiveThenActive::try_new(2).is_ok());
    assert!(matches!(
        UnconditionalThenInactive::try_new(1),
        Err(UnconditionalThenInactiveError::MinOrEq)
    ));
    assert!(UnconditionalThenInactive::try_new(2).is_ok());
    assert!(ConditionalThenUnconditional::try_new(3).is_ok());
    assert!(matches!(
        ConditionalThenUnconditional::try_new(2),
        Err(ConditionalThenUnconditionalError::MinOrEq)
    ));
    assert!(matches!(
        NestedActive::try_new(1),
        Err(NestedActiveError::MinOrEq)
    ));
    assert!(NestedActive::try_new(2).is_ok());
    assert!(NestedInactive::try_new(1).is_ok());
    match NestedInactive::try_new(0).err().unwrap() {
        NestedInactiveError::Positive => (),
    }
}
