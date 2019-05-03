use {
    conversion::{convert_12bit, convert_14bit, convert_8bit},
    i2c, ic, nb, Config, Error, GScale16, GScale8, InterruptInfo, Kxcj9, Measurement,
    OutputDataRate, PhantomData, Resolution, ScaledDevice, SlaveAddr, UnscaledMeasurement,
    WakeUpInterruptConfig, WakeUpTriggerMotion, DEVICE_BASE_ADDRESS,
};

struct Register;
impl Register {
    const XOUT_L: u8 = 0x06;
    const DCST_RESP: u8 = 0x0C;
    const WHO_AM_I: u8 = 0x0F;
    const INT_SOURCE1: u8 = 0x16;
    const STATUS: u8 = 0x18;
    const INT_REL: u8 = 0x1A;
    const CTRL1: u8 = 0x1B;
    const CTRL2: u8 = 0x1D;
    const INT_CTRL2: u8 = 0x1F;
    const DATA_CTRL: u8 = 0x21;
    const WAKEUP_TIMER: u8 = 0x29;
    const SELF_TEST: u8 = 0x3A;
    const WAKEUP_THRESHOLD: u8 = 0x6A;
}

struct BitFlags;
impl BitFlags {
    const PC1: u8 = 0b1000_0000;
    const RES: u8 = 0b0100_0000;
    const DRDYE: u8 = 0b0010_0000;
    const GSEL1: u8 = 0b0001_0000;
    const GSEL0: u8 = 0b0000_1000;
    const WUFE: u8 = 0b0000_0010;
    const SRST: u8 = 0b1000_0000;
    const DCST: u8 = 0b0001_0000;
    const INT: u8 = 0b0001_0000;
    const DRDY: u8 = 0b0001_0000;
    const WUFS: u8 = 0b0000_0010;
    const ZPWU: u8 = 0b0000_0001;
    const ZNWU: u8 = 0b0000_0010;
    const YPWU: u8 = 0b0000_0100;
    const YNWU: u8 = 0b0000_1000;
    const XPWU: u8 = 0b0001_0000;
    const XNWU: u8 = 0b0010_0000;
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
            ctrl2: Config::default(),
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
            ctrl2: Config::default(),
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
    IC: ScaledDevice,
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
    IC: ScaledDevice,
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
        self.read_register(Register::WHO_AM_I)
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

