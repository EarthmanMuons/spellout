use clap::{Parser, ValueEnum};
use spellabet::{PhoneticConverter, SpellingAlphabet};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Which spelling alphabet to use for the conversion
    #[arg(short, long, env = "SPELLOUT_ALPHABET")]
    #[arg(value_enum, default_value_t = Alphabet::Nato)]
    alphabet: Alphabet,

    /// Expand output into nonce form like "'A' as in ALFA"
    #[arg(short, long, env = "SPELLOUT_NONCE_FORM")]
    #[arg(value_parser = clap::builder::BoolishValueParser::new())]
    nonce_form: bool,

    /// Display the spelling alphabet and exit
    #[arg(long)]
    dump_alphabet: bool,

    /// The input character string to convert into code words
    #[arg(required_unless_present("dump_alphabet"))]
    input: Option<String>,
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
        dump_alphabet(&alphabet);
    } else if let Some(input) = cli.input {
        let converter = PhoneticConverter::new(&alphabet).nonce_form(cli.nonce_form);
        println!("{}", converter.convert(&input));
    }
}

fn dump_alphabet(alphabet: &SpellingAlphabet) {
    let mut entries: Vec<_> = alphabet.initialize().into_iter().collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    for (character, code_word) in entries {
        if character.is_alphabetic() {
            println!("{character} -> {code_word}");
        }
    }
}
