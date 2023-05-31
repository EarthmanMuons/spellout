#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

use insta::assert_snapshot;
use spellabet::{PhoneticConverter, SpellingAlphabet};

fn init_converter() -> PhoneticConverter {
    let alphabet = SpellingAlphabet::Nato;
    PhoneticConverter::new(&alphabet)
}

#[test]
fn test_lowercase_letters() {
    let converter = init_converter();
    assert_snapshot!(converter.convert("abc"), @"alfa bravo charlie");
}

#[test]
fn test_uppercase_letters() {
    let converter = init_converter();
    assert_snapshot!(converter.convert("ABC"), @"ALFA BRAVO CHARLIE");
}

#[test]
fn test_mixed_case_letters() {
    let converter = init_converter();
    assert_snapshot!(converter.convert("AbC"), @"ALFA bravo CHARLIE");
}

#[test]
fn test_digits() {
    let converter = init_converter();
    assert_snapshot!(converter.convert("123"), @"One Two Tree");
}

#[test]
fn test_symbols() {
    let converter = init_converter();
    assert_snapshot!(
        converter.convert("a.b,c!"),
        @"alfa Period bravo Comma charlie Exclamation"
    );
}

#[test]
fn test_space_character() {
    let converter = init_converter();
    assert_snapshot!(converter.convert(" "), @"Space");
}

#[test]
fn test_empty_string() {
    let converter = init_converter();
    assert_snapshot!(converter.convert(""), @"");
}

#[test]
fn test_unknown_characters() {
    let converter = init_converter();
    assert_snapshot!(converter.convert("aΦb💩c"), @"alfa Φ bravo 💩 charlie");
}

#[test]
fn test_nonce_form_false() {
    let converter = init_converter().nonce_form(false);
    assert_snapshot!(converter.convert("abc"), @"alfa bravo charlie");
    assert_snapshot!(converter.convert("ABC"), @"ALFA BRAVO CHARLIE");
    assert_snapshot!(converter.convert("AbC"), @"ALFA bravo CHARLIE");
}

#[test]
fn test_nonce_form_true() {
    let converter = init_converter().nonce_form(true);
    assert_snapshot!(
        converter.convert("abc"),
        @"'a' as in alfa, 'b' as in bravo, 'c' as in charlie"
    );
    assert_snapshot!(
        converter.convert("ABC"),
        @"'A' as in ALFA, 'B' as in BRAVO, 'C' as in CHARLIE"
    );
    assert_snapshot!(
        converter.convert("AbC"),
        @"'A' as in ALFA, 'b' as in bravo, 'C' as in CHARLIE"
    );
}

#[test]
fn test_nonce_form_single_char() {
    let converter = init_converter().nonce_form(true);
    assert_snapshot!(converter.convert("a"), @"'a' as in alfa");
    assert_snapshot!(converter.convert("A"), @"'A' as in ALFA");
    assert_snapshot!(converter.convert("b"), @"'b' as in bravo");
    assert_snapshot!(converter.convert("B"), @"'B' as in BRAVO");
    assert_snapshot!(converter.convert("c"), @"'c' as in charlie");
    assert_snapshot!(converter.convert("C"), @"'C' as in CHARLIE");
}

#[test]
fn test_nonce_form_digits() {
    let converter = init_converter().nonce_form(true);
    assert_snapshot!(converter.convert("123"), @"One, Two, Tree");
}

#[test]
fn test_nonce_form_symbols() {
    let converter = init_converter().nonce_form(true);
    assert_snapshot!(
        converter.convert("a.b,c!"),
        @"'a' as in alfa, Period, 'b' as in bravo, Comma, 'c' as in charlie, Exclamation"
    );
}

#[test]
fn test_without_overrides() {
    let converter = init_converter();
    assert_snapshot!(converter.convert("a"), @"alfa");
    assert_snapshot!(converter.convert("A"), @"ALFA");
    assert_snapshot!(converter.convert("b"), @"bravo");
    assert_snapshot!(converter.convert("B"), @"BRAVO");
    assert_snapshot!(converter.convert("c"), @"charlie");
    assert_snapshot!(converter.convert("C"), @"CHARLIE");
    assert_snapshot!(converter.convert("abc"), @"alfa bravo charlie");
}

