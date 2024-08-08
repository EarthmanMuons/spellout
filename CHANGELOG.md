# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][1], and this project adheres to
[Semantic Versioning][2].

[1]: https://keepachangelog.com/en/1.1.0/
[2]: https://semver.org/spec/v2.0.0.html

<!-- next-header -->

## [Unreleased] <!-- release-date -->

### Changed

- Relicense project from MIT to Zero Clause BSD (0BSD).

## [0.2.1] - 2023-11-29

### Changed

- Bump MSRV to the Rust 1.75 release.

### Security

- Fix moderate severity security issue with transient dependency rustix:
  <https://github.com/advisories/GHSA-c827-hfw6-qwvm>

## [0.2.0] - 2023-07-25

### Added

- Add the Joint Army/Navy spelling alphabet.
- Add the Royal Navy spelling alphabet.
- Add the Western Union spelling alphabet.

### Changed

- Mention `--` parsing behavior in help output.
- Bump MSRV to the Rust 1.65 release.

### Fixed

- Fix formatting for automated version bumps across docs.

## [0.1.0] - 2023-06-10

### Added

- Initial release.

<!-- next-url -->

[Unreleased]: https://github.com/EarthmanMuons/spellout/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/EarthmanMuons/spellout/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/EarthmanMuons/spellout/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/EarthmanMuons/spellout/commits/v0.1.0
