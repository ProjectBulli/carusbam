extern crate libusb;

use std::time::Duration;
use libusb::{Context, Device, Direction, Error, Recipient, request_type, RequestType};

const GET_PROTOCOL: u8 = 51;
const SEND_STRING: u8 = 52;
const REQUEST_START: u8 = 53;

fn main() -> Result<(), Error> {
    let context = Context::new()?;
    for device in context.devices()?.iter() {
        let bus = device.bus_number();
        let address = device.address();
        match probe_device(device) {
            Ok(version) => {
                println!("   ok {}:{} version {}", bus, address, version)
            }
            Err(e) => {
                println!("error {}:{} {}", bus, address, e)
            }
        }
    }
    Ok(())
}

fn probe_device(device: Device) -> Result<u16, Error> {
    let handle = device.open()?;
    let mut buffer: [u8; 2] = [0, 2];
    let read_request = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
    let write_request = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
    let timeout = Duration::from_secs(1);

    handle.read_control(read_request, GET_PROTOCOL, 0, 0, &mut buffer, timeout)?;
    let version = buffer.as_version();
    if version < 1 {
        return Result::Err(Error::NotSupported);
    }
    handle.write_control(write_request, SEND_STRING, 0, 0, "Android".as_ref(), timeout)?;
    handle.write_control(write_request, SEND_STRING, 0, 1, "Android Auto".as_ref(), timeout)?;
    handle.write_control(write_request, REQUEST_START, 0, 0, &mut [], timeout)?;
    Ok(version)
}

trait VersionPackage {
    fn as_version(&self) -> u16;
}

impl VersionPackage for [u8; 2] {
    fn as_version(&self) -> u16 {
        (self[1] as u16) << 8 | (self[0] as u16)
    }
}
