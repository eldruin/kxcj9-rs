//! This is a platform agnostic Rust driver for the KXCJ9 ultra-low-power
//! tri-axis accelerometer (up to +/-16g) using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device. See [`enable()`].
//! - Read the acceleration measurement. See [`read()`].
//! - Read the unscaled acceleration measurement. See [`read_unscaled()`].
//! - Set resolution. See [`set_resolution()`].
//! - Set output data rate. See [`set_output_data_rate()`].
//! - Set +/- G range. See [`set_scale()`].
//! - Read `WHO_AM_I` register. See [`who_am_i()`].
//! - Perform a software reset. See [`reset()`].
//! - Run a communication self-test. See [`self_test()`].
//!
//! [`enable()`]: struct.Kxcj9.html#method.enable
//! [`read()`]: struct.Kxcj9.html#method.read
//! [`read_unscaled()`]: struct.Kxcj9.html#method.read_unscaled
//! [`set_resolution()`]: struct.Kxcj9.html#method.set_resolution
//! [`set_output_data_rate()`]: struct.Kxcj9.html#method.set_output_data_rate
//! [`set_scale()`]: struct.Kxcj9.html#method.set_scale
//! [`who_am_i()`]: struct.Kxcj9.html#method.who_am_i
//! [`reset()`]: struct.Kxcj9.html#method.reset
//! [`self_test()`]: struct.Kxcj9.html#method.self_test
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
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read acceleration
//!
//! ```no_run
//! extern crate kxcj9;
//! extern crate linux_embedded_hal as hal;
//! use kxcj9::{Kxcj9, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address =  SlaveAddr::default();
//! let mut sensor = Kxcj9::new_1018(dev, address);
//! sensor.enable().unwrap();
//! let acc = sensor.read().unwrap();
//! println!("X: {:2}, Y: {:2}, Z: {:2}", acc.x, acc.y, acc.z);
//! # }
//! ```
//!
//! ### Select high resolution
//!
//! ```no_run
//! extern crate kxcj9;
//! extern crate linux_embedded_hal as hal;
//! use kxcj9::{Kxcj9, Resolution, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Kxcj9::new_1018(dev, SlaveAddr::default());
//! sensor.enable().unwrap();
//! sensor.set_resolution(Resolution::High).unwrap();
//! // with this settings measurements are taken with 12-bit resolution
//! # }
//! ```
//!
//! ### Select +/-16g scale
//!
//! ```no_run
//! extern crate kxcj9;
//! extern crate linux_embedded_hal as hal;
//! use kxcj9::{GScale16, Kxcj9, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Kxcj9::new_1018(dev, SlaveAddr::default());
//! sensor.enable().unwrap();
//! sensor.set_scale(GScale16::G16FP).unwrap();
//! // with this settings measurements are taken with 14-bit resolution
//! # }
//! ```
//!
//! ### Select 200Hz output data rate
//!
//! ```no_run
//! extern crate kxcj9;
//! extern crate linux_embedded_hal as hal;
//! use kxcj9::{Kxcj9, OutputDataRate, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Kxcj9::new_1018(dev, SlaveAddr::default());
//! sensor.enable().unwrap();
//! sensor.set_output_data_rate(OutputDataRate::Hz200).unwrap();
//! # }
//! ```
//!
//! ### Perform a software reset and wait for it to finish
//!
//! ```no_run
//! extern crate kxcj9;
//! extern crate linux_embedded_hal as hal;
//! #[macro_use(block)]
//! extern crate nb;
//! use kxcj9::{Kxcj9, OutputDataRate, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Kxcj9::new_1018(dev, SlaveAddr::default());
//! block!(sensor.reset());
//! # }
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
extern crate nb;
use core::marker::PhantomData;
use hal::blocking::i2c;

mod types;
pub use types::{
    Error, GScale16, GScale8, Measurement, OutputDataRate, Resolution, SlaveAddr,
    UnscaledMeasurement,
};

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
    fn is_high(self, mask: u8) -> bool {
        (self.bits & mask) != 0
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
    ctrl2: Config,
    data_ctrl: u8,
    was_reset_started: bool,
    _ic: PhantomData<IC>,
}

mod conversion;
mod scale_measurement;
pub use scale_measurement::ScaleMeasurement;
mod kxcj9;
pub use kxcj9::{GScaleConfig, MeasurementBits};

mod private {
    use super::{ic, kxcj9};
    pub trait Sealed {}

    impl Sealed for ic::Kxcj9_1008 {}
    impl Sealed for ic::Kxcj9_1018 {}
    impl Sealed for kxcj9::GScaleConfig {}
    impl Sealed for kxcj9::MeasurementBits {}
}
