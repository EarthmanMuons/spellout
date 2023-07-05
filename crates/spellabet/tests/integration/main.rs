#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

use insta::assert_snapshot;
use spellabet::{PhoneticConverter, SpellingAlphabet};

fn init_converter() -> PhoneticConverter {
    let alphabet = SpellingAlphabet::default();
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
fn test_mappings() {
    let converter = init_converter();
    let mappings = converter.mappings();

    // Check that mappings contain some expected entries
    assert_snapshot!(mappings.get(&'a').unwrap(), @"Alfa");
    assert_snapshot!(mappings.get(&'z').unwrap(), @"Zulu");
    assert_snapshot!(mappings.get(&'0').unwrap(), @"Zero");
    assert_snapshot!(mappings.get(&'9').unwrap(), @"Niner");
    assert_snapshot!(mappings.get(&' ').unwrap(), @"Space");
    assert_snapshot!(mappings.get(&'~').unwrap(), @"Tilde");

    // Check that the size of mappings matches the expect size
    assert_eq!(mappings.len(), 69);
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
    let mut overrides_map: HashMap<char, String> = HashMap::new();
    overrides_map.insert('A', "Able".to_string());
    overrides_map.insert('B', "Baker".to_string());

    converter = converter.with_overrides(overrides_map);

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
    let mut overrides_map: HashMap<char, String> = HashMap::new();
    overrides_map.insert('c', "Cain".to_string());

    converter = converter.with_overrides(overrides_map);

    // Check that overrides map key was normalized
    assert_snapshot!(converter.convert("c"), @"cain");
    assert_snapshot!(converter.convert("C"), @"CAIN");
    assert_snapshot!(converter.convert("abc"), @"alfa bravo cain");
}

#[test]
fn test_uppercase_key_in_overrides() {
    let mut converter = init_converter();
    let mut overrides_map: HashMap<char, String> = HashMap::new();
    overrides_map.insert('C', "Cain".to_string());

    converter = converter.with_overrides(overrides_map);

    // Check that overrides map key was normalized
    assert_snapshot!(converter.convert("c"), @"cain");
    assert_snapshot!(converter.convert("C"), @"CAIN");
    assert_snapshot!(converter.convert("abc"), @"alfa bravo cain");
}

#[test]
fn test_overrides_value_normalization() {
    let mut converter = init_converter();
    let mut overrides_map: HashMap<char, String> = HashMap::new();
    overrides_map.insert('-', "hyphen".to_string());
    overrides_map.insert('/', "SLANT".to_string());
    overrides_map.insert('(', "brackets on".to_string());
    overrides_map.insert(')', "bracketsOff".to_string());
    overrides_map.insert('!', "exclamation-mark".to_string());
    overrides_map.insert('?', "question_mark".to_string());

    converter = converter.with_overrides(overrides_map);

    // Check that overrides map value was normalized
    assert_snapshot!(converter.convert("-"), @"Hyphen");
    assert_snapshot!(converter.convert("/"), @"Slant");
    assert_snapshot!(converter.convert("("), @"BracketsOn");
    assert_snapshot!(converter.convert(")"), @"BracketsOff");
    assert_snapshot!(converter.convert("!"), @"ExclamationMark");
    assert_snapshot!(converter.convert("?"), @"QuestionMark");
}

#[test]
fn test_dump_alphabet() {
    let converter = init_converter();
    let mut buf = Vec::new();
    let verbose = false;
    converter.dump_alphabet(&mut buf, verbose).unwrap();
    let output = String::from_utf8(buf).unwrap();

    assert_snapshot!(output, @r###"
    a -> Alfa
    b -> Bravo
    c -> Charlie
    d -> Delta
    e -> Echo
    f -> Foxtrot
    g -> Golf
    h -> Hotel
    i -> India
    j -> Juliett
    k -> Kilo
    l -> Lima
    m -> Mike
    n -> November
    o -> Oscar
    p -> Papa
    q -> Quebec
    r -> Romeo
    s -> Sierra
    t -> Tango
    u -> Uniform
    v -> Victor
    w -> Whiskey
    x -> X-ray
    y -> Yankee
    z -> Zulu
    "###);
}

#[test]
fn test_dump_alphabet_verbose() {
    let converter = init_converter();
    let mut buf = Vec::new();
    let verbose = true;
    converter.dump_alphabet(&mut buf, verbose).unwrap();
    let output = String::from_utf8(buf).unwrap();

    assert_snapshot!(output, @r###"
    a -> Alfa
    b -> Bravo
    c -> Charlie
    d -> Delta
    e -> Echo
    f -> Foxtrot
    g -> Golf
    h -> Hotel
    i -> India
    j -> Juliett
    k -> Kilo
    l -> Lima
    m -> Mike
    n -> November
    o -> Oscar
    p -> Papa
    q -> Quebec
    r -> Romeo
    s -> Sierra
    t -> Tango
    u -> Uniform
    v -> Victor
    w -> Whiskey
    x -> X-ray
    y -> Yankee
    z -> Zulu
    0 -> Zero
    1 -> One
    2 -> Two
    3 -> Tree
    4 -> Fower
    5 -> Fife
    6 -> Six
    7 -> Seven
    8 -> Eight
    9 -> Niner
      -> Space
    ! -> Exclamation
    " -> DoubleQuote
    # -> Hash
    $ -> Dollars
    % -> Percent
    & -> Ampersand
    ' -> SingleQuote
    ( -> LeftParens
    ) -> RightParens
    * -> Asterisk
    + -> Plus
    , -> Comma
    - -> Dash
    . -> Period
    / -> ForeSlash
    : -> Colon
    ; -> SemiColon
    < -> LessThan
    = -> Equals
    > -> GreaterThan
    ? -> Question
    @ -> At
    [ -> LeftBracket
    \ -> BackSlash
    ] -> RightBracket
    ^ -> Caret
    _ -> Underscore
    ` -> Backtick
    { -> LeftBrace
    | -> Pipe
    } -> RightBrace
    ~ -> Tilde
    "###);
}

#[test]
fn test_jan_alphabet() {
    let alphabet = SpellingAlphabet::Jan;
    let converter = PhoneticConverter::new(&alphabet);

    assert_snapshot!(
        converter.convert("abc123xyz"),
        @"able baker charlie One Two Three x-ray yoke zebra"
    );
}

#[test]
fn test_lapd_alphabet() {
    let alphabet = SpellingAlphabet::Lapd;
    let converter = PhoneticConverter::new(&alphabet);

    assert_snapshot!(
        converter.convert("abc123xyz"),
        @"adam boy charles One Two Three x-ray young zebra"
    );

    // Check non-default digits
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

    // Check non-default digits
    assert_snapshot!(converter.convert("3"), @"Tree");
    assert_snapshot!(converter.convert("4"), @"Fower");
    assert_snapshot!(converter.convert("5"), @"Fife");
    assert_snapshot!(converter.convert("9"), @"Niner");
}

#[test]
fn test_royal_navy_alphabet() {
    let alphabet = SpellingAlphabet::RoyalNavy;
    let converter = PhoneticConverter::new(&alphabet);

    assert_snapshot!(
        converter.convert("abc123xyz"),
        @"apples butter charlie One Two Three xerxes yellow zebra"
    );
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

#[test]
fn test_western_union_alphabet() {
    let alphabet = SpellingAlphabet::WesternUnion;
    let converter = PhoneticConverter::new(&alphabet);

    assert_snapshot!(
        converter.convert("abc123xyz"),
        @"adams boston chicago One Two Three x-ray young zero"
    );
}
