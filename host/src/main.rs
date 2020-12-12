mod dev;
mod util;

use dev::Device;
use libusb::Context;

fn main() {
    let mut ctx = Context::new().unwrap();
    let mut device = Device::acquire(&mut ctx).unwrap();
    // device.store(&[0x04, 0x02, 0x03]).unwrap();
    println!("{:?}", device.load().unwrap());
}
