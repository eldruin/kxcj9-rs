extern crate embedded_hal_mock as hal;
extern crate kxcj9;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use kxcj9::{Kxcj9, SlaveAddr};

const DEV_ADDR: u8 = 0xE;

struct Register;
impl Register {
    const WHO_AM_I: u8 = 0x0F;
    const CTRL1: u8 = 0x1B;
}

struct BitFlags;
impl BitFlags {
    const PC1: u8 = 0b1000_0000;
}

pub fn new(transactions: &[I2cTrans]) -> Kxcj9<I2cMock> {
    Kxcj9::new(I2cMock::new(&transactions), SlaveAddr::default())
}

pub fn destroy(sensor: Kxcj9<I2cMock>) {
    sensor.destroy().done();
}

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}

#[test]
fn can_read_who_am_i() {
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::WHO_AM_I],
        vec![0x0F],
    )];
    let mut sensor = new(&transactions);
    assert_eq!(0x0F, sensor.who_am_i().unwrap());
    destroy(sensor);
}

#[test]
fn can_enable() {
    let transactions = [I2cTrans::write(
        DEV_ADDR,
        vec![Register::CTRL1, BitFlags::PC1],
    )];
    let mut sensor = new(&transactions);
    sensor.enable().unwrap();
    destroy(sensor);
}

#[test]
fn can_disable() {
    let transactions = [I2cTrans::write(
        DEV_ADDR,
        vec![Register::CTRL1, 0],
    )];
    let mut sensor = new(&transactions);
    sensor.disable().unwrap();
    destroy(sensor);
}
