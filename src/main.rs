extern crate libusb;

use std::time::Duration;
use libusb::{Context, Device, Direction, Recipient, request_type, RequestType};
use std::env;
use std::num::ParseIntError;

const GET_PROTOCOL: u8 = 51;
const SEND_STRING: u8 = 52;
const REQUEST_START: u8 = 53;

#[derive(Debug)]
enum Error {
    USB(libusb::Error),
    Parse(ParseIntError),
    Args(Vec<String>),
    UnsupportedVersion(u16)
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        return Err(Error::Args(args))
    }
    let bus_number = &args[1].parse::<u8>().map_err(Error::Parse)?;
    let device_number = &args[2].parse::<u8>().map_err( Error::Parse)?;
    let context = Context::new().map_err( Error::USB)?;
    for device in context.devices().map_err(Error::USB)?.iter() {
        let bus = device.bus_number();
        let address = device.address();
        match probe_device(device) {
            Ok(version) => {
                println!("   ok {}:{} version {}", bus, address, version)
            }
            Err(e) => {
                println!("error {}:{} {:?}", bus, address, e)
            }
        }
    }
    Ok(())
}

fn probe_device(device: Device) -> Result<u16, Error> {
    let handle = device.open().map_err( Error::USB)?;
    let mut buffer: [u8; 2] = [0, 2];
    let read_request = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
    let write_request = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
    let timeout = Duration::from_secs(1);

    handle.read_control(read_request, GET_PROTOCOL, 0, 0, &mut buffer, timeout).map_err(Error::USB)?;
    let version = buffer.as_version();
    if version < 1 {
        return Err(Error::UnsupportedVersion(version));
    }
    handle.write_control(write_request, SEND_STRING, 0, 0, "Android".as_ref(), timeout).map_err(Error::USB)?;
    handle.write_control(write_request, SEND_STRING, 0, 1, "Android Auto".as_ref(), timeout).map_err(Error::USB)?;
    handle.write_control(write_request, REQUEST_START, 0, 0, &mut [], timeout).map_err(Error::USB)?;
    Ok(version)
}

trait VersionPackage {
    fn as_version(&self) -> u16;
}

impl VersionPackage for [u8; 2] {
    fn as_version(&self) -> u16 {
        u16::from(self[1]) << 8 | u16::from(self[0])
    }
}
