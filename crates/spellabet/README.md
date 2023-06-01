# spellabet &emsp; [![MSRV]][rust-version]

[MSRV]: https://img.shields.io/badge/MSRV-1.64-blue
[rust-version]:
  https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field

**Convert characters into their equivalent spelling alphabet code words.**

---

spellabet is a Rust library for transforming text strings into their equivalent
code words based on predefined [spelling alphabets][]. These spelling alphabets,
such as the NATO phonetic alphabet, are designed to boost verbal clarity,
particluarly when spelling out words over low-fidelity voice channels. The
library supports multiple standard alphabets and allows for customization to
suit specific communication needs.

In its operation, spellabet will maintain the original capitalization of letters
by returning either lowercase or uppercase code words. Known digits and other
symbols undergo the same conversion process into code words. Unrecognized
characters are returned as is, without conversion.

[spelling alphabets]: https://en.wikipedia.org/wiki/Spelling_alphabet

## Usage

To use the crate, add it as dependency in your `Cargo.toml` file:

```toml
[dependencies]
spellabet = "0.1.0"
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

Detailed examples and generated API reference documentation can be found at
<https://earthmanmuons.github.io/spellout/spellabet/index.html>

## Minimum Supported Rust Version (MSRV) Policy

- We follow an "N-2 policy," supporting at least the current stable Rust release
  and the two preceding versions.
- Our MSRV only advances when we adopt a feature from a newer Rust version. We
  do not increase the MSRV systematically with each new release of Rust.
- MSRV increases are considered regular changes, not breaking changes, in terms
  of Semantic Versioning.

## License

spellabet is distributed under the terms of both the Apache License (Version
2.0) and the MIT License.

See [LICENSE-APACHE](../../LICENSE-APACHE) and [LICENSE-MIT](../../LICENSE-MIT)
for details.
