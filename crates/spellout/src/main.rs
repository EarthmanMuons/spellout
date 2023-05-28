use spellabet::{PhoneticConverter, SpellingAlphabet};

fn main() {
    let converter = PhoneticConverter::new(&SpellingAlphabet::Nato);
    let input = "Hello, world!";
    println!("{}", converter.convert(input));
}
