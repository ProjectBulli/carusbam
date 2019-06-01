extern crate libusb;

use std::time::Duration;
use libusb::{Context, Device, Direction, Recipient, request_type, RequestType};
use libusb::Error as USBError;
use std::env;
use std::num::ParseIntError;
use std::process::exit;

const GET_PROTOCOL: u8 = 51;
const SEND_STRING: u8 = 52;
const REQUEST_START: u8 = 53;

#[derive(Debug)]
enum Error {
    USB(USBError),
    Parse(ParseIntError),
    Args(Vec<String>),
    UnsupportedVersion(u16)
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let (exit_code, message) = match internal(args) {
        Ok(())
            => (0, ""),
        Err(Error::USB(USBError::Success))
            => ( 100, USBError::Success.strerror()),
        Err(Error::USB(USBError::Io))
            => (-101, USBError::Io.strerror()),
        Err(Error::USB(USBError::InvalidParam))
            => (-102, USBError::InvalidParam.strerror()),
        Err(Error::USB(USBError::Access))
            => (-103, USBError::Access.strerror()),
        Err(Error::USB(USBError::NoDevice))
            => (-104, USBError::NoDevice.strerror()),
        Err(Error::USB(USBError::NotFound))
            => (-105, USBError::NotFound.strerror()),
        Err(Error::USB(USBError::Busy))
            => (-106, USBError::Busy.strerror()),
        Err(Error::USB(USBError::Timeout))
            => (-107, USBError::Timeout.strerror()),
        Err(Error::USB(USBError::Overflow))
            => (-108, USBError::Overflow.strerror()),
        Err(Error::USB(USBError::Pipe))
            => (-109, USBError::Pipe.strerror()),
        Err(Error::USB(USBError::Interrupted))
            => (-110, USBError::Interrupted.strerror()),
        Err(Error::USB(USBError::NoMem))
            => (-111, USBError::NoMem.strerror()),
        Err(Error::USB(USBError::NotSupported))
            => (-112, USBError::NotSupported.strerror()),
        Err(Error::USB(USBError::Other))
            => (-113, USBError::Other.strerror()),
        Err(Error::Parse(_))
            //=> (-200, format!("Parse Error: {}", p).as_str()),
            => (-200, "Parse Error: {}"),
        Err(Error::Args(_args))
            //=> (-300, format!("trouble with arguments, need two arguments: bus-number and device-number but got '{:?}'", args).as_str()),
            => (-300, "trouble with arguments, need two arguments: bus-number and device-number"),
        Err(Error::UnsupportedVersion(_version))
            //=> (-400, format!("Unsupported android auto version {} found", version).as_str()),
            => (-400, "Unsupported android auto version found"),
    };
    println!("{}", message);
    exit(exit_code);
}

fn internal(args: Vec<String>) -> Result<(), Error> {
    if args.len() <= 2 {
        return Err(Error::Args(args))
    }
    let bus_number = &args[1].parse::<u8>().map_err(Error::Parse)?;
    let device_number = &args[2].parse::<u8>().map_err(Error::Parse)?;
    let context = Context::new().map_err(Error::USB)?;
    for device in context.devices().map_err(Error::USB)?.iter() {
        let bus = device.bus_number();
        let address = device.address();
        if bus == *bus_number && *device_number == address {
            match probe_device(device) {
                Ok(version) => {
                    println!("   ok {}:{} version {}", bus, address, version)
                }
                Err(e) => {
                    println!("error {}:{} {:?}", bus, address, e)
                }
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
