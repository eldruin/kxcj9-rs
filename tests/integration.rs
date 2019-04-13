extern crate embedded_hal_mock as hal;
extern crate kxcj9;
use hal::i2c::Mock as I2cMock;
use kxcj9::{Kxcj9, SlaveAddr};

#[test]
fn can_create_and_destroy() {
    let sensor = Kxcj9::new(I2cMock::new(&[]), SlaveAddr::default());
    sensor.destroy().done();
}
