/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid setting for the current configuration
    InvalidSetting,
    /// Error occured during self-test
    SelfTestError,
}

/// Measurement resolution
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resolution {
    /// 8-bit resolution.
    Low,
    /// 12-bit/14-bit resolution.
    High,
}

/// KXCJ9-1008 G scale (up to +/-8g)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GScale8 {
    /// Range: +/-2g
    G2,
    /// Range: +/-4g
    G4,
    /// Range: +/-8g
    G8,
    /// Range: +/-8g Full Power (selects 14-bit resolution)
    G8FP,
}

/// KXCJ9-1018 G scale (up to +/-16g)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GScale16 {
    /// Range: +/-4g
    G4,
    /// Range: +/-8g
    G8,
    /// Range: +/-16g
    G16,
    /// Range: +/-16g Full Power (selects 14-bit resolution)
    G16FP,
}

/// Output data rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputDataRate {
    /// 0.781 Hz
    Hz0_781,
    /// 1.563 Hz
    Hz1_563,
    /// 3.125 Hz
    Hz3_125,
    /// 6.25 Hz
    Hz6_25,
    /// 12.5 Hz
    Hz12_5,
    /// 25 Hz
    Hz25,
    /// 50 Hz (default)
    Hz50,
    /// 100 Hz
    Hz100,
    /// 200 Hz
    Hz200,
    /// 400 Hz (Forces device into full power mode)
    Hz400,
    /// 800 Hz (Forces device into full power mode)
    Hz800,
    /// 1600 Hz (Forces device into full power mode)
    Hz1600,
}

/// Acceleration measurement scaled to configured G range
#[derive(Debug, Default, Clone)]
pub struct Measurement {
    /// X-axis acceleration.
    pub x: f32,
    /// Y-axis acceleration.
    pub y: f32,
    /// Z-axis acceleration.
    pub z: f32,
}

/// Unscaled acceleration measurement
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnscaledMeasurement {
    /// X-axis acceleration.
    pub x: i16,
    /// Y-axis acceleration.
    pub y: i16,
    /// Z-axis acceleration.
    pub z: i16,
}

/// Interrupt source information
#[derive(Debug, Default, Clone, PartialEq)]
pub struct InterruptInfo {
    /// New acceleration data is available
    pub data_ready: bool,
    /// Wake-up motion detected
    pub wake_up: bool,
    /// Wake-up X-axis negative direction motion detected
    pub wake_up_x_negative: bool,
    /// Wake-up X-axis positive direction motion detected
    pub wake_up_x_positive: bool,
    /// Wake-up Y-axis negative direction motion detected
    pub wake_up_y_negative: bool,
    /// Wake-up Y-axis positive direction motion detected
    pub wake_up_y_positive: bool,
    /// Wake-up Z-axis negative direction motion detected
    pub wake_up_z_negative: bool,
    /// Wake-up Z-axis positive direction motion detected
    pub wake_up_z_positive: bool,
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
    pub(crate) fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a0) => default | a0 as u8,
        }
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
