#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static NATO_ALPHABET: Lazy<HashMap<char, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('a', "Alpha");
    map.insert('b', "Bravo");
    map.insert('c', "Charlie");
    map.insert('d', "Delta");
    map.insert('e', "Echo");
    map.insert('f', "Foxtrot");
    map.insert('g', "Golf");
    map.insert('h', "Hotel");
    map.insert('i', "India");
    map.insert('j', "Juliet");
    map.insert('k', "Kilo");
    map.insert('l', "Lima");
    map.insert('m', "Mike");
    map.insert('n', "November");
    map.insert('o', "Oscar");
    map.insert('p', "Papa");
    map.insert('q', "Quebec");
    map.insert('r', "Romeo");
    map.insert('s', "Sierra");
    map.insert('t', "Tango");
    map.insert('u', "Uniform");
    map.insert('v', "Victor");
    map.insert('w', "Whiskey");
    map.insert('x', "X-ray");
    map.insert('y', "Yankee");
    map.insert('z', "Zulu");
    map.insert('0', "Zero");
    map.insert('1', "One");
    map.insert('2', "Two");
    map.insert('3', "Three");
    map.insert('4', "Four");
    map.insert('5', "Five");
    map.insert('6', "Six");
    map.insert('7', "Seven");
    map.insert('8', "Eight");
    map.insert('9', "Nine");
    map.insert(' ', "Space");
    map.insert('!', "Exclamation");
    map.insert('"', "DoubleQuote");
    map.insert('#', "Hash");
    map.insert('$', "Dollars");
    map.insert('%', "Percent");
    map.insert('&', "Ampersand");
    map.insert('(', "LeftParens");
    map.insert(')', "RightParens");
    map.insert('*', "Asterisk");
    map.insert('+', "Plus");
    map.insert(',', "Comma");
    map.insert('-', "Dash");
    map.insert('.', "Period");
    map.insert('/', "ForeSlash");
    map.insert(':', "Colon");
    map.insert(';', "SemiColon");
    map.insert('<', "LessThan");
    map.insert('=', "Equals");
    map.insert('>', "GreaterThan");
    map.insert('?', "Question");
    map.insert('@', "At");
    map.insert('[', "LeftBracket");
    map.insert('\'', "SingleQuote");
    map.insert('\\', "BackSlash");
    map.insert(']', "RightBracket");
    map.insert('^', "Caret");
    map.insert('_', "Underscore");
    map.insert('`', "Backtick");
    map.insert('{', "LeftBrace");
    map.insert('|', "Pipe");
    map.insert('}', "RightBrace");
    map.insert('~', "Tilde");
    map
});

pub enum SpellingAlphabet {
    Nato,
}

pub struct PhoneticConverter {
    alphabet: &'static HashMap<char, &'static str>,
}

impl PhoneticConverter {
    #[must_use]
    pub fn new(alphabet_type: &SpellingAlphabet) -> Self {
        let alphabet = match alphabet_type {
            SpellingAlphabet::Nato => &*NATO_ALPHABET,
        };
        Self { alphabet }
    }

    #[must_use]
    pub fn convert(&self, text: &str) -> String {
        let mut result = String::new();
        for c in text.chars() {
            if let Some(word) = self.alphabet.get(&c.to_ascii_lowercase()) {
                if c.is_lowercase() {
                    result.push_str(&word.to_lowercase());
                } else if c.is_uppercase() {
                    result.push_str(&word.to_uppercase());
                } else {
                    result.push_str(word);
                }
            } else {
                result.push(c);
            }
            result.push(' ');
        }
        result.trim_end().to_owned()
    }
}
