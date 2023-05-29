use spellabet::{PhoneticConverter, SpellingAlphabet};

fn main() {
    let alphabet = &SpellingAlphabet::Nato;
    let converter = PhoneticConverter::new(alphabet);
    let input = "Hello, world!";

    println!("{}", converter.convert(input));
}
