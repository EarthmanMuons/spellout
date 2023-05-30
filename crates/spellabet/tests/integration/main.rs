#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

use spellabet::{PhoneticConverter, SpellingAlphabet};

#[test]
fn test_convert() {
    let alphabet = &SpellingAlphabet::Nato;
    let converter = PhoneticConverter::new(alphabet);

    // Test lowercase letters
    assert_eq!(converter.convert("abc"), "alfa bravo charlie");

    // Test uppercase letters
    assert_eq!(converter.convert("ABC"), "ALFA BRAVO CHARLIE");

    // Test mixed case letters
    assert_eq!(converter.convert("AbC"), "ALFA bravo CHARLIE");

    // Test digits
    assert_eq!(converter.convert("123"), "One Two Tree");

    // Test symbols
    assert_eq!(
        converter.convert("a.b,c!"),
        "alfa Period bravo Comma charlie Exclamation"
    );

    // Test space character
    assert_eq!(converter.convert(" "), "Space");

    // Test empty string
    assert_eq!(converter.convert(""), "");

    // Test characters not in the conversion_map
    assert_eq!(converter.convert("aΦb💩c"), "alfa Φ bravo 💩 charlie");
}

#[test]
fn test_nonce_form() {
    let alphabet = &SpellingAlphabet::Nato;

    // Test with nonce_form = false
    let converter = PhoneticConverter::new(alphabet).nonce_form(false);
    assert_eq!(converter.convert("a"), "alfa");
    assert_eq!(converter.convert("abc"), "alfa bravo charlie");
    assert_eq!(converter.convert("ABC"), "ALFA BRAVO CHARLIE");
    assert_eq!(converter.convert("123"), "One Two Tree");

    // Test with nonce_form = true
    let converter = PhoneticConverter::new(alphabet).nonce_form(true);
    assert_eq!(converter.convert("a"), "'a' as in alfa");
    assert_eq!(
        converter.convert("abc"),
        "'a' as in alfa, 'b' as in bravo, 'c' as in charlie"
    );
    assert_eq!(
        converter.convert("ABC"),
        "'A' as in ALFA, 'B' as in BRAVO, 'C' as in CHARLIE"
    );
    assert_eq!(converter.convert("123"), "One, Two, Tree");

    // Test mixed case letters
    assert_eq!(
        converter.convert("AbC"),
        "'A' as in ALFA, 'b' as in bravo, 'C' as in CHARLIE"
    );

    // Test symbols
    assert_eq!(
        converter.convert("a.b,c!"),
        "'a' as in alfa, Period, 'b' as in bravo, Comma, 'c' as in charlie, Exclamation"
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

    // Test that lowercase and uppercase keys will be normalized
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
