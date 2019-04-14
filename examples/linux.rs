extern crate kxcj9;
extern crate linux_embedded_hal as hal;
use kxcj9::{Kxcj9, SlaveAddr};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let address =  SlaveAddr::default();
    let mut sensor = Kxcj9::new_1018(dev, address);
    let acc = sensor.read().unwrap();
    println!("X: {:2}, Y: {:2}, Z: {:2}", acc.x, acc.y, acc.z);
}
