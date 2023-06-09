use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, about, long_about = None)]
#[command(version = get_version())]
pub struct Cli {
    /// Which spelling alphabet to use for the conversion
    #[arg(short, long, env = "SPELLOUT_ALPHABET")]
    #[arg(value_enum, default_value_t = Alphabet::Nato)]
    pub alphabet: Alphabet,

    /// Define overrides for spelling alphabet code words
    ///
    /// Provide a comma-separated list of character=word pairs like
    /// "a=apple,b=banana" which will override the default values.
    #[arg(short, long, env = "SPELLOUT_OVERRIDES")]
    pub overrides: Option<String>,

    /// Display the spelling alphabet and exit
    ///
    /// Shows only letters by default; add the `--verbose` flag to also show
    /// digits and symbols.
    #[arg(long)]
    pub dump_alphabet: bool,

    /// Expand output into nonce form like "'A' as in ALFA"
    #[arg(short, long, env = "SPELLOUT_NONCE_FORM")]
    #[arg(value_parser = clap::builder::FalseyValueParser::new())]
    pub nonce_form: bool,

    /// Use verbose output
    ///
    /// Include the input characters along with each line's output.
    #[arg(short, long, env = "SPELLOUT_VERBOSE")]
    #[arg(value_parser = clap::builder::FalseyValueParser::new())]
    pub verbose: bool,

    /// An input character string to convert into code words
    ///
    /// Each string will have its output printed on a separate line. Using `--`
    /// will stop the program from interpreting subsequent arguments as
    /// options. If no input strings are provided, lines will be read from
    /// standard input.
    #[arg(value_name = "STRING")]
    pub input: Vec<String>,

    /// Generate auxiliary asset files
    ///
    /// The requested asset file will be printed to standard output.
    #[arg(long, value_enum, value_name = "ASSET")]
    #[arg(hide_short_help = true)]
    pub generate: Option<Asset>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Alphabet {
    /// Joint Army/Navy
    Jan,
    /// Los Angeles Police Department
    Lapd,
    /// North Atlantic Treaty Organization
    Nato,
    /// Royal Navy
    RoyalNavy,
    /// United States Financial Industry
    UsFinancial,
    /// Western Union
    WesternUnion,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Asset {
    /// Manual page in ROFF format
    ManPage,
    /// Completions file for Bash shell
    Bash,
    /// Completions file for Elvish shell
    Elvish,
    /// Completions file for Fish shell
    Fish,
    /// Completions file for PowerShell
    Powershell,
    /// Completions file for Z shell
    Zsh,
}

fn get_version() -> &'static str {
    // fallback if compiling from a source tarball without git
    option_env!("SPELLOUT_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))
}
