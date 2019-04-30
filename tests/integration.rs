extern crate embedded_hal_mock as hal;
extern crate kxcj9;
use hal::i2c::Transaction as I2cTrans;
use kxcj9::{GScale16, GScale8, InterruptInfo, OutputDataRate, Resolution};

mod common;
use common::{destroy, new_1008, new_1018, BitFlags as BF, Register as Reg, DEV_ADDR};

#[test]
fn can_create_and_destroy() {
    let sensor = new_1018(&[]);
    destroy(sensor);
}

#[test]
fn can_read_who_am_i() {
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Reg::WHO_AM_I],
        vec![0x0F],
    )];
    let mut sensor = new_1018(&transactions);
    assert_eq!(0x0F, sensor.who_am_i().unwrap());
    destroy(sensor);
}

#[test]
fn can_enable() {
    let transactions = [I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1])];
    let mut sensor = new_1018(&transactions);
    sensor.enable().unwrap();
    destroy(sensor);
}

#[test]
fn can_disable() {
    let transactions = [I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0])];
    let mut sensor = new_1018(&transactions);
    sensor.disable().unwrap();
    destroy(sensor);
}

macro_rules! cannot_set_res_low_for_odr {
    ($name:ident, $variant:ident, $value:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Reg::DATA_CTRL, $value]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::RES]),
                I2cTrans::write_read(DEV_ADDR, vec![Reg::DATA_CTRL], vec![$value]),
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
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DATA_CTRL], vec![4]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.set_resolution(Resolution::Low).unwrap();
    destroy(sensor);
}

#[test]
fn can_set_resolution_high() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::RES]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.set_resolution(Resolution::High).unwrap();
    destroy(sensor);
}

#[test]
fn set_resolution_keeps_enable_status() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1 | BF::RES]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.enable().unwrap();
    sensor.set_resolution(Resolution::High).unwrap();
    destroy(sensor);
}

#[test]
fn set_odr_keeps_enable_status() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::DATA_CTRL, 2]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
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
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Reg::DATA_CTRL, $expected]),
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

macro_rules! set_high_odr_test {
    ($name:ident, $variant:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Reg::DATA_CTRL, $expected]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::RES]),
            ];
            let mut sensor = new_1018(&transactions);
            sensor
                .set_output_data_rate(OutputDataRate::$variant)
                .unwrap();
            destroy(sensor);
        }
    };
}
set_high_odr_test!(set_odr_hz400, Hz400, 5);
set_high_odr_test!(set_odr_hz800, Hz800, 6);
set_high_odr_test!(set_odr_hz1600, Hz1600, 7);

macro_rules! set_gscale_test {
    ($name:ident, $create:ident, $scale:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, $expected]),
            ];
            let mut sensor = $create(&transactions);
            sensor.set_scale($scale).unwrap();
            destroy(sensor);
        }
    };
}

set_gscale_test!(set_gscale16_4g, new_1018, GScale16::G4, 0);
set_gscale_test!(set_gscale16_8g, new_1018, GScale16::G8, BF::GSEL0);
set_gscale_test!(set_gscale16_16g, new_1018, GScale16::G16, BF::GSEL1);
set_gscale_test!(
    set_gscale16_16g_fp,
    new_1018,
    GScale16::G16FP,
    BF::GSEL0 | BF::GSEL1 | BF::RES
);

set_gscale_test!(set_gscale8_2g, new_1008, GScale8::G2, 0);
set_gscale_test!(set_gscale8_4g, new_1008, GScale8::G4, BF::GSEL0);
set_gscale_test!(set_gscale8_8g, new_1008, GScale8::G8, BF::GSEL1);
set_gscale_test!(
    set_gscale8_8g_fp,
    new_1008,
    GScale8::G8FP,
    BF::GSEL0 | BF::GSEL1 | BF::RES
);

#[test]
fn can_trigger_sw_reset_then_driver_configuration_is_reset() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::RES]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::CTRL2], vec![0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL2, BF::SRST]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::CTRL2], vec![BF::SRST]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::CTRL2], vec![0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.set_resolution(Resolution::High).unwrap();
    sensor.reset().expect_err("Should return WouldBlock error"); // trigger reset
    sensor.reset().expect_err("Should return WouldBlock error"); // reset still not finished
    sensor.reset().unwrap(); // reset finished
    sensor.enable().unwrap();
    destroy(sensor);
}

#[test]
fn can_perform_communication_self_test() {
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0x55]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL2, BF::DCST]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0xAA]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::CTRL2], vec![0]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0x55]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.communication_self_test().unwrap();
    destroy(sensor);
}

