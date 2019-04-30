extern crate embedded_hal_mock as hal;
extern crate kxcj9;
use hal::i2c::Transaction as I2cTrans;
use kxcj9::{InterruptInfo, WakeUpOutputDataRate, WakeUpTriggerMotion};

mod common;
use common::{destroy, new_1018, BitFlags as BF, Register as Reg, DEV_ADDR};

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
change_ctrl1_test!(can_enable_wake_up_int, enable_wake_up_interrupt, BF::WUFE);
change_ctrl1_test!(can_disable_wake_up_int, disable_wake_up_interrupt, 0);

#[test]
fn config_wu_int_disable_all() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::INT_CTRL2, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL2, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.enable().unwrap();
    let trigger_motion = WakeUpTriggerMotion {
        x_negative: false,
        x_positive: false,
        y_negative: false,
        y_positive: false,
        z_negative: false,
        z_positive: false,
    };
    let odr = WakeUpOutputDataRate::default();
    sensor
        .configure_wake_up_interrupt(trigger_motion, odr)
        .unwrap();
    destroy(sensor);
}

#[test]
fn config_wu_int_enable_all() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::INT_CTRL2, 0b0011_1111]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL2, 0]),
        I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
    ];
    let mut sensor = new_1018(&transactions);
    sensor.enable().unwrap();
    let trigger_motion = WakeUpTriggerMotion::default();
    let odr = WakeUpOutputDataRate::default();
    sensor
        .configure_wake_up_interrupt(trigger_motion, odr)
        .unwrap();
    destroy(sensor);
}

macro_rules! config_wu_int_motion_test {
    ($name:ident, $direction:ident, $int_ctrl2:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Reg::INT_CTRL2, $int_ctrl2]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL2, 0]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
            ];
            let mut sensor = new_1018(&transactions);
            sensor.enable().unwrap();
            let mut trigger_motion = WakeUpTriggerMotion {
                x_negative: false,
                x_positive: false,
                y_negative: false,
                y_positive: false,
                z_negative: false,
                z_positive: false,
            };
            trigger_motion.$direction = true;
            let odr = WakeUpOutputDataRate::default();
            sensor
                .configure_wake_up_interrupt(trigger_motion, odr)
                .unwrap();
            destroy(sensor);
        }
    };
}

config_wu_int_motion_test!(can_enable_wu_int_motion_x_neg, x_negative, BF::XNWU);
config_wu_int_motion_test!(can_enable_wu_int_motion_x_pos, x_positive, BF::XPWU);
config_wu_int_motion_test!(can_enable_wu_int_motion_y_neg, y_negative, BF::YNWU);
config_wu_int_motion_test!(can_enable_wu_int_motion_y_pos, y_positive, BF::YPWU);
config_wu_int_motion_test!(can_enable_wu_int_motion_z_neg, z_negative, BF::ZNWU);
config_wu_int_motion_test!(can_enable_wu_int_motion_z_pos, z_positive, BF::ZPWU);

macro_rules! set_wu_odr_test {
    ($name:ident, $variant:ident, $ctrl2:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, 0]),
                I2cTrans::write(DEV_ADDR, vec![Reg::INT_CTRL2, 0b0011_1111]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL2, $ctrl2]),
                I2cTrans::write(DEV_ADDR, vec![Reg::CTRL1, BF::PC1]),
            ];
            let mut sensor = new_1018(&transactions);
            sensor.enable().unwrap();
            let trigger_motion = WakeUpTriggerMotion::default();
            let odr = WakeUpOutputDataRate::$variant;
            sensor
                .configure_wake_up_interrupt(trigger_motion, odr)
                .unwrap();
            destroy(sensor);
        }
    };
}

set_wu_odr_test!(set_odr_hz0, Hz0_781, 0);
set_wu_odr_test!(set_odr_hz1, Hz1_563, 1);
set_wu_odr_test!(set_odr_hz3, Hz3_125, 2);
set_wu_odr_test!(set_odr_hz6, Hz6_25, 3);
set_wu_odr_test!(set_odr_hz12, Hz12_5, 4);
set_wu_odr_test!(set_odr_hz25, Hz25, 5);
set_wu_odr_test!(set_odr_hz50, Hz50, 6);
set_wu_odr_test!(set_odr_hz100, Hz100, 7);
