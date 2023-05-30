#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

use spellabet::{PhoneticConverter, SpellingAlphabet};

fn init_converter() -> PhoneticConverter {
    let alphabet = &SpellingAlphabet::Nato;
    PhoneticConverter::new(alphabet)
}

#[test]
fn test_lowercase_letters() {
    let converter = init_converter();
    assert_eq!(converter.convert("abc"), "alfa bravo charlie");
}

#[test]
fn test_uppercase_letters() {
    let converter = init_converter();
    assert_eq!(converter.convert("ABC"), "ALFA BRAVO CHARLIE");
}

#[test]
fn test_mixed_case_letters() {
    let converter = init_converter();
    assert_eq!(converter.convert("AbC"), "ALFA bravo CHARLIE");
}

#[test]
fn test_digits() {
    let converter = init_converter();
    assert_eq!(converter.convert("123"), "One Two Tree");
}

#[test]
fn test_symbols() {
    let converter = init_converter();
    assert_eq!(
        converter.convert("a.b,c!"),
        "alfa Period bravo Comma charlie Exclamation"
    );
}

#[test]
fn test_space_character() {
    let converter = init_converter();
    assert_eq!(converter.convert(" "), "Space");
}

#[test]
fn test_empty_string() {
    let converter = init_converter();
    assert_eq!(converter.convert(""), "");
}

#[test]
fn test_unknown_characters() {
    let converter = init_converter();
    assert_eq!(converter.convert("aΦb💩c"), "alfa Φ bravo 💩 charlie");
}

#[test]
fn test_nonce_form_false() {
    let converter = init_converter().nonce_form(false);
    assert_eq!(converter.convert("abc"), "alfa bravo charlie");
    assert_eq!(converter.convert("ABC"), "ALFA BRAVO CHARLIE");
    assert_eq!(converter.convert("AbC"), "ALFA bravo CHARLIE");
}

#[test]
fn test_nonce_form_true() {
    let converter = init_converter().nonce_form(true);
    assert_eq!(
        converter.convert("abc"),
        "'a' as in alfa, 'b' as in bravo, 'c' as in charlie"
    );
    assert_eq!(
        converter.convert("ABC"),
        "'A' as in ALFA, 'B' as in BRAVO, 'C' as in CHARLIE"
    );
    assert_eq!(
        converter.convert("AbC"),
        "'A' as in ALFA, 'b' as in bravo, 'C' as in CHARLIE"
    );
}

#[test]
fn test_nonce_form_single_char() {
    let converter = init_converter().nonce_form(true);
    assert_eq!(converter.convert("a"), "'a' as in alfa");
    assert_eq!(converter.convert("A"), "'A' as in ALFA");
    assert_eq!(converter.convert("b"), "'b' as in bravo");
    assert_eq!(converter.convert("B"), "'B' as in BRAVO");
    assert_eq!(converter.convert("c"), "'c' as in charlie");
    assert_eq!(converter.convert("C"), "'C' as in CHARLIE");
}

#[test]
fn test_nonce_form_digits() {
    let converter = init_converter().nonce_form(true);
    assert_eq!(converter.convert("123"), "One, Two, Tree");
}

#[test]
fn test_nonce_form_symbols() {
    let converter = init_converter().nonce_form(true);
    assert_eq!(
        converter.convert("a.b,c!"),
        "'a' as in alfa, Period, 'b' as in bravo, Comma, 'c' as in charlie, Exclamation"
    );
}

#[test]
fn test_without_overrides() {
    let converter = init_converter();
    assert_eq!(converter.convert("a"), "alfa");
    assert_eq!(converter.convert("A"), "ALFA");
    assert_eq!(converter.convert("b"), "bravo");
    assert_eq!(converter.convert("B"), "BRAVO");
    assert_eq!(converter.convert("c"), "charlie");
    assert_eq!(converter.convert("C"), "CHARLIE");
    assert_eq!(converter.convert("abc"), "alfa bravo charlie");
}

#[test]
fn test_with_overrides() {
    let mut converter = init_converter();
    let mut overrides: HashMap<char, String> = HashMap::new();
    overrides.insert('A', "Able".to_string());
    overrides.insert('B', "Baker".to_string());

    converter = converter.with_overrides(overrides);

    // Check that overrides worked
    assert_eq!(converter.convert("a"), "able");
    assert_eq!(converter.convert("A"), "ABLE");
    assert_eq!(converter.convert("b"), "baker");
    assert_eq!(converter.convert("B"), "BAKER");

    // Check if non-overridden character is still using original conversion
    assert_eq!(converter.convert("c"), "charlie");
    assert_eq!(converter.convert("C"), "CHARLIE");
    assert_eq!(converter.convert("abc"), "able baker charlie");
}

#[test]
fn test_lowercase_key_in_overrides() {
    let mut converter = init_converter();
    let mut overrides: HashMap<char, String> = HashMap::new();
    overrides.insert('c', "Cain".to_string());

    converter = converter.with_overrides(overrides);

    // Check that overrides map key was normalized
    assert_eq!(converter.convert("c"), "cain");
    assert_eq!(converter.convert("C"), "CAIN");
    assert_eq!(converter.convert("abc"), "alfa bravo cain");
}

#[test]
fn test_uppercase_key_in_overrides() {
    let mut converter = init_converter();
    let mut overrides: HashMap<char, String> = HashMap::new();
    overrides.insert('C', "Cain".to_string());

    converter = converter.with_overrides(overrides);

    // Check that overrides map key was normalized
    assert_eq!(converter.convert("c"), "cain");
    assert_eq!(converter.convert("C"), "CAIN");
    assert_eq!(converter.convert("abc"), "alfa bravo cain");
}
