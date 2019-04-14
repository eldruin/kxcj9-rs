extern crate embedded_hal_mock as hal;
extern crate kxcj9;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use kxcj9::{ic, Kxcj9, OutputDataRate, Resolution, SlaveAddr};

const DEV_ADDR: u8 = 0xE;

struct Register;
impl Register {
    const WHO_AM_I: u8 = 0x0F;
    const CTRL1: u8 = 0x1B;
    const DATA_CTRL: u8 = 0x21;
}

struct BitFlags;
impl BitFlags {
    const PC1: u8 = 0b1000_0000;
    const RES: u8 = 0b0100_0000;
}

pub fn new_1018(transactions: &[I2cTrans]) -> Kxcj9<I2cMock, ic::Kxcj9_1018> {
    Kxcj9::new_1018(I2cMock::new(&transactions), SlaveAddr::default())
}

pub fn destroy<IC>(sensor: Kxcj9<I2cMock, IC>) {
    sensor.destroy().done();
}

#[test]
fn can_create_and_destroy() {
    let sensor = new_1018(&[]);
    destroy(sensor);
}

#[test]
fn can_read_who_am_i() {
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::WHO_AM_I],
        vec![0x0F],
    )];
    let mut sensor = new_1018(&transactions);
    assert_eq!(0x0F, sensor.who_am_i().unwrap());
    destroy(sensor);
}

#[test]
fn can_enable() {
    let transactions = [I2cTrans::write(
        DEV_ADDR,
        vec![Register::CTRL1, BitFlags::PC1],
    )];
    let mut sensor = new_1018(&transactions);
    sensor.enable().unwrap();
    destroy(sensor);
}

#[test]
fn can_disable() {
    let transactions = [I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0])];
    let mut sensor = new_1018(&transactions);
    sensor.disable().unwrap();
    destroy(sensor);
}

macro_rules! cannot_set_res_low_for_odr {
    ($name:ident, $variant:ident, $value:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Register::DATA_CTRL, $value]),
                I2cTrans::write_read(DEV_ADDR, vec![Register::DATA_CTRL], vec![$value]),
            ];
            let mut sensor = new_1018(&transactions);
            sensor
                .set_output_data_rate(OutputDataRate::$variant)
                .unwrap();
            sensor
                .set_resolution(Resolution::Low)
                .expect_err("Should have returned error");
            destroy(sensor);
        }
    };
}
cannot_set_res_low_for_odr!(cannot_set_res_low_odr_400, Hz400, 5);
cannot_set_res_low_for_odr!(cannot_set_res_low_odr_800, Hz800, 6);
cannot_set_res_low_for_odr!(cannot_set_res_low_odr_1600, Hz1600, 7);

#[test]
fn can_set_resolution_low() {
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::DATA_CTRL], vec![4]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.set_resolution(Resolution::Low).unwrap();
    destroy(sensor);
}

#[test]
fn can_set_resolution_high() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BitFlags::RES]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.set_resolution(Resolution::High).unwrap();
    destroy(sensor);
}

#[test]
fn set_resolution_keeps_enable_status() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BitFlags::PC1]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::CTRL1, BitFlags::PC1 | BitFlags::RES],
        ),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.enable().unwrap();
    sensor.set_resolution(Resolution::High).unwrap();
    destroy(sensor);
}

#[test]
fn set_odr_keeps_enable_status() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BitFlags::PC1]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Register::DATA_CTRL, 2]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BitFlags::PC1]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.enable().unwrap();
    sensor.set_output_data_rate(OutputDataRate::Hz50).unwrap();
    destroy(sensor);
}

macro_rules! set_odr_test {
    ($name:ident, $variant:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Register::DATA_CTRL, $expected]),
            ];
            let mut sensor = new_1018(&transactions);
            sensor
                .set_output_data_rate(OutputDataRate::$variant)
                .unwrap();
            destroy(sensor);
        }
    };
}

set_odr_test!(set_odr_hz0, Hz0_781, 8);
set_odr_test!(set_odr_hz1, Hz1_563, 9);
set_odr_test!(set_odr_hz3, Hz3_125, 10);
set_odr_test!(set_odr_hz6, Hz6_25, 11);
set_odr_test!(set_odr_hz12, Hz12_5, 0);
set_odr_test!(set_odr_hz25, Hz25, 1);
set_odr_test!(set_odr_hz50, Hz50, 2);
set_odr_test!(set_odr_hz100, Hz100, 3);
set_odr_test!(set_odr_hz200, Hz200, 4);
set_odr_test!(set_odr_hz400, Hz400, 5);
set_odr_test!(set_odr_hz800, Hz800, 6);
set_odr_test!(set_odr_hz1600, Hz1600, 7);
