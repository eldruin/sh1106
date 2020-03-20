# Changelog

[`sh1106`](https://crates.io/crates/sh1106) is a Rust driver for the SH1106 OLED display. It supports
[embedded-graphics](https://crates.io/crates/embedded-graphics) or raw pixel drawing modes and works
with the [embedded-hal](crates.io/crates/embedded-hal) traits for maximum portability.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- Migrate from Travis to CircleCI

### Changed

- **(breaking)** [#18](https://github.com/jamwaffles/sh1106/pull/18) Upgrade to embedded-graphics 0.6.0

## [0.3.0-alpha.4]

### Fixed

- Pin `embedded-graphics` dependency versio to `0.6.0-alpha.2`

## 0.3.0-alpha.3

### Added

- Added the `NoOutputPin` dummy pin type for SPI cases when no Chip Select pin is required. Use it like this:

```rust
let spi = Spi::spi1(
    // ...
);

let mut disp: GraphicsMode<_> = sh1106::Builder::new()
    .connect_spi(spi, dc, sh1106::NoOutputPin::new())
    .into();
```

## 0.3.0-alpha.2

Upgrade to new embedded-graphics `0.6.0-alpha.2` release. Please see the [embedded-graphics changelog](https://github.com/jamwaffles/embedded-graphics/blob/c0ed1700635f307a4c5114fec1769147878fd584/CHANGELOG.md) for more information.

### Changed

- **(breaking)** #11 Upgraded to [embedded-graphics](https://crates.io/crates/embedded-graphics) 0.6.0-alpha.2

## 0.3.0-alpha.1

Upgrade to new embedded-graphics `0.6.0-alpha.1` release. Please see the [embedded-graphics changelog](https://github.com/jamwaffles/embedded-graphics/blob/embedded-graphics-v0.6.0-alpha.1/CHANGELOG.md) for more information.

### Changed

- **(breaking)** #9 Upgraded to [embedded-graphics](https://crates.io/crates/embedded-graphics) 0.6.0-alpha.1

<!-- next-url -->

[unreleased]: https://github.com/jamwaffles/sh1106/compare/v0.3.0-alpha.4...HEAD
[0.3.0-alpha.4]: https://github.com/jamwaffles/sh1106/compare/v0.3.0-alpha.3...v0.3.0-alpha.4
[0.3.0-alpha.3]: https://github.com/jamwaffles/sh1106/compare/v0.3.0-alpha.2...v0.3.0-alpha.3