#[test]
fn communication_self_test_can_fail_in_step1() {
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Reg::DCST_RESP],
        vec![0x56],
    )];
    let mut sensor = new_1018(&transactions);
    sensor
        .communication_self_test()
        .expect_err("Should return error");
    destroy(sensor);
}

#[test]
fn communication_self_test_can_fail_in_step2() {
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0x55]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL2, BF::DCST]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0xAB]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor
        .communication_self_test()
        .expect_err("Should return error");
    destroy(sensor);
}

#[test]
fn communication_self_test_can_fail_in_step3() {
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0x55]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL2, BF::DCST]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0xAA]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::CTRL2], vec![BF::DCST]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor
        .communication_self_test()
        .expect_err("Should return error");
    destroy(sensor);
}

#[test]
fn communication_self_test_can_fail_in_step4() {
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0x55]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL2, BF::DCST]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0xAA]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::CTRL2], vec![0]),
        I2cTrans::write_read(DEV_ADDR, vec![Reg::DCST_RESP], vec![0x56]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor
        .communication_self_test()
        .expect_err("Should return error");
    destroy(sensor);
}

#[test]
fn can_enable_mems_self_test() {
    let transactions = [I2cTrans::write(DEV_ADDR, vec![Reg::SELF_TEST, 0xCA])];
    let mut sensor = new_1018(&transactions);
    sensor.enable_mems_self_test().unwrap();
    destroy(sensor);
}

#[test]
fn can_disable_mems_self_test() {
    let transactions = [I2cTrans::write(DEV_ADDR, vec![Reg::SELF_TEST, 0])];
    let mut sensor = new_1018(&transactions);
    sensor.disable_mems_self_test().unwrap();
    destroy(sensor);
}

#[test]
fn interrupt_has_happened() {
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Reg::STATUS],
        vec![BF::INT],
    )];
    let mut sensor = new_1018(&transactions);
    assert!(sensor.has_interrupt_happened().unwrap());
    destroy(sensor);
}

#[test]
fn interrupt_has_not_happened() {
    let transactions = [I2cTrans::write_read(DEV_ADDR, vec![Reg::STATUS], vec![0])];
    let mut sensor = new_1018(&transactions);
    assert!(!sensor.has_interrupt_happened().unwrap());
    destroy(sensor);
}

#[test]
fn can_clear_interrupts() {
    let transactions = [I2cTrans::write_read(DEV_ADDR, vec![Reg::INT_REL], vec![0])];
    let mut sensor = new_1018(&transactions);
    sensor.clear_interrupts().unwrap();
    destroy(sensor);
}

#[test]
fn no_interrupt_has_happened() {
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Reg::INT_SOURCE1],
        vec![0, 0],
    )];
    let mut sensor = new_1018(&transactions);
    assert_eq!(
        InterruptInfo::default(),
        sensor.read_interrupt_info().unwrap()
    );
    destroy(sensor);
}

macro_rules! int_happened_test {
    ($name:ident, $int_source1:expr, $int_source2:expr, $field:ident) => {
        #[test]
        fn $name() {
            let transactions = [I2cTrans::write_read(
                DEV_ADDR,
                vec![Reg::INT_SOURCE1],
                vec![$int_source1, $int_source2],
            )];
            let mut sensor = new_1018(&transactions);
            assert_eq!(
                InterruptInfo {
                    $field: true,
                    ..Default::default()
                },
                sensor.read_interrupt_info().unwrap()
            );
            destroy(sensor);
        }
    };
}
int_happened_test!(data_ready_int, BF::DRDY, 0, data_ready);
int_happened_test!(wake_up_int, BF::WUFS, 0, wake_up);
int_happened_test!(wake_up_x_pos_int, 0, BF::XPWU, wake_up_x_positive);
int_happened_test!(wake_up_x_neg_int, 0, BF::XNWU, wake_up_x_negative);
int_happened_test!(wake_up_y_pos_int, 0, BF::YPWU, wake_up_y_positive);
int_happened_test!(wake_up_y_neg_int, 0, BF::YNWU, wake_up_y_negative);
int_happened_test!(wake_up_z_pos_int, 0, BF::ZPWU, wake_up_z_positive);
int_happened_test!(wake_up_z_neg_int, 0, BF::ZNWU, wake_up_z_negative);

macro_rules! change_ctrl1_test {
    ($name:ident, $method:ident, $ctrl1_after:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, $ctrl1_after]),
            ];
            let mut sensor = new_1018(&transactions);
            sensor.$method().unwrap();
            destroy(sensor);
        }
    };
}

change_ctrl1_test!(can_enable_drdy_int, enable_data_ready_interrupt, BF::DRDYE);
change_ctrl1_test!(can_disable_drdy_int, disable_data_ready_interrupt, 0);
