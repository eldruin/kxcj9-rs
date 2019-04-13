extern crate embedded_hal_mock as hal;
extern crate kxcj9;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use kxcj9::{Kxcj9, SlaveAddr};

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
