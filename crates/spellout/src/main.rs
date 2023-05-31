use clap::{Parser, ValueEnum};
use spellabet::{PhoneticConverter, SpellingAlphabet};

#[derive(Parser, Debug)]
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

    /// The input characters to convert into code words
    input: String,
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

    let converter = PhoneticConverter::new(&alphabet).nonce_form(cli.nonce_form);
    println!("{}", converter.convert(&cli.input));
}
