mod dev;
mod util;

use clap::{App, Arg, SubCommand};
use dev::Device;
use libusb::Context;
use std::fs;

fn main() {
    let mut ctx = Context::new().unwrap();
    let matches = App::new("host")
        .version("0.1")
        .subcommand(
            SubCommand::with_name("load")
                .about("Loads data from the device")
                .arg(Arg::with_name("FILE").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("store")
                .about("Stores data in the device")
                .arg(Arg::with_name("FILE").required(true).index(1)),
        )
        .get_matches();

    let mut device = Device::acquire(&mut ctx)
        .expect("Error while acquiring the device, maybe you need to run as sudo");

    if let Some(matches) = matches.subcommand_matches("store") {
        let file = matches.value_of("FILE").unwrap();
        let content = fs::read(file).unwrap();
        println!("Storing data: {:?}", content);
        device
            .store(&content)
            .expect("Error while storing data in the device");
    } else if let Some(matches) = matches.subcommand_matches("load") {
        let file = matches.value_of("FILE").unwrap();
        let data = device
            .load()
            .expect("Error while loading data from the device");
        println!("Loaded data: {:?}", data);
        fs::write(file, &data).unwrap();
    }
}
