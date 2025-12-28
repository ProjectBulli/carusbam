use std::env;
use std::num::ParseIntError;
use std::process::exit;
use std::time::Duration;

use rusb::{Device, DeviceList, Direction, GlobalContext, Recipient, request_type, RequestType};

const GET_PROTOCOL: u8 = 51;
const SEND_STRING: u8 = 52;
const REQUEST_START: u8 = 53;
const TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Debug)]
enum Error {
    USB(rusb::Error),
    Parse(ParseIntError),
    Args(Vec<String>),
    UnsupportedVersion(u16),
}

trait ErrorCode {
    fn error_code(&self) -> i32;
}

impl ErrorCode for rusb::Error {
    fn error_code(&self) -> i32 {
       match self {
           rusb::Error::Io            => -101,
           rusb::Error::InvalidParam  => -102,
           rusb::Error::Access        => -103,
           rusb::Error::NoDevice      => -104,
           rusb::Error::NotFound      => -105,
           rusb::Error::Busy          => -106,
           rusb::Error::Timeout       => -107,
           rusb::Error::Overflow      => -108,
           rusb::Error::Pipe          => -109,
           rusb::Error::Interrupted   => -110,
           rusb::Error::NoMem         => -111,
           rusb::Error::NotSupported  => -112,
           rusb::Error::Other         => -113,
           rusb::Error::BadDescriptor => -114,
       }
    }
}

impl ErrorCode for ParseIntError {
    fn error_code(&self) -> i32 {
        -200
    }
}

impl ErrorCode for Error {
    fn error_code(&self) -> i32 {
        match self {
            Error::USB(e) => e.error_code(),
            Error::Parse(e) =>e.error_code(),
            Error::Args(_) => -300,
            Error::UnsupportedVersion(_) => -400,
        }
    }
}

impl ErrorCode for Result<(), Error> {
    fn error_code(&self) -> i32 {
        match self {
            Ok(()) => 0,
            Err(e) => e.error_code(),
        }
    }
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let result = internal(args);
    let exit_code = result.error_code();
    match result {
        Ok(())
            => {},
        Err(Error::Parse(e))
            => println!("Parse Error: {}", e),
        Err(Error::Args(args))
            => println!("trouble with arguments, need two arguments: bus-number and device-number {}", args.len()),
        Err(Error::UnsupportedVersion(version))
            => println!("Unsupported android auto version found '{}'", version),
        Err(Error::USB(e))
            => println!("USB Error {}", e)
    };
    exit(exit_code);
}

fn internal(args: Vec<String>) -> Result<(), Error> {
    if args.len() <= 2 {
        return Err(Error::Args(args));
    }
    let bus_number = &args[1].parse::<u8>().map_err(Error::Parse)?;
    let device_number = &args[2].parse::<u8>().map_err(Error::Parse)?;
    for device in DeviceList::new().map_err(Error::USB)?.iter() {
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

fn probe_device(device: Device<GlobalContext>) -> Result<u16, Error> {
    let handle = device.open().map_err(Error::USB)?;
    let mut buffer: [u8; 2] = [0, 2];
    let read_request = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
    let write_request = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);

    handle.read_control(read_request, GET_PROTOCOL, 0, 0, &mut buffer, TIMEOUT).map_err(Error::USB)?;
    let version = buffer.as_version();
    if version < 1 {
        return Err(Error::UnsupportedVersion(version));
    }
    handle.write_control(write_request, SEND_STRING, 0, 0, "Android".as_ref(), TIMEOUT).map_err(Error::USB)?;
    handle.write_control(write_request, SEND_STRING, 0, 1, "Android Auto".as_ref(), TIMEOUT).map_err(Error::USB)?;
    handle.write_control(write_request, REQUEST_START, 0, 0, &mut [], TIMEOUT).map_err(Error::USB)?;
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
