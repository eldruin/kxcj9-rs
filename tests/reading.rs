extern crate embedded_hal_mock as hal;
extern crate kxcj9;
use hal::i2c::Transaction as I2cTrans;
use kxcj9::{GScale16, Resolution};

mod common;
use common::{destroy, new_1018, BitFlags, Register, DEV_ADDR};

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

fn assert_near_positive(a: f32, b: f32) {
    if (a - b).abs() > 0.1 {
        panic!(format!("{} vs {}", a, b));
    }
}

#[test]
#[should_panic]
fn assert_near_can_fail() {
    assert_near_positive(1.0, 2.0)
}

#[test]
fn can_read_8bit_4g_1018() {
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::XOUT_L],
        vec![0, 0xAB, 64, 0xCD, 127, 0xEF],
    )];
    let mut sensor = new_1018(&transactions);
    let measurement = sensor.read().unwrap();
    assert_near_positive(0.0, measurement.x);
    assert_near_positive(2.0, measurement.y);
    assert_near_positive(4.0, measurement.z);
    destroy(sensor);
}

#[test]
fn can_read_12bit_8g_1018() {
    use BitFlags as BF;
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BF::RES]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BF::RES]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BF::RES | BF::GSEL0]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::XOUT_L], vec![1, 0, 255, 3, 255, 7]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.set_resolution(Resolution::High).unwrap();
    sensor.set_scale(GScale16::G8).unwrap();
    let measurement = sensor.read().unwrap();
    assert_near_positive(0.0, measurement.x);
    assert_near_positive(4.0, measurement.y);
    assert_near_positive(8.0, measurement.z);
    destroy(sensor);
}

#[test]
fn can_read_12bit_16g_1018() {
    use BitFlags as BF;
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BF::RES]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BF::RES]),
        I2cTrans::write(DEV_ADDR, vec![Register::CTRL1, BF::RES | BF::GSEL1]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::XOUT_L], vec![1, 0, 255, 3, 255, 7]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.set_resolution(Resolution::High).unwrap();
    sensor.set_scale(GScale16::G16).unwrap();
    let measurement = sensor.read().unwrap();
    assert_near_positive(0.0, measurement.x);
    assert_near_positive(8.0, measurement.y);
    assert_near_positive(16.0, measurement.z);
    destroy(sensor);
}

#[test]
fn can_read_14bit_16g_1018() {
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
    let measurement = sensor.read().unwrap();
    assert_near_positive(0.0, measurement.x);
    assert_near_positive(8.0, measurement.y);
    assert_near_positive(16.0, measurement.z);
    destroy(sensor);
}
