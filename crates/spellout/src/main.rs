use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

use anyhow::{Context, Result};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{CommandFactory, Parser, ValueEnum};
use is_terminal::IsTerminal;
use spellabet::{PhoneticConverter, SpellingAlphabet};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Which spelling alphabet to use for the conversion
    #[arg(short, long, env = "SPELLOUT_ALPHABET")]
    #[arg(value_enum, default_value_t = Alphabet::Nato)]
    alphabet: Alphabet,

    /// Define overrides for spelling alphabet code words
    ///
    /// Provide a comma-separated list of character=word pairs like
    /// "a=apple,b=banana" which will override the default values.
    #[arg(short, long, env = "SPELLOUT_OVERRIDES")]
    overrides: Option<String>,

    /// Display the spelling alphabet and exit
    ///
    /// Shows only letters by default; add the `--verbose` flag to also show
    /// digits and symbols.
    #[arg(long)]
    dump_alphabet: bool,

    /// Expand output into nonce form like "'A' as in ALFA"
    #[arg(short, long, env = "SPELLOUT_NONCE_FORM")]
    #[arg(value_parser = clap::builder::BoolishValueParser::new())]
    nonce_form: bool,

    /// Use verbose output
    #[arg(short, long, env = "SPELLOUT_VERBOSE")]
    #[arg(value_parser = clap::builder::BoolishValueParser::new())]
    verbose: bool,

    /// An input character string to convert into code words
    ///
    /// If no input strings are provided, the program reads lines from standard
    /// input.
    #[arg(value_name = "STRING")]
    input: Vec<String>,
}

#[derive(Clone, Debug, ValueEnum)]
enum Alphabet {
    /// Los Angeles Police Department (LAPD)
    Lapd,
    /// North Atlantic Treaty Organization (NATO)
    Nato,
    /// United States Financial Industry
    UsFinancial,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let alphabet = match cli.alphabet {
        Alphabet::Lapd => SpellingAlphabet::Lapd,
        Alphabet::Nato => SpellingAlphabet::Nato,
        Alphabet::UsFinancial => SpellingAlphabet::UsFinancial,
    };

    let mut converter = PhoneticConverter::new(&alphabet).nonce_form(cli.nonce_form);

    if let Some(overrides_str) = cli.overrides {
        let overrides = parse_overrides(&overrides_str).context("Failed to parse overrides")?;
        converter = converter.with_overrides(overrides);
    }

    if cli.dump_alphabet {
        dump_alphabet(converter, cli.verbose);
        return Ok(());
    }

    if !cli.input.is_empty() {
        // The user provided an input string argument
        for input in cli.input {
            process_input(&input, &converter, cli.verbose);
        }
    } else if io::stdin().is_terminal() {
        // The user did not provide an input string argument nor data piped from stdin
        let cmd = Cli::command();
        let mut err = clap::Error::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
        err.insert(
            ContextKind::InvalidArg,
            ContextValue::Strings(vec!["<STRING>...".to_string()]),
        );
        err.exit();
    } else {
        // The user provided data piped from stdin
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let input = line.context("Failed to read line from stdin")?;
            process_input(&input, &converter, cli.verbose);
        }
    }

    Ok(())
}

fn process_input(input: &str, converter: &PhoneticConverter, verbose: bool) {
    if verbose {
        print!("{input} -> ");
    }
    println!("{}", converter.convert(input));
}

fn parse_overrides(input: &str) -> Result<HashMap<char, String>> {
    let mut overrides = HashMap::new();

    for s in input.split(',') {
        let parts: Vec<&str> = s.split('=').collect();

        if parts.len() < 2 {
            anyhow::bail!("Invalid override (missing '='): {s}");
        }
        if parts.len() > 2 {
            anyhow::bail!("Invalid override (extra '='): {s}");
        }
        if parts[0].len() != 1 {
            anyhow::bail!("Key in override is not a single character: {s}");
        }

        let key = parts[0].chars().next().unwrap(); // safe to unwrap because we checked the length

        if parts[1].is_empty() {
            anyhow::bail!("Empty value in override: {s}");
        }

        overrides.insert(key, parts[1].to_string());
    }

    Ok(overrides)
}

fn dump_alphabet(converter: PhoneticConverter, verbose: bool) {
    let mut entries: Vec<_> = converter.mappings().iter().collect();
    entries.sort_by(|a, b| custom_char_ordering(a.0, b.0));
    for (character, code_word) in entries {
        if verbose || character.is_alphabetic() {
            println!("{character} -> {code_word}");
        }
    }
}

// Sort all characters in the order of letters before digits before symbols.
// Within each group, characters will be sorted in their natural order.
fn custom_char_ordering(a: &char, b: &char) -> Ordering {
    match (
        a.is_alphabetic(),
        b.is_alphabetic(),
        a.is_numeric(),
        b.is_numeric(),
    ) {
        (true, false, _, _) => Ordering::Less,
        (false, true, _, _) => Ordering::Greater,
        (false, false, true, false) => Ordering::Less,
        (false, false, false, true) => Ordering::Greater,
        _ => a.cmp(b),
    }
}

#[test]
fn verify_cli() {
    Cli::command().debug_assert()
}
