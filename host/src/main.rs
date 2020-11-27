mod dev;

use dev::Device;
use libusb::Context;

fn main() {
    let mut ctx = Context::new().unwrap();
    let mut device = Device::acquire(&mut ctx).unwrap();
    println!("{}", device.write(&[0x90, 0x01, 0x00, 0x08, 0x12]).unwrap());
}
