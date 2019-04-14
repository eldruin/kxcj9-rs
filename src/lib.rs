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

extern crate embedded_hal as hal;
use core::marker::PhantomData;
use hal::blocking::i2c;

mod types;
pub use types::{Error, GScale16, GScale8, OutputDataRate, Resolution, SlaveAddr};

const DEVICE_BASE_ADDRESS: u8 = 0xE;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Config {
    bits: u8,
}

impl Config {
    fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

#[doc(hidden)]
pub mod ic {
    pub struct Kxcj9_1008(());
    pub struct Kxcj9_1018(());
}

/// KXCJ9 device driver.
#[derive(Debug)]
pub struct Kxcj9<I2C, IC> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    address: u8,
    ctrl1: Config,
    data_ctrl: u8,
    _ic: PhantomData<IC>,
}

mod kxcj9;

mod private {
    use super::ic;
    pub trait Sealed {}

    impl Sealed for ic::Kxcj9_1008 {}
    impl Sealed for ic::Kxcj9_1018 {}
}
