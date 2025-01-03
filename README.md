# spellout

**Convert characters into spelling alphabet code words.**

[![CI status](https://img.shields.io/github/actions/workflow/status/EarthmanMuons/spellout/on-pull-request.yml?event=merge_group&label=ci&logo=github)](https://github.com/EarthmanMuons/spellout/actions?query=event%3Amerge_group)

---

A command-line application for transforming text strings into corresponding code
words based on predefined [spelling alphabets][], like the NATO phonetic
alphabet. These alphabets are designed to enhance verbal clarity, especially
when spelling out words over low-fidelity voice channels.

[spelling alphabets]: https://en.wikipedia.org/wiki/Spelling_alphabet

## Examples

    $ spellout Example123!
    ECHO x-ray alfa mike papa lima echo One Two Tree Exclamation

    $ spellout --alphabet us-financial Example123!
    EDDIE xavier adam mary peter larry eddie One Two Three Exclamation

    $ spellout --nonce-form Rust
    'R' as in ROMEO, 'u' as in uniform, 's' as in sierra, 't' as in tango

    $ spellout --verbose Aaron "Bull Schaefer"
    Aaron -> ALFA alfa romeo oscar november
    Bull Schaefer -> BRAVO uniform lima lima Space SIERRA charlie hotel alfa echo foxtrot echo romeo

    $ spellout -- --help
    Dash Dash hotel echo lima papa

    $ spellout "So 📞 me, maybe?"
    SIERRA oscar Space 📞 Space mike echo Comma Space mike alfa yankee bravo echo Question

spellout will also read lines from standard input (stdin):

    $ cat secrets | spellout --verbose
    4PN%mAnt -> Fower PAPA NOVEMBER Percent mike ALFA november tango
    5Jzd}y(d -> Fife JULIETT zulu delta RightBrace yankee LeftParens delta
    BTW{2J~l -> BRAVO TANGO WHISKEY LeftBrace Two JULIETT Tilde lima

## Usage

In operation, spellout preserves the original capitalization of letters by
returning either lowercase or uppercase code words. It similarly converts known
digits and other symbols into code words, while unrecognized characters are
returned unconverted.

Each string will have its output printed on a separate line. Using `--` will
stop the program from interpreting subsequent arguments as options.

    Usage: spellout [OPTIONS] [STRING]...

    Arguments:
      [STRING]...  An input character string to convert into code words

    Options:
      -a, --alphabet <ALPHABET>    Which spelling alphabet to use for the conversion
      -o, --overrides <OVERRIDES>  Define overrides for spelling alphabet code words
          --dump-alphabet          Display the spelling alphabet and exit
      -n, --nonce-form             Expand output into nonce form like "'A' as in ALFA"
      -v, --verbose                Use verbose output
      -h, --help                   Print help (see more with '--help')
      -V, --version                Print version

### Environment Variables

Some options can alternatively be provided by setting environment variables (the
command-line arguments take precedence). To set the variables, use:
`export VARNAME=value`, where `VARNAME` is the name of the environment variable
and `value` is the desired setting.

##### `SPELLOUT_ALPHABET`

This environment variable determines the spelling alphabet to use for the
conversion.

Default: `nato`

Possible values:

- `jan`: Use the JAN (Joint Army/Navy) spelling alphabet.
- `lapd`: Use the LAPD (Los Angeles Police Department) spelling alphabet.
- `nato`: Use the NATO (North Atlantic Treaty Organization) spelling alphabet.
  This is the default setting.
- `royal-navy`: Use the Royal Navy spelling alphabet.
- `us-financial`: Use the United States Financial Industry spelling alphabet.
- `western-union`: Use the Western Union spelling alphabet.

##### `SPELLOUT_OVERRIDES`

Default: None

This environment variable allows you to define overrides for spelling alphabet
code words. Provide a comma-separated list of _character=word_ pairs like
`"a=apple,b=banana"`.

##### `SPELLOUT_NONCE_FORM`

Default: `false`

Setting this environment variable to any non-falsey value enables the nonce form
output, which expands conversions into a form like "'A' as in ALFA".

##### `SPELLOUT_VERBOSE`

Default: `false`

Setting this environment variable to any non-falsey value enables the verbose
output, which will include the input characters along with each line's output.

## Installation

### Precompiled Binaries

Precompiled binaries for various architectures are provided in the [Releases][]
section of our GitHub repository. Download the appropriate release archive for
your system and extract the contents.

The extracted directory contains:

- The `spellout` binary, which is the main application. Ensure to place it in a
  directory listed in your system's PATH to make it globally accessible.
- Auxiliary assets, including shell completions and a manual page in ROFF
  format. Install these according to your operating system's standard
  instructions.

[Releases]: https://github.com/EarthmanMuons/spellout/releases

### Building from Source

After [installing Rust][], to build the binary and install it on your system
under the `~/.cargo/bin/` directory, run the following command:

```
cargo install --locked --git https://github.com/EarthmanMuons/spellout/ spellout
```

[installing Rust]: https://www.rust-lang.org/learn/get-started

## Contribution

If you would like to contribute to the project, please read our [guide for
contributors][CONTRIBUTING.md].

[CONTRIBUTING.md]:
  https://github.com/EarthmanMuons/spellout/blob/main/CONTRIBUTING.md

## Credits

spellout was inspired by the output from the no-longer-in-existence [WinGuides
Secure Password Generator][WinGuides] that disappeared back in January 2007, and
the similarly inspired [Lingua::Alphabet::Phonetic::Password][Lingua] Perl
module written by [James FitzGibbon][@jf647].

[WinGuides]:
  https://web.archive.org/web/20070106073206/www.winguides.com/security/password.php
[Lingua]: https://github.com/jf647/Lingua-Alphabet-Phonetic-Password/
[@jf647]: https://github.com/jf647/

## License

spellout is released under the [Zero Clause BSD License][LICENSE] (SPDX: 0BSD).

Copyright &copy; 2023 [Aaron Bull Schaefer][EMAIL] and contributors

[LICENSE]: https://github.com/EarthmanMuons/spellout/blob/main/LICENSE
[EMAIL]: mailto:aaron@elasticdog.com
