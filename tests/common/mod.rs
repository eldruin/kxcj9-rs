use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use kxcj9::{ic, Kxcj9, SlaveAddr};

pub const DEV_ADDR: u8 = 0xE;

pub struct Register;
#[allow(unused)]
impl Register {
    pub const XOUT_L: u8 = 0x06;
    pub const WHO_AM_I: u8 = 0x0F;
    pub const CTRL1: u8 = 0x1B;
    pub const CTRL2: u8 = 0x1D;
    pub const DATA_CTRL: u8 = 0x21;
}

pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    pub const PC1: u8 = 0b1000_0000;
    pub const RES: u8 = 0b0100_0000;
    pub const GSEL1: u8 = 0b0001_0000;
    pub const GSEL0: u8 = 0b0000_1000;
    pub const SRST: u8 = 0b1000_0000;
}

#[allow(unused)]
pub fn new_1008(transactions: &[I2cTrans]) -> Kxcj9<I2cMock, ic::Kxcj9_1008> {
    Kxcj9::new_1008(I2cMock::new(&transactions), SlaveAddr::default())
}

#[allow(unused)]
pub fn new_1018(transactions: &[I2cTrans]) -> Kxcj9<I2cMock, ic::Kxcj9_1018> {
    Kxcj9::new_1018(I2cMock::new(&transactions), SlaveAddr::default())
}

pub fn destroy<IC>(sensor: Kxcj9<I2cMock, IC>) {
    sensor.destroy().done();
}