    /// Set output data rate.
    ///
    /// Setting a rate higher than or equal to 400Hz sets the resolution to high.
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
        let mut new_ctrl1 = self.ctrl1;
        self.prepare_ctrl1_to_change_settings()?;
        self.i2c
            .write(self.address, &[Register::DATA_CTRL, config])
            .map_err(Error::I2C)?;
        self.data_ctrl = config;
        let needs_full_power = odr == ODR::Hz400 || odr == ODR::Hz800 || odr == ODR::Hz1600;
        if needs_full_power {
            new_ctrl1 = new_ctrl1.with_high(BitFlags::RES);
        }
        if self.ctrl1 != new_ctrl1 {
            self.update_ctrl1(new_ctrl1)
        } else {
            Ok(())
        }
    }

    /// Enable new acceleration data ready interrupt.
    pub fn enable_data_ready_interrupt(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl1.with_high(BitFlags::DRDYE);
        self.prepare_ctrl1_to_change_settings()?;
        self.update_ctrl1(config)
    }

    /// Disable new acceleration data ready interrupt.
    pub fn disable_data_ready_interrupt(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl1.with_low(BitFlags::DRDYE);
        self.prepare_ctrl1_to_change_settings()?;
        self.update_ctrl1(config)
    }

    /// Configure and enable wake-up motion detected interrupt.
    pub fn enable_wake_up_interrupt(
        &mut self,
        config: WakeUpInterruptConfig,
    ) -> Result<(), Error<E>> {
        use WakeUpOutputDataRate as ODR;
        if config.fault_count == 0 {
            return Err(Error::InvalidSetting);
        }
        let threshold = IC::get_wake_up_threshold(config.threshold)?;

        let int_ctrl2 = config.trigger_motion.get_int_ctrl2();
        let ctrl2 = self.ctrl2.with_low(0b0000_0111);
        let ctrl2 = match config.data_rate {
            ODR::Hz0_781 => ctrl2,
            ODR::Hz1_563 => ctrl2.with_high(1),
            ODR::Hz3_125 => ctrl2.with_high(2),
            ODR::Hz6_25 => ctrl2.with_high(3),
            ODR::Hz12_5 => ctrl2.with_high(4),
            ODR::Hz25 => ctrl2.with_high(5),
            ODR::Hz50 => ctrl2.with_high(6),
            ODR::Hz100 => ctrl2.with_high(7),
        };
        let ctrl1 = self.ctrl1.with_high(BitFlags::WUFE);
        self.prepare_ctrl1_to_change_settings()?;
        self.write_register(Register::INT_CTRL2, int_ctrl2)?;
        self.write_register(Register::CTRL2, ctrl2.bits)?;
        self.ctrl2 = ctrl2;
        self.write_register(Register::WAKEUP_TIMER, config.fault_count)?;
        self.write_register(Register::WAKEUP_THRESHOLD, threshold)?;
        self.update_ctrl1(ctrl1)
    }

    /// Disable wake-up motion detected interrupt.
    pub fn disable_wake_up_interrupt(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl1.with_low(BitFlags::WUFE);
        self.prepare_ctrl1_to_change_settings()?;
        self.update_ctrl1(config)
    }

    /// Check if any interrupt has happened
    pub fn has_interrupt_happened(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS)?;
        Ok((status & BitFlags::INT) != 0)
    }

    /// Read interrupt source information
    pub fn read_interrupt_info(&mut self) -> Result<InterruptInfo, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[Register::INT_SOURCE1], &mut data)
            .map_err(Error::I2C)?;
        let info = InterruptInfo {
            data_ready: is_high(data[0], BitFlags::DRDY),
            wake_up: is_high(data[0], BitFlags::WUFS),
            wake_up_x_positive: is_high(data[1], BitFlags::XPWU),
            wake_up_x_negative: is_high(data[1], BitFlags::XNWU),
            wake_up_y_positive: is_high(data[1], BitFlags::YPWU),
            wake_up_y_negative: is_high(data[1], BitFlags::YNWU),
            wake_up_z_positive: is_high(data[1], BitFlags::ZPWU),
            wake_up_z_negative: is_high(data[1], BitFlags::ZNWU),
        };
        Ok(info)
    }

    /// Clear interrupts.
    ///
    /// This clears all interrupt source registers and changes the physical
    /// interrupt pin to its inactive state.
    pub fn clear_interrupts(&mut self) -> Result<(), Error<E>> {
        self.read_register(Register::INT_REL).and(Ok(()))
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
            self.ctrl2 = Config::default();
            self.data_ctrl = DATA_CTRL_DEFAULT;
            self.was_reset_started = true;
            Err(nb::Error::WouldBlock)
        }
    }

    fn has_reset_finished(&mut self) -> Result<bool, Error<E>> {
        let ctrl2 = self.read_register(Register::CTRL2)?;
        Ok((ctrl2 & BitFlags::SRST) == 0)
    }

    /// Perform a digital communication self-test
    pub fn communication_self_test(&mut self) -> Result<(), Error<E>> {
        let resp = self.read_register(Register::DCST_RESP)?;
        if resp != 0x55 {
            return Err(Error::SelfTestError);
        }
        let ctrl2 = self.ctrl2.with_high(BitFlags::DCST);
        self.write_register(Register::CTRL2, ctrl2.bits)?;
        let resp = self.read_register(Register::DCST_RESP)?;
        if resp != 0xAA {
            return Err(Error::SelfTestError);
        }
        let ctrl2 = self.read_register(Register::CTRL2)?;
        if (ctrl2 & BitFlags::DCST) != 0 {
            return Err(Error::SelfTestError);
        }
        let resp = self.read_register(Register::DCST_RESP)?;
        if resp != 0x55 {
            return Err(Error::SelfTestError);
        }
        Ok(())
    }

    /// Enable the MEMS self-test function
    pub fn enable_mems_self_test(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::SELF_TEST, 0xCA)
    }

    /// Disable the MEMS self-test function
    pub fn disable_mems_self_test(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::SELF_TEST, 0)
    }

    fn output_data_rate_greater_eq_400hz(&mut self) -> Result<bool, Error<E>> {
        let data_ctrl = self.read_register(Register::DATA_CTRL)?;
        Ok(data_ctrl >= 0b000_0101 && data_ctrl <= 0b000_0111)
    }

    /// Ensure PC1 in CTRL1 is set to 0 before changing settings
    fn prepare_ctrl1_to_change_settings(&mut self) -> Result<(), Error<E>> {
        self.disable()
    }
}

impl<I2C, E, IC> Kxcj9<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
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

    fn read_register(&mut self, reg_addr: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[reg_addr], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
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

fn is_high(value: u8, mask: u8) -> bool {
    (value & mask) != 0
}

impl WakeUpTriggerMotion {
    fn get_int_ctrl2(self) -> u8 {
        let mut int_ctrl2 = 0;
        if self.x_negative {
            int_ctrl2 |= BitFlags::XNWU;
        }
        if self.x_positive {
            int_ctrl2 |= BitFlags::XPWU;
        }
        if self.y_negative {
            int_ctrl2 |= BitFlags::YNWU;
        }
        if self.y_positive {
            int_ctrl2 |= BitFlags::YPWU;
        }
        if self.z_negative {
            int_ctrl2 |= BitFlags::ZNWU;
        }
        if self.z_positive {
            int_ctrl2 |= BitFlags::ZPWU;
        }
        int_ctrl2
    }
}
