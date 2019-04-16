use {
    conversion::{convert_12bit, convert_14bit, convert_8bit},
    i2c, ic, nb, Config, Error, GScale16, GScale8, Kxcj9, Measurement, OutputDataRate, PhantomData,
    Resolution, ScaleMeasurement, SlaveAddr, UnscaledMeasurement, DEVICE_BASE_ADDRESS,
};

struct Register;
impl Register {
    const XOUT_L: u8 = 0x06;
    const WHO_AM_I: u8 = 0x0F;
    const CTRL1: u8 = 0x1B;
    const CTRL2: u8 = 0x1D;
    const DATA_CTRL: u8 = 0x21;
}

struct BitFlags;
impl BitFlags {
    const PC1: u8 = 0b1000_0000;
    const RES: u8 = 0b0100_0000;
    const GSEL1: u8 = 0b0001_0000;
    const GSEL0: u8 = 0b0000_1000;
    const SRST: u8 = 0b1000_0000;
}

const DATA_CTRL_DEFAULT: u8 = 0x02;

#[doc(hidden)]
pub enum MeasurementBits {
    _8bit,
    _12bit,
    _14bit,
}

impl MeasurementBits {
    pub(crate) fn max(self) -> f32 {
        match self {
            MeasurementBits::_8bit => 128.0,
            MeasurementBits::_12bit => 2048.0,
            MeasurementBits::_14bit => 8192.0,
        }
    }
}

#[doc(hidden)]
pub enum GScaleConfig {
    _0,
    _1,
    _2,
    _3,
}

impl GScaleConfig {
    fn from_ctrl1(ctrl1: Config) -> Self {
        match ctrl1.bits & (BitFlags::GSEL0 | BitFlags::GSEL1) {
            0 => GScaleConfig::_0,
            BitFlags::GSEL0 => GScaleConfig::_1,
            BitFlags::GSEL1 => GScaleConfig::_2,
            _ => GScaleConfig::_3,
        }
    }
}

impl<I2C, E> Kxcj9<I2C, ic::Kxcj9_1008>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create new instance of the KXCJ9-1008 device.
    pub fn new_1008(i2c: I2C, address: SlaveAddr) -> Self {
        Kxcj9 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            ctrl1: Config::default(),
            data_ctrl: DATA_CTRL_DEFAULT,
            was_reset_started: false,
            _ic: PhantomData,
        }
    }
}

impl<I2C, E> Kxcj9<I2C, ic::Kxcj9_1018>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create new instance of the KXCJ9-1018 device.
    pub fn new_1018(i2c: I2C, address: SlaveAddr) -> Self {
        Kxcj9 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            ctrl1: Config::default(),
            data_ctrl: DATA_CTRL_DEFAULT,
            was_reset_started: false,
            _ic: PhantomData,
        }
    }
}

impl<I2C, E, IC> Kxcj9<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Enable the device (starts taking measurements).
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl1.with_high(BitFlags::PC1);
        self.update_ctrl1(config)
    }

    /// Disable the device.
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl1.with_low(BitFlags::PC1);
        self.update_ctrl1(config)
    }
}

impl<I2C, E, IC> Kxcj9<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
    IC: ScaleMeasurement,
{
    /// Read acceleration sensor data scaled to configured G range
    pub fn read(&mut self) -> Result<Measurement, Error<E>> {
        let unscaled = self.read_unscaled()?;
        Ok(IC::get_scaled(
            unscaled,
            self.get_measurement_bits(),
            GScaleConfig::from_ctrl1(self.ctrl1),
        ))
    }
}

