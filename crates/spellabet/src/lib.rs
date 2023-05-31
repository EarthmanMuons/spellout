#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

use code_words::{DEFAULT_DIGITS_AND_SYMBOLS, LAPD_ALPHABET, NATO_ALPHABET, US_FINANCIAL_ALPHABET};
use convert_case::{Case, Casing};

mod code_words;

pub struct PhoneticConverter {
    conversion_map: HashMap<char, String>,
    nonce_form: bool,
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

        Self {
            conversion_map,
            nonce_form: false,
        }
    }

    #[must_use]
    pub const fn nonce_form(mut self, nonce_form: bool) -> Self {
        self.nonce_form = nonce_form;
        self
    }

    #[must_use]
    pub const fn mappings(&self) -> &HashMap<char, String> {
        &self.conversion_map
    }

    #[must_use]
    pub fn with_overrides(mut self, overrides: HashMap<char, String>) -> Self {
        let lower_overrides: HashMap<char, String> = overrides
            .into_iter()
            .map(|(k, v)| (k.to_ascii_lowercase(), v.to_case(Case::Pascal)))
            .collect();

        self.conversion_map.extend(lower_overrides);
        self
    }

    #[must_use]
    pub fn convert(&self, text: &str) -> String {
        let mut result = String::new();

        for (i, c) in text.chars().enumerate() {
            // add separator between converted characters
            if i != 0 {
                if self.nonce_form {
                    result.push_str(", ");
                } else {
                    result.push(' ');
                }
            }
            self.convert_char(c, &mut result);
        }
        result
    }

    fn convert_char(&self, character: char, result: &mut String) {
        match self.conversion_map.get(&character.to_ascii_lowercase()) {
            Some(word) => {
                let code_word = if character.is_lowercase() {
                    word.to_lowercase()
                } else if character.is_uppercase() {
                    word.to_uppercase()
                } else {
                    word.clone()
                };

                if self.nonce_form && character.is_alphabetic() {
                    result.push_str(&format!("'{character}' as in {code_word}"));
                } else {
                    result.push_str(&code_word);
                }
            }
            None => result.push(character),
        }
    }
}

impl SpellingAlphabet {
    #[must_use]
    pub fn initialize(&self) -> HashMap<char, String> {
        let mut map: HashMap<char, String> = HashMap::new();

        let extend_map = |map: &mut HashMap<char, String>, source_map: &[(char, &str)]| {
            map.extend(
                source_map
                    .iter()
                    .map(|(k, v)| (*k, (*v).to_string()))
                    .collect::<HashMap<char, String>>(),
            );
        };

        extend_map(&mut map, &DEFAULT_DIGITS_AND_SYMBOLS);

        match self {
            Self::Lapd => extend_map(&mut map, &LAPD_ALPHABET),
            Self::Nato => extend_map(&mut map, &NATO_ALPHABET),
            Self::UsFinancial => extend_map(&mut map, &US_FINANCIAL_ALPHABET),
        };

        map
    }
}
