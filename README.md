# spellout &emsp; [![CI Status]][actions] [![MSRV]][rust-version]

[CI Status]:
  https://img.shields.io/github/actions/workflow/status/EarthmanMuons/spellout/rust.yml?event=merge_group&label=CI&logo=github
[actions]:
  https://github.com/EarthmanMuons/spellout/actions?query=event%3Amerge_group
[MSRV]: https://img.shields.io/badge/MSRV-1.64-blue
[rust-version]:
  https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field

**Convert characters into their equivalent spelling alphabet code words.**

---

## Usage

    $ spellout Example123
    ECHO x-ray alpha mike papa lima echo One Two Three

## Installation

To build the binary and install it on your system under the `~/.cargo/bin`
directory, run the following command:

```
cargo install --locked --git https://github.com/EarthmanMuons/spellout/ spellout
```

## Documentation

Generated [Rustdoc][] reference documentation can be found at
<https://earthmanmuons.github.io/spellout/>

[Rustdoc]: https://doc.rust-lang.org/stable/rustdoc/

## Minimum Supported Rust Version (MSRV) Policy

- We follow an "N-2 policy," supporting at least the current stable Rust release
  and the two preceding versions.
- Our MSRV only advances when we adopt a feature from a newer Rust version. We
  do not increase the MSRV systematically with each new release of Rust.
- MSRV increases are considered regular changes, not breaking changes, in terms
  of Semantic Versioning.

## License

RustOps Blueprint is distributed under the terms of both the Apache License
(Version 2.0) and the MIT License.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

If you would like to contribute to the project, please read our
[guide for contributors](CONTRIBUTING.md).
