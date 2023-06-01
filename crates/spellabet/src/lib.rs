#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

//! # Spelling Alphabet
//!
//! A library for converting text string characters into their equivalent
//! [spelling alphabet](https://en.wikipedia.org/wiki/Spelling_alphabet) code words.
//!
//! In its operation, spellabet will maintain the original capitalization of
//! letters by returning either lowercase or uppercase code words. Known digits
//! and other symbols undergo the same conversion process into code words.
//! Unrecognized characters are returned as is, without conversion.
//!
//! # Example
//!
//! ```
//! use spellabet::{PhoneticConverter, SpellingAlphabet};
//!
//! let converter = PhoneticConverter::new(&SpellingAlphabet::Nato);
//! println!("{}", converter.convert("Example123!"));
//! ```
//!
//! ```text
//! ECHO x-ray alfa mike papa lima echo One Two Tree Exclamation
//! ```

use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;

use code_words::{DEFAULT_DIGITS_AND_SYMBOLS, LAPD_ALPHABET, NATO_ALPHABET, US_FINANCIAL_ALPHABET};
use convert_case::{Case, Casing};

mod code_words;

/// A phonetic converter.
pub struct PhoneticConverter {
    /// The map of characters to code words.
    conversion_map: HashMap<char, String>,
    /// Is set when the code word output will be in "nonce form".
    nonce_form: bool,
}

/// A spelling alphabet.
#[derive(Default)]
pub enum SpellingAlphabet {
    /// The LAPD (Los Angeles Police Department) spelling alphabet.
    Lapd,
    /// The NATO (North Atlantic Treaty Organization) spelling alphabet.
    /// This is the default.
    #[default]
    Nato,
    /// The United States Financial Industry spelling alphabet.
    UsFinancial,
}

impl PhoneticConverter {
    /// Creates and returns a new instance of `PhoneticConverter` using the
    /// desired spelling alphabet character mappings.
    ///
    /// # Arguments
    ///
    /// * `alphabet` - The [`SpellingAlphabet`] to use for character
    ///   conversions.
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// # use spellabet::{PhoneticConverter, SpellingAlphabet};
    /// let converter = PhoneticConverter::new(&SpellingAlphabet::default());
    /// ```
    #[must_use]
    pub fn new(alphabet: &SpellingAlphabet) -> Self {
        let conversion_map = alphabet.initialize();

        Self {
            conversion_map,
            nonce_form: false,
        }
    }

    /// Get the current character mappings of the `PhoneticConverter` instance.
    #[must_use]
    pub const fn mappings(&self) -> &HashMap<char, String> {
        &self.conversion_map
    }

    /// Configures the current `PhoneticConverter` instance to either output
    /// code words in "nonce form" or not, based on the given boolean value.
    ///
    /// Nonce form means each letter character is expanded into the form "'A' as
    /// in ALFA". Digits and symbols are always returned using the normal output
    /// format.
    ///
    /// # Arguments
    ///
    /// * `nonce_form` - If true, enables nonce form output. Otherwise, the
    ///   normal output format is used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spellabet::{PhoneticConverter, SpellingAlphabet};
    /// let converter = PhoneticConverter::new(&SpellingAlphabet::default()).nonce_form(true);
    /// println!("{}", converter.convert("Hello"));
    /// ```
    ///
    /// ```text
    /// 'H' as in HOTEL, 'e' as in echo, 'l' as in lima, 'l' as in lima, 'o' as in oscar
    /// ```
    #[must_use]
    pub const fn nonce_form(mut self, nonce_form: bool) -> Self {
        self.nonce_form = nonce_form;
        self
    }

