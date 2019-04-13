# Rust KXCJ9 Ultra-Low-Power Tri-Axis Accelerometer

[![crates.io](https://img.shields.io/crates/v/kxcj9.svg)](https://crates.io/crates/kxcj9)
[![Docs](https://docs.rs/kxcj9/badge.svg)](https://docs.rs/kxcj9)
[![Build Status](https://travis-ci.org/eldruin/kxcj9-rs.svg?branch=master)](https://travis-ci.org/eldruin/kxcj9-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/kxcj9-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/kxcj9-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the KXCJ9 ultra-low-power tri-axis accelerometer
(up to +/-16g) using the [`embedded-hal`] traits.

## The device

The KXCJ9 is a high-performance, ultra-low-power, tri-axis accelerometer designed for mobile applications. It offers our best power performance along with an embedded wake-up feature, Fast-mode I²C and up to 14-bit resolution. The KXCJ9 sensor offers improved shock, reflow, and temperature performance, and the ASIC has internal voltage regulators that allow operation from 1.8 V to 3.6 V within the specified product performance.

The communication is done through an I2C bidirectional bus.

Datasheet:
- [KXCJ9-1008](http://kionixfs.kionix.com/en/datasheet/KXCJ9-1008%20Specifications%20Rev%205.pdf)
- [KXCJ9-1018](http://kionixfs.kionix.com/en/datasheet/KXCJ9-1018%20Specifications%20Rev%202.pdf)

Application Note:
- [Getting started with the KXCJ9 and KXCJB](http://kionixfs.kionix.com/en/document/AN028%20Getting%20Started%20with%20the%20KXCJ9%20and%20KXCJB.pdf)

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/kxcj9-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
