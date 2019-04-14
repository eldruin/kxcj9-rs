extern crate embedded_hal_mock as hal;
extern crate kxcj9;
use hal::i2c::{Transaction as I2cTrans};
use kxcj9::{GScale16, GScale8, OutputDataRate, Resolution};

mod common;
use common::{new_1008, new_1018, destroy, DEV_ADDR, Register, BitFlags};

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

macro_rules! set_gscale_test {
    ($name:ident, $create:ident, $scale:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, $expected]),
            ];
            let mut sensor = $create(&transactions);
            sensor.set_scale($scale).unwrap();
            destroy(sensor);
        }
    };
}

set_gscale_test!(set_gscale16_4g, new_1018, GScale16::G4, 0);
set_gscale_test!(set_gscale16_8g, new_1018, GScale16::G8, BitFlags::GSEL0);
set_gscale_test!(set_gscale16_16g, new_1018, GScale16::G16, BitFlags::GSEL1);
set_gscale_test!(
    set_gscale16_16g_fp,
    new_1018,
    GScale16::G16FP,
    BitFlags::GSEL0 | BitFlags::GSEL1 | BitFlags::RES
);

set_gscale_test!(set_gscale8_2g, new_1008, GScale8::G2, 0);
set_gscale_test!(set_gscale8_4g, new_1008, GScale8::G4, BitFlags::GSEL0);
set_gscale_test!(set_gscale8_8g, new_1008, GScale8::G8, BitFlags::GSEL1);
set_gscale_test!(
    set_gscale8_8g_fp,
    new_1008,
    GScale8::G8FP,
    BitFlags::GSEL0 | BitFlags::GSEL1 | BitFlags::RES
);

#[test]
fn can_read_unscaled_8bit() {
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::XOUT_L],
        vec![0, 0xAB, 127, 0xCD, 128, 0xEF],
    )];
    let mut sensor = new_1018(&transactions);
    let measurement = sensor.read_unscaled().unwrap();
    assert_eq!(0, measurement.x);
    assert_eq!(127, measurement.y);
    assert_eq!(-128, measurement.z);
    destroy(sensor);
}

#[test]
fn can_read_unscaled_12bit() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BitFlags::RES]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::XOUT_L], vec![1, 0, 255, 3, 255, 7]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.set_resolution(Resolution::High).unwrap();
    let measurement = sensor.read_unscaled().unwrap();
    assert_eq!(1, measurement.x);
    assert_eq!(1023, measurement.y);
    assert_eq!(2047, measurement.z);
    destroy(sensor);
}

#[test]
fn can_read_unscaled_14bit() {
    use BitFlags as BF;
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::CTRL1, BF::RES | BF::GSEL0 | BF::GSEL1],
        ),
        I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::XOUT_L],
            vec![1, 0, 255, 15, 255, 31],
        ),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.set_scale(GScale16::G16FP).unwrap();
    let measurement = sensor.read_unscaled().unwrap();
    assert_eq!(1, measurement.x);
    assert_eq!(4095, measurement.y);
    assert_eq!(8191, measurement.z);
    destroy(sensor);
}
