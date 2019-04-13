//! This is a platform agnostic Rust driver for the KXCJ9 ultra-low-power
//! tri-axis accelerometer (up to +/-16g) using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! ## The device
//!
//! The KXCJ9 is a high-performance, ultra-low-power, tri-axis accelerometer
//! designed for mobile applications. It offers our best power performance
//! along with an embedded wake-up feature, Fast-mode I²C and up to 14-bit
//! resolution. The KXCJ9 sensor offers improved shock, reflow, and temperature
//! performance, and the ASIC has internal voltage regulators that allow
//! operation from 1.8 V to 3.6 V within the specified product performance.
//!
//! The communication is done through an I2C bidirectional bus.
//!
//! Datasheet:
//! - [KXCJ9-1008](http://kionixfs.kionix.com/en/datasheet/KXCJ9-1008%20Specifications%20Rev%205.pdf)
//! - [KXCJ9-1018](http://kionixfs.kionix.com/en/datasheet/KXCJ9-1018%20Specifications%20Rev%202.pdf)
//!
//! Application Note:
//! - [Getting started with the KXCJ9 and KXCJB](http://kionixfs.kionix.com/en/document/AN028%20Getting%20Started%20with%20the%20KXCJ9%20and%20KXCJB.pdf)
//!
#![deny(unsafe_code, missing_docs)]
#![no_std]

