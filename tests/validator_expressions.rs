#![cfg(feature = "macros")]
#![allow(dead_code)]
use nnn::{NNNewType as _, nnn};

#[nnn(validators(min_or_eq = if true { 2 } else { 3 }))]
struct ConditionalBound(u32);

#[nnn(validators(max_or_eq = { 1 + 1 }))]
struct BlockBound(u32);

#[nnn(validators(exactly = "{literal}"))]
struct LiteralBraces(String);

#[test]
fn expression_text_is_not_a_format_string() {
    assert!(ConditionalBound::try_new(2).is_ok());
    let message = ConditionalBound::try_new(1).err().unwrap().to_string();
    assert!(message.contains("if true"));
    assert!(message.contains('{') && message.contains('}'));
    assert!(BlockBound::try_new(2).is_ok());
    let message = BlockBound::try_new(3).err().unwrap().to_string();
    assert!(message.contains("1 + 1"));
    assert!(message.contains('{') && message.contains('}'));
    assert!(LiteralBraces::try_new("{literal}".into()).is_ok());
    assert!(
        LiteralBraces::try_new("other".into())
            .err()
            .unwrap()
            .to_string()
            .contains("{literal}")
    );
}
