#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

use code_words::{DEFAULT_DIGITS_AND_SYMBOLS, LAPD_ALPHABET, NATO_ALPHABET, US_FINANCIAL_ALPHABET};

mod code_words;

pub struct PhoneticConverter {
    conversion_map: HashMap<char, String>,
}

pub enum SpellingAlphabet {
    Lapd,
    Nato,
    UsFinancial,
}

impl PhoneticConverter {
    #[must_use]
    pub fn new(alphabet: &SpellingAlphabet) -> Self {
        let conversion_map = alphabet.initialize();
        Self { conversion_map }
    }

    #[must_use]
    pub fn convert(&self, text: &str) -> String {
        let mut result = String::new();
        for c in text.chars() {
            if let Some(word) = self.conversion_map.get(&c.to_ascii_lowercase()) {
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

impl SpellingAlphabet {
    #[must_use]
    pub fn initialize(&self) -> HashMap<char, String> {
        let mut map: HashMap<char, String> = HashMap::new();
        extend_map(&mut map, &DEFAULT_DIGITS_AND_SYMBOLS);

        match self {
            Self::Lapd => extend_map(&mut map, &LAPD_ALPHABET),
            Self::Nato => extend_map(&mut map, &NATO_ALPHABET),
            Self::UsFinancial => extend_map(&mut map, &US_FINANCIAL_ALPHABET),
        };

        map
    }
}

fn extend_map(map: &mut HashMap<char, String>, source_map: &[(char, &str)]) {
    map.extend(
        source_map
            .iter()
            .map(|(k, v)| (*k, (*v).to_string()))
            .collect::<HashMap<char, String>>(),
    );
}