#[test]
fn test_with_overrides() {
    let mut converter = init_converter();
    let mut overrides: HashMap<char, String> = HashMap::new();
    overrides.insert('A', "Able".to_string());
    overrides.insert('B', "Baker".to_string());

    converter = converter.with_overrides(overrides);

    // Check that overrides worked
    assert_snapshot!(converter.convert("a"), @"able");
    assert_snapshot!(converter.convert("A"), @"ABLE");
    assert_snapshot!(converter.convert("b"), @"baker");
    assert_snapshot!(converter.convert("B"), @"BAKER");

    // Check if non-overridden character is still using original conversion
    assert_snapshot!(converter.convert("c"), @"charlie");
    assert_snapshot!(converter.convert("C"), @"CHARLIE");
    assert_snapshot!(converter.convert("abc"), @"able baker charlie");
}

#[test]
fn test_lowercase_key_in_overrides() {
    let mut converter = init_converter();
    let mut overrides: HashMap<char, String> = HashMap::new();
    overrides.insert('c', "Cain".to_string());

    converter = converter.with_overrides(overrides);

    // Check that overrides map key was normalized
    assert_snapshot!(converter.convert("c"), @"cain");
    assert_snapshot!(converter.convert("C"), @"CAIN");
    assert_snapshot!(converter.convert("abc"), @"alfa bravo cain");
}

#[test]
fn test_uppercase_key_in_overrides() {
    let mut converter = init_converter();
    let mut overrides: HashMap<char, String> = HashMap::new();
    overrides.insert('C', "Cain".to_string());

    converter = converter.with_overrides(overrides);

    // Check that overrides map key was normalized
    assert_snapshot!(converter.convert("c"), @"cain");
    assert_snapshot!(converter.convert("C"), @"CAIN");
    assert_snapshot!(converter.convert("abc"), @"alfa bravo cain");
}

#[test]
fn test_overrides_value_normalization() {
    let mut converter = init_converter();
    let mut overrides: HashMap<char, String> = HashMap::new();
    overrides.insert('-', "hyphen".to_string());
    overrides.insert('/', "SLANT".to_string());
    overrides.insert('(', "brackets on".to_string());
    overrides.insert(')', "bracketsOff".to_string());
    overrides.insert('!', "exclamation-mark".to_string());
    overrides.insert('?', "question_mark".to_string());

    converter = converter.with_overrides(overrides);

    // Check that overrides map value was normalized
    assert_snapshot!(converter.convert("-"), @"Hyphen");
    assert_snapshot!(converter.convert("/"), @"Slant");
    assert_snapshot!(converter.convert("("), @"BracketsOn");
    assert_snapshot!(converter.convert(")"), @"BracketsOff");
    assert_snapshot!(converter.convert("!"), @"ExclamationMark");
    assert_snapshot!(converter.convert("?"), @"QuestionMark");
}

#[test]
fn test_lapd_alphabet() {
    let alphabet = SpellingAlphabet::Lapd;
    let converter = PhoneticConverter::new(&alphabet);

    assert_snapshot!(
        converter.convert("abc123xyz"),
        @"adam boy charles One Two Three x-ray young zebra"
    );

    // Test non-default digits
    assert_snapshot!(converter.convert("9"), @"Niner");
}

#[test]
fn test_nato_alphabet() {
    let alphabet = SpellingAlphabet::Nato;
    let converter = PhoneticConverter::new(&alphabet);

    assert_snapshot!(
        converter.convert("abc123xyz"),
        @"alfa bravo charlie One Two Tree x-ray yankee zulu"
    );

    // Test non-default digits
    assert_snapshot!(converter.convert("3"), @"Tree");
    assert_snapshot!(converter.convert("4"), @"Fower");
    assert_snapshot!(converter.convert("5"), @"Fife");
    assert_snapshot!(converter.convert("9"), @"Niner");
}

#[test]
fn test_us_financial_alphabet() {
    let alphabet = SpellingAlphabet::UsFinancial;
    let converter = PhoneticConverter::new(&alphabet);

    assert_snapshot!(
        converter.convert("abc123xyz"),
        @"adam bob carol One Two Three xavier yogi zachary"
    );
}
