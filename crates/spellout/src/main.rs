use std::collections::HashMap;
use std::io::{self, BufRead};

use anyhow::{Context, Result};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal;
use spellabet::{PhoneticConverter, SpellingAlphabet};

use crate::cli::{Alphabet, Cli};

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let alphabet = match cli.alphabet {
        Alphabet::Lapd => SpellingAlphabet::Lapd,
        Alphabet::Nato => SpellingAlphabet::Nato,
        Alphabet::UsFinancial => SpellingAlphabet::UsFinancial,
    };

    let mut converter = PhoneticConverter::new(&alphabet).nonce_form(cli.nonce_form);

    if let Some(overrides_str) = cli.overrides {
        let overrides_map = parse_overrides(&overrides_str).context("Failed to parse overrides")?;
        converter = converter.with_overrides(overrides_map);
    }

    if cli.dump_alphabet {
        converter.dump_alphabet(&mut io::stdout(), cli.verbose)?;
        return Ok(());
    }

    if cli.input.is_empty() {
        // Check standard input
        if io::stdin().is_terminal() {
            // No data was provided to stdin
            let mut cmd = Cli::command();
            eprintln!("{}\n", cmd.render_usage());
            let mut err = clap::Error::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
            err.insert(
                ContextKind::InvalidArg,
                ContextValue::Strings(vec!["[STRING]...".to_string()]),
            );
            err.exit();
        } else {
            // Data was provided to stdin
            for line in io::stdin().lock().lines() {
                let input = line.context("Failed to read line from stdin")?;
                process_input(&input, &converter, cli.verbose);
            }
        }
        return Ok(());
    }

    for input in cli.input {
        process_input(&input, &converter, cli.verbose);
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
    let mut overrides_map = HashMap::new();

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

        overrides_map.insert(key, parts[1].to_string());
    }

    Ok(overrides_map)
}

#[test]
fn verify_cli() {
    Cli::command().debug_assert()
}
