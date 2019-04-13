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

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit value for A0
    Alternative(bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a0) => default | a0 as u8,
        }
    }
}

const DEVICE_BASE_ADDRESS: u8 = 0xE;

struct Register;
impl Register {
    const WHO_AM_I: u8 = 0x0F;
}

/// KXCJ9 device driver.
#[derive(Debug)]
pub struct Kxcj9<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Kxcj9<I2C>
where
    I2C: hal::blocking::i2c::WriteRead<Error = E>,
{
    /// Create new instance of the KXCJ9 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Kxcj9 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Read the `WHO_AM_I` register. This should return `0xF`.
    pub fn who_am_i(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[Register::WHO_AM_I], &mut data)
            .map_err(Error::I2C)?;
        Ok(data[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DEVICE_BASE_ADDRESS as BASE_ADDR;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(BASE_ADDR, addr.addr(BASE_ADDR));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b000_1110, SlaveAddr::Alternative(false).addr(BASE_ADDR));
        assert_eq!(0b000_1111, SlaveAddr::Alternative(true).addr(BASE_ADDR));
    }
}
