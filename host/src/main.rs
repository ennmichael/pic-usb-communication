mod dev;
mod util;

use dev::RawDevice;
use libusb::Context;

fn main() {
    let mut ctx = Context::new().unwrap();
    let mut device = RawDevice::acquire(&mut ctx).unwrap();
    println!("{:?}", device.read_message(5).unwrap());
    println!("{:?}", device.read_message(5).unwrap());
    device.write_message(&[0x09, 0x02, 0x03]).unwrap();
    println!("{:?}", device.read_message(5).unwrap());
    device
        .write_message(&[0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69])
        .unwrap();
    println!("{:?}", device.read_message(5).unwrap());
}
