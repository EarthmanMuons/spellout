# spellabet

**Convert characters into spelling alphabet code words.**

[![CI status](https://img.shields.io/github/actions/workflow/status/EarthmanMuons/spellout/on-pull-request.yml?event=merge_group&label=ci&logo=github)](https://github.com/EarthmanMuons/spellout/actions?query=event%3Amerge_group)
[![crates.io](https://img.shields.io/crates/v/spellabet)](https://crates.io/crates/spellabet/)
[![docs.rs](https://img.shields.io/docsrs/spellabet)](https://docs.rs/spellabet/0.2.0/spellabet/)
[![MSRV](https://img.shields.io/badge/rust-1.75%2B-blue)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)

---

A Rust library for transforming text strings into corresponding code words based
on predefined [spelling alphabets][], like the NATO phonetic alphabet. These alphabets
are designed to enhance verbal clarity, especially when spelling out words over low-fidelity
voice channels. This library supports several standard alphabets and allows for customization
to suit specific communication needs.

In operation, spellabet preserves the original capitalization of letters by
returning either lowercase or uppercase code words. It similarly converts known
digits and other symbols into code words, while unrecognized characters are
returned unconverted.

This library powers the command line utility `spellout`, which provides a handy
interface for phonetic conversions. Check out [spellout on GitHub][] for more
information.

[spelling alphabets]: https://en.wikipedia.org/wiki/Spelling_alphabet
[spellout on GitHub]: https://github.com/EarthmanMuons/spellout/

## Usage

To use the crate, add it as dependency in your `Cargo.toml` file:

```toml
[dependencies]
spellabet = "0.2.0"
```

### Example

```rust
use spellabet::{PhoneticConverter, SpellingAlphabet};

let converter = PhoneticConverter::new(&SpellingAlphabet::Nato);
println!("{}", converter.convert("Example123!"));
```

```
ECHO x-ray alfa mike papa lima echo One Two Tree Exclamation
```

## Documentation

For detailed examples of using this library, along with the latest generated API
reference documentation, please visit
<https://earthmanmuons.github.io/spellout/spellabet/index.html>.

## Minimum Supported Rust Version (MSRV) Policy

- We follow an "N-2 policy," supporting at least the current stable Rust release
  and the two preceding versions.
- Our MSRV only advances when we adopt a feature from a newer Rust version. We
  do not increase the MSRV systematically with each new release of Rust.
- MSRV increases are considered regular changes, not breaking changes, in terms
  of Semantic Versioning.

## Contribution

If you would like to contribute to the project, please read our [guide for
contributors][CONTRIBUTING.md].

[CONTRIBUTING.md]:
  https://github.com/EarthmanMuons/spellout/blob/main/CONTRIBUTING.md

## Credits

spellabet was inspired by the output from the no-longer-in-existence [WinGuides
Secure Password Generator][WinGuides] that disappeared back in January 2007, and
the similarly inspired [Lingua::Alphabet::Phonetic::Password][Lingua] Perl
module written by [James FitzGibbon][@jf647].

[WinGuides]:
  https://web.archive.org/web/20070106073206/www.winguides.com/security/password.php
[Lingua]: https://github.com/jf647/Lingua-Alphabet-Phonetic-Password/
[@jf647]: https://github.com/jf647/

## License

spellabet is released under the [Zero Clause BSD License][LICENSE] (SPDX: 0BSD).

Copyright &copy; 2023 [Aaron Bull Schaefer][EMAIL] and contributors

[LICENSE]: https://github.com/EarthmanMuons/spellout/blob/main/LICENSE
[EMAIL]: mailto:aaron@elasticdog.com
