#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use spellabet::{PhoneticConverter, SpellingAlphabet};

#[test]
fn test_convert() {
    let converter = PhoneticConverter::new(&SpellingAlphabet::Nato);
    let input = "Example123";
    assert_eq!(
        converter.convert(input),
        "ECHO x-ray alfa mike papa lima echo One Two Tree"
    );
}
