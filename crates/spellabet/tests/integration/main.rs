#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

use spellabet::{PhoneticConverter, SpellingAlphabet};

#[test]
fn test_convert() {
    let alphabet = &SpellingAlphabet::Nato;
    let converter = PhoneticConverter::new(alphabet);

    assert_eq!(
        converter.convert("Example123"),
        "ECHO x-ray alfa mike papa lima echo One Two Tree"
    );
}

#[test]
fn test_nonce_form() {
    let alphabet = &SpellingAlphabet::Nato;
    let converter = PhoneticConverter::new(alphabet).nonce_form(true);

    assert_eq!(
        converter.convert("Example123"),
        "'E' as in ECHO, 'x' as in x-ray, 'a' as in alfa, 'm' as in mike, 'p' as in papa, 'l' as \
         in lima, 'e' as in echo, One, Two, Tree"
    );
}

#[test]
fn test_with_overrides() {
    let alphabet = &SpellingAlphabet::Nato;
    let mut converter = PhoneticConverter::new(alphabet);

    assert_eq!(converter.convert("a"), "alfa");
    assert_eq!(converter.convert("A"), "ALFA");
    assert_eq!(converter.convert("b"), "bravo");
    assert_eq!(converter.convert("B"), "BRAVO");
    assert_eq!(converter.convert("c"), "charlie");
    assert_eq!(converter.convert("C"), "CHARLIE");

    // Lowercase and uppercase keys will be normalized.
    let mut overrides: HashMap<char, String> = HashMap::new();
    overrides.insert('a', "Able".to_string());
    overrides.insert('B', "Baker".to_string());

    converter = converter.with_overrides(overrides);

    assert_eq!(converter.convert("a"), "able");
    assert_eq!(converter.convert("A"), "ABLE");
    assert_eq!(converter.convert("b"), "baker");
    assert_eq!(converter.convert("B"), "BAKER");
    assert_eq!(converter.convert("c"), "charlie");
    assert_eq!(converter.convert("C"), "CHARLIE");
}
