//! This is a platform agnostic Rust driver for the KXCJ9 and KXCJB ultra-low-power
//! tri-axis accelerometers (up to +/-16g) using the [`embedded-hal`] traits.
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
//! - Run a communication self-test. See [`communication_self_test()`].
//! - Enable/disable MEMS self-test function. See [`enable_mems_self_test()`].
//! - Interrupt support:
//!     - Enable/disable new acceleration data ready interrupt. See [`enable_data_ready_interrupt()`].
//!     - Enable/disable and configure wake-up motion detected interrupt. See [`enable_wake_up_interrupt()`].
//!     - Enable/disable physical interrupt pin. See [`enable_interrupt_pin()`].
//!     - Set physical interrupt pin polarity. See [`set_interrupt_pin_polarity()`].
//!     - Set physical interrupt pin latching behavior. See [`set_interrupt_pin_latching()`].
//!     - Check if any interrupt has happened. See [`has_interrupt_happened()`].
//!     - Clear interrupts. See [`clear_interrupts()`].
//!     - Read interrupt source information. See [`read_interrupt_info()`].
//!
//! [`enable()`]: struct.Kxcj9.html#method.enable
//! [`read()`]: struct.Kxcj9.html#method.read
//! [`read_unscaled()`]: struct.Kxcj9.html#method.read_unscaled
//! [`set_resolution()`]: struct.Kxcj9.html#method.set_resolution
//! [`set_output_data_rate()`]: struct.Kxcj9.html#method.set_output_data_rate
//! [`set_scale()`]: struct.Kxcj9.html#method.set_scale
//! [`who_am_i()`]: struct.Kxcj9.html#method.who_am_i
//! [`reset()`]: struct.Kxcj9.html#method.reset
//! [`communication_self_test()`]: struct.Kxcj9.html#method.communication_self_test
//! [`enable_mems_self_test()`]: struct.Kxcj9.html#method.enable_mems_self_test
//! [`enable_data_ready_interrupt()`]: struct.Kxcj9.html#method.enable_data_ready_interrupt
//! [`enable_wake_up_interrupt()`]: struct.Kxcj9.html#method.enable_wake_up_interrupt
//! [`enable_interrupt_pin()`]: struct.Kxcj9.html#method.enable_interrupt_pin
//! [`set_interrupt_pin_polarity()`]: struct.Kxcj9.html#method.set_interrupt_pin_polarity
//! [`set_interrupt_pin_latching()`]: struct.Kxcj9.html#method.set_interrupt_pin_latching
//! [`has_interrupt_happened()`]: struct.Kxcj9.html#method.has_interrupt_happened
//! [`clear_interrupts()`]: struct.Kxcj9.html#method.clear_interrupts
//! [`read_interrupt_info()`]: struct.Kxcj9.html#method.read_interrupt_info
//!
//! ## The devices
//!
//! The KXCJ9 is a high-performance, ultra-low-power, tri-axis accelerometer
//! designed for mobile applications. It offers our best power performance
//! along with an embedded wake-up feature, Fast-mode I²C and up to 14-bit
//! resolution. The KXCJ9 sensor offers improved shock, reflow, and temperature
//! performance, and the ASIC has internal voltage regulators that allow
//! operation from 1.8 V to 3.6 V within the specified product performance.
//!
//! The KXCJB is the thinnest tri-axis accelerometer available on the market
//! today. This ultra-thin 3x3x0.45mm low-power accelerometer is also one of
//! our most full-featured products. The KXCJB offers up to 14-bit resolution
//! for greater precision. User-selectable parameters include ± 2g, 4g or 8g
//! ranges and Output Data Rates (ODR) with programmable low-pass filter.
//! The KXCJB also features the Kionix XAC sense element, our most advanced
//! sense element, for outstanding stability over temperature, shock and
//! post-reflow performance.
//!
//! The communication is done through an I2C bidirectional bus.
//!
//! Datasheet:
//! - [KXCJ9-1008](http://kionixfs.kionix.com/en/datasheet/KXCJ9-1008%20Specifications%20Rev%205.pdf)
//! - [KXCJ9-1018](http://kionixfs.kionix.com/en/datasheet/KXCJ9-1018%20Specifications%20Rev%202.pdf)
//! - [KXCJB-1041](http://kionixfs.kionix.com/en/datasheet/KXCJB-1041%20Specifications%20Rev%201.0.pdf)
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
//! ### Read acceleration in G
//!
//! ```no_run
//! extern crate kxcj9;
//! extern crate linux_embedded_hal as hal;
//! use kxcj9::{Kxcj9, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address =  SlaveAddr::default();
//! let mut sensor = Kxcj9::new_kxcj9_1018(dev, address);
//! sensor.enable().unwrap();
//! loop {
//!     let acc = sensor.read().unwrap();
//!     println!("X: {:2}, Y: {:2}, Z: {:2}", acc.x, acc.y, acc.z);
//! }
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
//! let mut sensor = Kxcj9::new_kxcj9_1018(dev, SlaveAddr::default());
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
//! let mut sensor = Kxcj9::new_kxcj9_1018(dev, SlaveAddr::default());
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
//! let mut sensor = Kxcj9::new_kxcj9_1018(dev, SlaveAddr::default());
//! sensor.enable().unwrap();
//! sensor.set_output_data_rate(OutputDataRate::Hz200).unwrap();
//! # }
//! ```
//!
//! ### Configure and enable wake-up interrupt
//!
//! ```no_run
//! extern crate kxcj9;
//! extern crate linux_embedded_hal as hal;
//! use kxcj9::{
//!     Kxcj9, SlaveAddr, WakeUpInterruptConfig, WakeUpOutputDataRate,
//!     WakeUpTriggerMotion,
//! };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Kxcj9::new_kxcj9_1008(dev, SlaveAddr::default());
//! let config = WakeUpInterruptConfig {
//!     trigger_motion: WakeUpTriggerMotion::default(),
//!     data_rate: WakeUpOutputDataRate::Hz3_125,
//!     fault_count: 3,
//!     threshold: 0.5, // G
//! };
//! // 0.5g acceleration must be present for 0.96s to trigger interrupt
//! sensor.enable_wake_up_interrupt(config).unwrap();
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
//! let mut sensor = Kxcj9::new_kxcj9_1018(dev, SlaveAddr::default());
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
    Error, GScale16, GScale8, InterruptInfo, InterruptPinLatching, InterruptPinPolarity,
    Measurement, OutputDataRate, Resolution, SlaveAddr, UnscaledMeasurement, WakeUpInterruptConfig,
    WakeUpOutputDataRate, WakeUpTriggerMotion,
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
    /// Used for KXCJ9-1008 and KXCJB-1041 devices
    pub struct G8Device(());
    /// Used for KXCJ9-1018 devices
    pub struct G16Device(());
}

/// KXCJ9/KXCJB device driver
#[derive(Debug)]
pub struct Kxcj9<I2C, IC> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    address: u8,
    ctrl1: Config,
    ctrl2: Config,
    int_ctrl1: Config,
    data_ctrl: u8,
    was_reset_started: bool,
    _ic: PhantomData<IC>,
}

mod conversion;
mod scaled_device;
pub use scaled_device::ScaledDevice;
mod device_impl;
pub use device_impl::{GScaleConfig, MeasurementBits};

mod private {
    use super::{device_impl, ic};
    pub trait Sealed {}

    impl Sealed for ic::G8Device {}
    impl Sealed for ic::G16Device {}
    impl Sealed for device_impl::GScaleConfig {}
    impl Sealed for device_impl::MeasurementBits {}
}