    /// Modifies the conversion map of the current `PhoneticConverter` instance
    /// by adding or replacing mappings based on the given overrides map.
    ///
    /// # Arguments
    ///
    /// * `overrides_map` - The desired character to code word mappings to
    ///   override. The capitalization of the keys and values will be
    ///   automatically normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// # use spellabet::{PhoneticConverter, SpellingAlphabet};
    ///
    /// let mut converter = PhoneticConverter::new(&SpellingAlphabet::default());
    ///
    /// let mut overrides_map = HashMap::new();
    /// overrides_map.insert('a', "Apple".to_string());
    /// overrides_map.insert('b', "Banana".to_string());
    ///
    /// println!("BEFORE: {}", converter.convert("abcd"));
    /// ```
    ///
    /// ```text
    /// BEFORE: alfa bravo charlie delta
    /// ```
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use spellabet::{PhoneticConverter, SpellingAlphabet};
    /// # let mut converter = PhoneticConverter::new(&SpellingAlphabet::default());
    /// # let mut overrides_map = HashMap::new();
    /// # overrides_map.insert('a', "Apple".to_string());
    /// # overrides_map.insert('b', "Banana".to_string());
    /// converter = converter.with_overrides(overrides_map);
    /// println!("AFTER: {}", converter.convert("abcd"));
    /// ```
    ///
    /// ```text
    /// AFTER: apple banana charlie delta
    /// ```
    #[must_use]
    pub fn with_overrides(mut self, overrides_map: HashMap<char, String>) -> Self {
        let normalized_overrides: HashMap<char, String> = overrides_map
            .into_iter()
            .map(|(k, v)| (k.to_ascii_lowercase(), v.to_case(Case::Pascal)))
            .collect();

        self.conversion_map.extend(normalized_overrides);
        self
    }

    /// Converts the given text into a string of code words using the current
    /// character mappings of the `PhoneticConverter` instance.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to convert into code words.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spellabet::{PhoneticConverter, SpellingAlphabet};
    /// let converter = PhoneticConverter::new(&SpellingAlphabet::default());
    /// assert_eq!(converter.convert("Hello"), "HOTEL echo lima lima oscar");
    /// ```
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
                let code_word = match character {
                    _ if character.is_lowercase() => word.to_lowercase(),
                    _ if character.is_uppercase() => word.to_uppercase(),
                    _ => word.clone(),
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

    /// Writes the current character mappings of the `PhoneticConverter`
    /// instance to the given writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The output destination.
    /// * `verbose` - If true, dumps all characters. Otherwise, dumps only
    ///   letter characters.
    ///
    /// # Errors
    ///
    /// This function will return an error if writing to the provided writer
    /// fails. The specific conditions under which this may occur depend on the
    /// nature of the writer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spellabet::{PhoneticConverter, SpellingAlphabet};
    /// let converter = PhoneticConverter::new(&SpellingAlphabet::default());
    ///
    /// let mut buf = Vec::new();
    /// let verbose = false;
    /// converter.dump_alphabet(&mut buf, verbose)?;
    /// let output = String::from_utf8(buf)?;
    /// println!("{output}");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ```text
    /// a -> Alfa
    /// b -> Bravo
    /// c -> Charlie
    /// ...
    /// ```
    pub fn dump_alphabet(
        &self,
        mut writer: impl std::io::Write,
        verbose: bool,
    ) -> std::io::Result<()> {
        let mut entries: Vec<_> = self.conversion_map.iter().collect();
        entries.sort_by(|a, b| custom_char_ordering(*a.0, *b.0));
        for (character, code_word) in entries {
            if verbose || character.is_alphabetic() {
                writeln!(writer, "{character} -> {code_word}")?;
            }
        }
        Ok(())
    }
}

// Sort characters in the order of letters before digits before symbols.
// Within each group, characters will be sorted in their natural order.
fn custom_char_ordering(a: char, b: char) -> Ordering {
    match (
        a.is_alphabetic(),
        b.is_alphabetic(),
        a.is_numeric(),
        b.is_numeric(),
    ) {
        (true, false, _, _) | (false, false, true, false) => Ordering::Less,
        (false, true, _, _) | (false, false, false, true) => Ordering::Greater,
        _ => a.cmp(&b),
    }
}

impl SpellingAlphabet {
    /// Generates and returns a character to code word map based on the current
    /// `SpellingAlphabet`.
    #[must_use]
    pub fn initialize(&self) -> HashMap<char, String> {
        let mut map: HashMap<char, String> = HashMap::new();

        let extend_map = |map: &mut HashMap<char, String>, source_map: &[(char, &str)]| {
            for (k, v) in source_map {
                map.insert(*k, (*v).to_string());
            }
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
