use crate::util;
use libusb::{self, Context, DeviceHandle, Error as UsbError};
use std::fmt::{self, Display, Formatter};
use std::time::Duration;

const VENDOR_ID: u16 = 1240;
const PRODUCT_ID: u16 = 221;
const HID_INTERFACE: u8 = 2;
const HID_INPUT_ENDPOINT: u8 = 0x83;
const HID_OUTPUT_ENDPOINT: u8 = 0x03;
const DEVICE_USB_ADDRESS: u8 = 0x08;
const COMMUNICATION_TIMEOUT: Duration = Duration::from_secs(5);
const MESSAGE_BUFFER_SIZE: usize = 64;

#[derive(Debug)]
pub enum Error {
    UsbError(UsbError),
    DeviceFailure(&'static str),
    DeviceNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match *self {
            Self::UsbError(ref usb_error) => usb_error.fmt(f),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<UsbError> for Error {
    fn from(error: UsbError) -> Self {
        Self::UsbError(error)
    }
}

type Result<T> = ::std::result::Result<T, Error>;

pub struct Device<'a> {
    raw_device: RawDevice<'a>,
}

impl<'a> Device<'a> {
    pub fn acquire(ctx: &'a mut Context) -> Result<Device> {
        Ok(Self {
            raw_device: RawDevice::acquire(ctx)?,
        })
    }

    pub fn store(&mut self, data: &[u8]) -> Result<()> {
        let mut data = Vec::from(data);
        data.insert(0, data.len() as u8);
        self.raw_device.write_message(&data)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<Vec<u8>> {
        self.raw_device.write_message(&[0x00])?;
        println!("Wrote 0");
        let len_message = self.raw_device.read_message(1)?;
        println!("Device says {}", len_message[0]);
        Ok(self.raw_device.read_message(len_message[0])?)
    }
}

pub struct RawDevice<'a> {
    handle: DeviceHandle<'a>,
}

impl<'a> RawDevice<'a> {
    pub fn acquire(ctx: &'a mut Context) -> Result<RawDevice> {
        match ctx.open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID) {
            Some(mut handle) => {
                claim_hid_interface(&mut handle)?;
                let mut device = RawDevice { handle };
                device.set_speed()?;
                Ok(device)
            }
            None => Err(Error::DeviceNotFound),
        }
    }

    fn set_speed(&mut self) -> Result<()> {
        self.write_interrupt(&[0x10, 0x00, 0x00, 0x20, 0xFF])?;
        let mut buf = [0; 64];
        self.read_interrupt(&mut buf)?;
        if buf[0] != 0x10 || buf[3] != 0x20 {
            Err(Error::DeviceFailure("Failed to set I2C speed"))
        } else {
            Ok(())
        }
    }

    pub fn write_message(&mut self, data: &[u8]) -> Result<()> {
        if data.len() > 20 {
            unimplemented!()
        } else {
            loop {
                let mut raw_data = [0; 64];
                raw_data[0] = 0x90;
                raw_data[1] = data.len() as u8;
                raw_data[3] = DEVICE_USB_ADDRESS;
                raw_data[4..4 + data.len()].copy_from_slice(data);
                self.write_interrupt(&raw_data)?;
                if self.check_response()? {
                    break;
                }
            }
            Ok(())
        }
    }

    fn write_interrupt(&mut self, raw_data: &[u8]) -> Result<()> {
        let n =
            self.handle
                .write_interrupt(HID_OUTPUT_ENDPOINT, raw_data, COMMUNICATION_TIMEOUT)?;
        if n != raw_data.len() {
            Err(Error::DeviceFailure(
                "Error while writing to device: wrote unexpected number of bytes",
            ))
        } else {
            Ok(())
        }
    }

    pub fn read_message(&mut self, len: u8) -> Result<Vec<u8>> {
        if len > 20 {
            unimplemented!();
        }

        loop {
            let mut msg = [0; MESSAGE_BUFFER_SIZE];
            msg[0] = 0x91;
            msg[1] = len;
            msg[3] = DEVICE_USB_ADDRESS + 1;
            self.write_interrupt(&msg)?;
            if self.check_response()? {
                break;
            }
        }

        let mut buf = [0; MESSAGE_BUFFER_SIZE];

        loop {
            let mut msg = [0; MESSAGE_BUFFER_SIZE];
            msg[0] = 0x40;
            self.write_interrupt(&msg)?;
            self.read_interrupt(&mut buf)?;
            if buf[1] == 0x00 {
                break;
            }
        }

        println!("Read {:?}", buf);
        Ok(Vec::from(&buf[4..4 + buf[3] as usize]))
    }

    fn read_interrupt(&mut self, buf: &mut [u8]) -> Result<()> {
        let n = self
            .handle
            .read_interrupt(HID_INPUT_ENDPOINT, buf, COMMUNICATION_TIMEOUT)?;
        if n != MESSAGE_BUFFER_SIZE {
            Err(Error::DeviceFailure(
                "Error while reading: read unexpected number of bytes",
            ))
        } else {
            Ok(())
        }
    }

    fn check_response(&mut self) -> Result<bool> {
        let mut buf = [0; MESSAGE_BUFFER_SIZE];
        self.read_interrupt(&mut buf)?;
        Ok(buf[1] == 0x00)
    }
}

impl Drop for RawDevice<'_> {
    fn drop(&mut self) {
        self.handle.release_interface(HID_INTERFACE).unwrap();
        util::exponential_backoff(|| self.handle.attach_kernel_driver(HID_INTERFACE)).unwrap();
    }
}

fn claim_hid_interface(handle: &mut DeviceHandle) -> Result<()> {
    handle.detach_kernel_driver(HID_INTERFACE)?;
    handle.claim_interface(HID_INTERFACE)?;
    Ok(())
}
