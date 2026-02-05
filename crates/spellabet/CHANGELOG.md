# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][1], and this project adheres to
[Semantic Versioning][2].

[1]: https://keepachangelog.com/en/1.1.0/
[2]: https://semver.org/spec/v2.0.0.html

<!-- next-header -->

## [Unreleased] <!-- release-date -->

### Added

- Derive common traits for `SpellingAlphabet` and `PhoneticConverter`, plus a
  `Default` implementation for `PhoneticConverter`.

### Changed

- Relicense project from MIT to Zero Clause BSD (0BSD).
- Bump MSRV to the Rust 1.81 release.
- Allow Unicode override keys and apply Unicode-aware lowercase normalization
  when possible.

### Changed

- Bump MSRV to the Rust 1.75 release.

## [0.2.0] - 2023-07-25

### Added

- Add the Joint Army/Navy spelling alphabet.
- Add the Royal Navy spelling alphabet.
- Add the Western Union spelling alphabet.

### Changed

- Bump MSRV to the Rust 1.65 release.

## [0.1.1] - 2023-06-10

### Changed

- Rely on docs.rs for versioned library documentation.

### Fixed

- Fix formatting for automated version bumps across docs.

## [0.1.0] - 2023-06-08

### Added

- Initial release; published to https://crates.io/.

<!-- next-url -->

[Unreleased]:
  https://github.com/EarthmanMuons/spellout/compare/spellabet-v0.2.0...HEAD
[0.2.0]:
  https://github.com/EarthmanMuons/spellout/compare/spellabet-v0.1.1...spellabet-v0.2.0
[0.1.1]:
  https://github.com/EarthmanMuons/spellout/compare/spellabet-v0.1.0...spellabet-v0.1.1
[0.1.0]: https://github.com/EarthmanMuons/spellout/commits/spellabet-v0.1.0
