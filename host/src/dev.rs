use libusb::{self, Context, DeviceHandle, Result};
use std::io::Read;
use std::io::Write;
use std::time::Duration;

const VENDOR_ID: u16 = 1240;
const PRODUCT_ID: u16 = 221;
const HID_INTERFACE: u8 = 2;
const HID_INPUT_ENDPOINT: u8 = 0x83;
const HID_OUTPUT_ENDPOINT: u8 = 0x03;
const DEVICE_USB_ADDRESS: u8 = 0x08;

pub struct Device<'a> {
    handle: DeviceHandle<'a>,
}

impl<'a> Device<'a> {
    pub fn acquire(ctx: &'a mut Context) -> Option<Self> {
        ctx.open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID)
            .map(|mut handle| {
                claim_hid_interface(&mut handle).unwrap();
                Device { handle }
            })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.handle
            .write_interrupt(HID_OUTPUT_ENDPOINT, data, Duration::from_secs(5))
    }

    pub fn read(&mut self) -> Vec<u8> {
        Vec::new()
    }
}

fn claim_hid_interface(handle: &mut DeviceHandle) -> Result<()> {
    handle.detach_kernel_driver(HID_INTERFACE)?;
    handle.claim_interface(HID_INTERFACE)?;
    Ok(())
}
