use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use kxcj9::{ic, Kxcj9, SlaveAddr};

pub const DEV_ADDR: u8 = 0xE;

pub struct Register;
#[allow(unused)]
impl Register {
    pub const XOUT_L: u8 = 0x06;
    pub const DCST_RESP: u8 = 0x0C;
    pub const WHO_AM_I: u8 = 0x0F;
    pub const INT_SOURCE1: u8 = 0x16;
    pub const STATUS: u8 = 0x18;
    pub const INT_REL: u8 = 0x1A;
    pub const CTRL1: u8 = 0x1B;
    pub const CTRL2: u8 = 0x1D;
    pub const INT_CTRL2: u8 = 0x1F;
    pub const DATA_CTRL: u8 = 0x21;
    pub const WAKEUP_TIMER: u8 = 0x29;
    pub const SELF_TEST: u8 = 0x3A;
}

pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    pub const PC1: u8 = 0b1000_0000;
    pub const RES: u8 = 0b0100_0000;
    pub const DRDYE: u8 = 0b0010_0000;
    pub const GSEL1: u8 = 0b0001_0000;
    pub const GSEL0: u8 = 0b0000_1000;
    pub const WUFE: u8 = 0b0000_0010;
    pub const SRST: u8 = 0b1000_0000;
    pub const DCST: u8 = 0b0001_0000;
    pub const INT: u8 = 0b0001_0000;
    pub const DRDY: u8 = 0b0001_0000;
    pub const WUFS: u8 = 0b0000_0010;
    pub const ZPWU: u8 = 0b0000_0001;
    pub const ZNWU: u8 = 0b0000_0010;
    pub const YPWU: u8 = 0b0000_0100;
    pub const YNWU: u8 = 0b0000_1000;
    pub const XPWU: u8 = 0b0001_0000;
    pub const XNWU: u8 = 0b0010_0000;
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
