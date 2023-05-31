use std::char;
use std::cmp::Ordering;
use std::io::{self, BufRead};

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

    /// Display the spelling alphabet and exit
    ///
    /// Shows only letters by default; add the `--verbose` flag to also show
    /// digits and symbols
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
    /// input
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

fn main() {
    let cli = Cli::parse();

    let alphabet = match cli.alphabet {
        Alphabet::Lapd => SpellingAlphabet::Lapd,
        Alphabet::Nato => SpellingAlphabet::Nato,
        Alphabet::UsFinancial => SpellingAlphabet::UsFinancial,
    };

    if cli.dump_alphabet {
        dump_alphabet(&alphabet, cli.verbose);
        return;
    }

    let converter = PhoneticConverter::new(&alphabet).nonce_form(cli.nonce_form);
    if !cli.input.is_empty() {
        for input in cli.input {
            process_input(&input, &converter, cli.verbose);
        }
    } else if io::stdin().is_terminal() {
        let cmd = Cli::command();
        let mut err = clap::Error::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
        err.insert(
            ContextKind::InvalidArg,
            ContextValue::Strings(vec!["<STRING>...".to_string()]),
        );
        err.exit();
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let input = line.expect("Failed to read line from stdin");
            process_input(&input, &converter, cli.verbose);
        }
    }
}

fn process_input(input: &str, converter: &PhoneticConverter, verbose: bool) {
    if verbose {
        print!("{input} -> ");
    }
    println!("{}", converter.convert(input));
}

fn dump_alphabet(alphabet: &SpellingAlphabet, verbose: bool) {
    let mut entries: Vec<_> = alphabet.initialize().into_iter().collect();
    entries.sort_by(|a, b| custom_char_ordering(&a.0, &b.0));
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
