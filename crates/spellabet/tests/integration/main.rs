#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use spellabet::{PhoneticConverter, SpellingAlphabet};

#[test]
fn test_convert() {
    let alphabet = &SpellingAlphabet::Nato;
    let converter = PhoneticConverter::new(alphabet);
    let input = "Example123";

    assert_eq!(
        converter.convert(input),
        "ECHO x-ray alfa mike papa lima echo One Two Tree"
    );
}

#[test]
fn test_nonce_form() {
    let alphabet = &SpellingAlphabet::Nato;
    let converter = PhoneticConverter::new(alphabet).nonce_form(true);
    let input = "Example123";

    assert_eq!(
        converter.convert(input),
        "'E' as in ECHO, 'x' as in x-ray, 'a' as in alfa, 'm' as in mike, 'p' as in papa, 'l' as \
         in lima, 'e' as in echo, One, Two, Tree"
    );
}
