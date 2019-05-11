# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## [0.2.0] - 2019-05-11

This driver is now functionally complete.

### Added
- Enable/disable MEMS self-test function.
- Support interrupts.
- Support physical interrupt pin configuration.
- Support KXCJB device.

### Changed
- [breaking-change] Renamed constructor methods `new_1008` -> `new_kxcj9_1008`
  and `new_1018` -> `new_kxcj9_1018` as support for device KXCJB has been added.
- [breaking-change] Renamed communication self-test function `self_test()` to
  `communication_self_test()`.

## 0.1.0 - 2019-04-28

This is the initial release to crates.io. All changes will be documented in
this CHANGELOG.

[Unreleased]: https://github.com/eldruin/kxcj9-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eldruin/kxcj9-rs/compare/v0.1.0...v0.2.0

