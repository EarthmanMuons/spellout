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

To use the library, add it as dependency in your `Cargo.toml` file:

```toml
[dependencies]
spellabet = "0.1.0"
```

Then, import the `PhoneticConverter` struct into your Rust code:

```rust
use spellabet::{PhoneticConverter, SpellingAlphabet};
```

### Creating a Converter

To create a `PhoneticConverter` instance, use the `new` method and specify the
desired spelling alphabet:

```rust
let alphabet = SpellingAlphabet::default();
let converter = PhoneticConverter::new(&alphabet);
```

#### Alphabet Options

The library supports the following spelling alphabets:

- `SpellingAlphabet::Lapd`: The LAPD (Los Angeles Police Department) spelling
  alphabet.
- `SpellingAlphabet::Nato`: The NATO (North Atlantic Treaty Organization)
  spelling alphabet (default).
- `SpellingAlphabet::UsFinancial`: The United States Financial Industry spelling
  alphabet.

### Converting Characters

To convert characters into their spelling alphabet code words, use the `convert`
method:

```rust
let text = "Hello";
let result = converter.convert(text);
println!("Result: {result}");
```

The above code will output:

```
Result: HOTEL echo lima lima oscar
```

### Nonce Form

The `PhoneticConverter` also supports "nonce form" where each converted
character is expanded into the form "'A' as in ALFA". To enable the nonce form
output, use the `nonce_form` method:

```rust
let converter = converter.nonce_form(true)
```

### Customizing the Conversion Map

You can override characters in the default conversion map by using the
`with_overrides` method and providing a `HashMap<char, String>` containing the
desired mappings:

```rust
use std::collections::HashMap;

let mut overrides = HashMap::new();
overrides.insert('A', "Apple".to_string());
overrides.insert('B', "Banana".to_string());

let converter = converter.with_overrides(overrides);
```

### Dumping the Conversion Map

To dump the current conversion map, you can use the `dump_alphabet` method. It
writes the map to a provided writer.

```rust
use std::fs::File;
use std::io::Write;

let mut file = File::create("alphabet.txt")?;
converter.dump_alphabet(&mut file, false)?;
```

This will create a file named "alphabet.txt" with the following content:

```
A -> Alfa
B -> Bravo
...
```

## Documentation

Generated [Rustdoc][] reference documentation can be found at
<https://earthmanmuons.github.io/spellout/spellabet/index.html>

[Rustdoc]: https://doc.rust-lang.org/stable/rustdoc/

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