impl<I2C, E, IC> Kxcj9<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Read unscaled acceleration sensor data
    pub fn read_unscaled(&mut self) -> Result<UnscaledMeasurement, Error<E>> {
        let mut data = [0; 6];
        self.i2c
            .write_read(self.address, &[Register::XOUT_L], &mut data)
            .map_err(Error::I2C)?;
        let m = match self.get_measurement_bits() {
            MeasurementBits::_8bit => convert_8bit(data[0], data[2], data[4]),
            MeasurementBits::_12bit => convert_12bit(
                u16::from(data[0]) | u16::from(data[1]) << 8,
                u16::from(data[2]) | u16::from(data[3]) << 8,
                u16::from(data[4]) | u16::from(data[5]) << 8,
            ),
            MeasurementBits::_14bit => convert_14bit(
                u16::from(data[0]) | u16::from(data[1]) << 8,
                u16::from(data[2]) | u16::from(data[3]) << 8,
                u16::from(data[4]) | u16::from(data[5]) << 8,
            ),
        };
        Ok(m)
    }

    fn get_measurement_bits(&self) -> MeasurementBits {
        let is_low_res = !self.ctrl1.is_high(BitFlags::RES);
        if is_low_res {
            MeasurementBits::_8bit
        } else {
            let on_full_power =
                self.ctrl1.is_high(BitFlags::GSEL0) && self.ctrl1.is_high(BitFlags::GSEL1);
            if on_full_power {
                MeasurementBits::_14bit
            } else {
                MeasurementBits::_12bit
            }
        }
    }

    /// Read the `WHO_AM_I` register. This should return `0xF`.
    pub fn who_am_i(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[Register::WHO_AM_I], &mut data)
            .map_err(Error::I2C)?;
        Ok(data[0])
    }

    /// Set resolution.
    ///
    /// Returns `Err(Error::InvalidSetting)` if setting `Resolution::Low` but the
    /// configured output data rate is greater or equal to 400 Hz.
    pub fn set_resolution(&mut self, resolution: Resolution) -> Result<(), Error<E>> {
        let config;
        match resolution {
            Resolution::Low => {
                if self.output_data_rate_greater_eq_400hz()? {
                    return Err(Error::InvalidSetting);
                } else {
                    config = self.ctrl1.with_low(BitFlags::RES);
                }
            }
            Resolution::High => config = self.ctrl1.with_high(BitFlags::RES),
        }
        self.prepare_ctrl1_to_change_settings()?;
        self.update_ctrl1(config)
    }

    /// Set output data rate
    pub fn set_output_data_rate(&mut self, odr: OutputDataRate) -> Result<(), Error<E>> {
        use OutputDataRate as ODR;
        let config = match odr {
            ODR::Hz0_781 => 0b000_1000,
            ODR::Hz1_563 => 0b000_1001,
            ODR::Hz3_125 => 0b000_1010,
            ODR::Hz6_25 => 0b000_1011,
            ODR::Hz12_5 => 0,
            ODR::Hz25 => 0b000_0001,
            ODR::Hz50 => 0b000_0010,
            ODR::Hz100 => 0b000_0011,
            ODR::Hz200 => 0b000_0100,
            ODR::Hz400 => 0b000_0101,
            ODR::Hz800 => 0b000_0110,
            ODR::Hz1600 => 0b000_0111,
        };
        let previous_ctrl1 = self.ctrl1;
        self.prepare_ctrl1_to_change_settings()?;
        self.i2c
            .write(self.address, &[Register::DATA_CTRL, config])
            .map_err(Error::I2C)?;
        self.data_ctrl = config;
        if self.ctrl1 != previous_ctrl1 {
            self.update_ctrl1(previous_ctrl1)
        } else {
            Ok(())
        }
    }

    /// Perform software reset
    ///
    /// This method offers a non-blocking interface. While the reset is in
    /// progress and when the reset was first triggered this will
    /// return `nb::Error::WouldBlock`.
    pub fn reset(&mut self) -> nb::Result<(), Error<E>> {
        if !self.has_reset_finished().map_err(nb::Error::Other)? {
            Err(nb::Error::WouldBlock)
        } else if self.was_reset_started {
            self.was_reset_started = false;
            Ok(())
        } else {
            self.i2c
                .write(self.address, &[Register::CTRL2, BitFlags::SRST])
                .map_err(Error::I2C)
                .map_err(nb::Error::Other)?;
            self.ctrl1 = Config::default();
            self.data_ctrl = DATA_CTRL_DEFAULT;
            self.was_reset_started = true;
            Err(nb::Error::WouldBlock)
        }
    }

    fn has_reset_finished(&mut self) -> Result<bool, Error<E>> {
        let mut ctrl2 = [0];
        self.i2c
            .write_read(self.address, &[Register::CTRL2], &mut ctrl2)
            .map_err(Error::I2C)?;
        Ok((ctrl2[0] & BitFlags::SRST) == 0)
    }

    fn output_data_rate_greater_eq_400hz(&mut self) -> Result<bool, Error<E>> {
        let mut data_ctrl = [0];
        self.i2c
            .write_read(self.address, &[Register::DATA_CTRL], &mut data_ctrl)
            .map_err(Error::I2C)?;
        Ok(data_ctrl[0] >= 0b000_0101 && data_ctrl[0] <= 0b000_0111)
    }

    /// Ensure PC1 in CTRL1 is set to 0 before changing settings
    fn prepare_ctrl1_to_change_settings(&mut self) -> Result<(), Error<E>> {
        self.disable()
    }

    fn update_ctrl1(&mut self, value: Config) -> Result<(), Error<E>> {
        self.write_register(Register::CTRL1, value.bits)?;
        self.ctrl1 = value;
        Ok(())
    }

    fn write_register(&mut self, reg_addr: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[reg_addr, value])
            .map_err(Error::I2C)
    }
}

impl<I2C, E> Kxcj9<I2C, ic::Kxcj9_1018>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Set G scale.
    pub fn set_scale(&mut self, scale: GScale16) -> Result<(), Error<E>> {
        use self::BitFlags as BF;
        let config = match scale {
            GScale16::G4 => self.ctrl1.with_low(BF::GSEL0).with_low(BF::GSEL1),
            GScale16::G8 => self.ctrl1.with_high(BF::GSEL0).with_low(BF::GSEL1),
            GScale16::G16 => self.ctrl1.with_low(BF::GSEL0).with_high(BF::GSEL1),
            GScale16::G16FP => self
                .ctrl1
                .with_high(BF::RES)
                .with_high(BF::GSEL0)
                .with_high(BF::GSEL1),
        };
        self.prepare_ctrl1_to_change_settings()?;
        self.update_ctrl1(config)
    }
}

impl<I2C, E> Kxcj9<I2C, ic::Kxcj9_1008>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Set G scale.
    pub fn set_scale(&mut self, scale: GScale8) -> Result<(), Error<E>> {
        use self::BitFlags as BF;
        let config = match scale {
            GScale8::G2 => self.ctrl1.with_low(BF::GSEL0).with_low(BF::GSEL1),
            GScale8::G4 => self.ctrl1.with_high(BF::GSEL0).with_low(BF::GSEL1),
            GScale8::G8 => self.ctrl1.with_low(BF::GSEL0).with_high(BF::GSEL1),
            GScale8::G8FP => self
                .ctrl1
                .with_high(BF::RES)
                .with_high(BF::GSEL0)
                .with_high(BF::GSEL1),
        };
        self.prepare_ctrl1_to_change_settings()?;
        self.update_ctrl1(config)
    }
}
